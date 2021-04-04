use crate::events::{ChatLine, Clock, GameEvent, GameFull, GameState};
use crate::lichess::{LichessService, LichessChatRoom};
use crate::messages;
use crate::timing::Timing;
use crate::TimeConstraints;
use myopic_brain::{EvalBoardImpl, MutBoard, MutBoardImpl, Side};
use reqwest::StatusCode;
use std::error::Error;
use std::ops::Add;
use std::time::{Duration, Instant};

const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";
const MOVE_LATENCY_MS: u64 = 200;
const MIN_COMPUTE_TIME_MS: u64 = 200;

pub trait LookupService {
    fn lookup_move(
        &mut self,
        initial_position: &InitalPosition,
        uci_sequence: &str,
    ) -> Result<Option<String>, String>;
}

pub trait ComputeService {
    fn compute_move(
        &self,
        initial_position: &InitalPosition,
        uci_sequence: &str,
        time_limit: Duration,
    ) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InitalPosition {
    Start,
    CustomFen(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
    initial_position: InitalPosition,
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub game_id: String,
    pub bot_id: String,
    pub expected_half_moves: u32,
    pub time_constraints: TimeConstraints,
    pub lichess_auth_token: String,
}

#[derive(Debug)]
pub struct Game<O, C, E>
where
    O: LookupService,
    C: ComputeService,
    E: LookupService,
{
    bot_id: String,
    expected_half_moves: u32,
    time_constraints: TimeConstraints,
    inferred_metadata: Option<InferredGameMetadata>,
    lichess_service: LichessService,
    opening_service: O,
    compute_service: C,
    endgame_service: E,
    halfmove_count: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Recurse,
}

impl<O, C, E> Game<O, C, E>
where
    O: LookupService,
    C: ComputeService,
    E: LookupService,
{
    pub fn new(config: GameConfig, openings: O, compute: C, endgame: E) -> Game<O, C, E> {
        Game {
            lichess_service: LichessService::new(config.lichess_auth_token, config.game_id),
            opening_service: openings,
            endgame_service: endgame,
            compute_service: compute,
            bot_id: config.bot_id,
            expected_half_moves: config.expected_half_moves,
            time_constraints: config.time_constraints,
            inferred_metadata: None,
            halfmove_count: 0,
        }
    }

    pub fn time_constraints(&self) -> &TimeConstraints {
        &self.time_constraints
    }

    pub fn halfmove_count(&self) -> usize {
        self.halfmove_count
    }

    pub fn abort(&self) -> Result<StatusCode, String> {
        self.lichess_service.abort()
    }

    pub fn post_introduction(&self) {
        for chatline in vec![
            messages::INTRO_1,
            messages::INTRO_2,
            messages::INTRO_3,
            messages::INTRO_4,
        ] {
            self.post_chatline(chatline, LichessChatRoom::Player);
            self.post_chatline(chatline, LichessChatRoom::Spectator);
        }
    }

    fn post_chatline(&self, text: &str, room: LichessChatRoom) {
        match self.lichess_service.post_chatline(text, room) {
            Err(err) => {
                log::warn!("Failed to post chatline {} in {:?}: {}", text, room, err)
            },
            Ok(status) => {
                log::info!("Response status {} for chatline {} in room {:?}", status, text, room)
            },
        }
    }

    pub fn process_event(&mut self, event_json: &str) -> Result<GameExecutionState, String> {
        match serde_json::from_str(event_json) {
            Err(error) => {
                log::warn!("Error parsing event {}", error);
                Err(format!("{}", error))
            }
            Ok(event) => match event {
                GameEvent::GameFull { content } => self.process_game_full(content),
                GameEvent::State { content } => self.process_game_state(content),
                GameEvent::ChatLine { content } => self.process_chat_line(content),
            },
        }
    }

    fn process_chat_line(&self, _chat_line: ChatLine) -> Result<GameExecutionState, String> {
        // Do nothing for now
        Ok(GameExecutionState::Running)
    }

    fn process_game_full(&mut self, game_full: GameFull) -> Result<GameExecutionState, String> {
        // Track info required for playing future gamestates
        self.inferred_metadata = Some(InferredGameMetadata {
            clock: game_full.clock,
            lambda_side: if self.bot_id == game_full.white.id {
                log::info!("Detected lambda is playing as white");
                Side::White
            } else if self.bot_id == game_full.black.id {
                log::info!("Detected lambda is playing as black");
                Side::Black
            } else {
                return Err(format!("Unrecognized names"));
            },
            initial_position: if game_full.initial_fen.as_str() == "startpos" {
                InitalPosition::Start
            } else {
                InitalPosition::CustomFen(game_full.initial_fen)
            },
        });
        self.process_game_state(game_full.state)
    }

    fn get_game_state(&self, moves: &str) -> Result<(EvalBoardImpl<MutBoardImpl>, u32), String> {
        let mut state = match &self.get_latest_metadata()?.initial_position {
            InitalPosition::Start => myopic_brain::pos::start(),
            InitalPosition::CustomFen(fen) => myopic_brain::pos::from_fen(fen.as_str())?,
        };
        let moves = myopic_brain::parse::partial_uci(&state, moves)?;
        moves.iter().for_each(|mv| {
            state.evolve(mv);
        });
        Ok((state, moves.len() as u32))
    }

    fn process_game_state(&mut self, state: GameState) -> Result<GameExecutionState, String> {
        log::info!("Parsing previous game moves: {}", state.moves);
        let (board, n_moves) = self.get_game_state(state.moves.as_str())?;
        self.halfmove_count = n_moves as usize;
        match state.status.as_str() {
            STARTED_STATUS | CREATED_STATUS => {
                let metadata = self.get_latest_metadata()?.clone();
                if board.active() != metadata.lambda_side {
                    log::info!("It is not our turn, waiting for opponents move");
                    Ok(GameExecutionState::Running)
                } else {
                    match self.get_opening_move(&metadata.initial_position, &state.moves) {
                        Some(mv) => self.lichess_service.post_move(mv),
                        None => {
                            match self.get_endgame_move(&metadata.initial_position, &state.moves) {
                                Some(mv) => self.lichess_service.post_move(mv),
                                None => self.compute_and_post_move(
                                    &state.moves,
                                    self.compute_thinking_time(n_moves, &state)?,
                                ),
                            }
                        }
                    }
                }
            }
            // All other possibilities indicate the game is over
            status => {
                log::info!(
                    "Game has finished with status: {}! Terminating execution",
                    status
                );
                Ok(GameExecutionState::Finished)
            }
        }
    }

    fn get_endgame_move(
        &mut self,
        initial_position: &InitalPosition,
        current_sequence: &str,
    ) -> Option<String> {
        match self
            .endgame_service
            .lookup_move(initial_position, current_sequence)
        {
            Ok(mv) => mv,
            Err(message) => {
                log::info!("Error in the endgame service: {}", message);
                None
            }
        }
    }

    fn get_opening_move(
        &mut self,
        initial_position: &InitalPosition,
        current_sequence: &str,
    ) -> Option<String> {
        match self
            .opening_service
            .lookup_move(initial_position, current_sequence)
        {
            Ok(mv) => mv,
            Err(message) => {
                log::info!("Error in the opening service: {}", message);
                None
            }
        }
    }

    fn compute_and_post_move(
        &self,
        moves: &String,
        time: Duration,
    ) -> Result<GameExecutionState, String> {
        let lambda_end_instant = self.time_constraints.lambda_end_instant();
        if Instant::now().add(time) >= lambda_end_instant {
            Ok(GameExecutionState::Recurse)
        } else {
            let metadata = self.get_latest_metadata()?;
            self.compute_service
                .compute_move(&metadata.initial_position, moves.as_str(), time)
                .map_err(|e| format!("{}", e))
                .and_then(|mv| self.lichess_service.post_move(mv))
        }
    }

    fn compute_thinking_time(
        &self,
        moves_played: u32,
        state: &GameState,
    ) -> Result<Duration, String> {
        let metadata = self.get_latest_metadata()?;
        let (remaining, inc) = match metadata.lambda_side {
            Side::White => (
                Duration::from_millis(state.wtime),
                Duration::from_millis(state.winc),
            ),
            Side::Black => (
                Duration::from_millis(state.btime),
                Duration::from_millis(state.binc),
            ),
        };
        Ok(Timing::new(
            inc,
            Duration::from_millis(MOVE_LATENCY_MS),
            Duration::from_millis(MIN_COMPUTE_TIME_MS),
        )
        .compute_thinking_time(moves_played as usize, remaining))
    }

    fn get_latest_metadata(&self) -> Result<&InferredGameMetadata, String> {
        self.inferred_metadata
            .as_ref()
            .ok_or(format!("Metadata not initialized"))
    }
}
