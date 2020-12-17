use crate::events::{ChatLine, Clock, GameEvent, GameFull, GameState};
use crate::helper::*;
use crate::lichess::LichessService;
use crate::TimeConstraints;
use myopic_brain::{MutBoard, Side};
use std::error::Error;
use std::ops::Add;
use std::time::{Duration, Instant};

const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";
const MAX_TABLE_MISSES: usize = 2;

pub trait LookupService {
    fn lookup_move(&self, uci_sequence: &str) -> Result<Option<String>, String>;
}

pub trait ComputeService {
    fn compute_move(
        &self,
        uci_sequence: &str,
        time_limit: Duration,
    ) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
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
pub struct Game<O, C>
where
    O: LookupService,
    C: ComputeService,
{
    bot_id: String,
    expected_half_moves: u32,
    time_constraints: TimeConstraints,
    inferred_metadata: Option<InferredGameMetadata>,
    lichess_service: LichessService,
    opening_service: O,
    compute_service: C,
    opening_misses: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Recurse,
}

impl<O, C> Game<O, C>
where
    O: LookupService,
    C: ComputeService,
{
    pub fn new(config: GameConfig, openings: O, compute: C) -> Game<O, C> {
        Game {
            lichess_service: LichessService::new(config.lichess_auth_token, config.game_id),
            opening_service: openings,
            opening_misses: 0,
            compute_service: compute,
            bot_id: config.bot_id,
            expected_half_moves: config.expected_half_moves,
            time_constraints: config.time_constraints,
            inferred_metadata: None,
        }
    }

    pub fn time_constraints(&self) -> &TimeConstraints {
        &self.time_constraints
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
        });
        self.process_game_state(game_full.state)
    }

    fn process_game_state(&mut self, state: GameState) -> Result<GameExecutionState, String> {
        log::info!("Parsing previous game moves: {}", state.moves);
        let (board, n_moves) = get_game_state(&state.moves)?;
        match state.status.as_str() {
            STARTED_STATUS | CREATED_STATUS => {
                if board.active() != self.get_latest_metadata()?.lambda_side {
                    log::info!("It is not our turn, waiting for opponents move");
                    Ok(GameExecutionState::Running)
                } else {
                    match self.get_opening_move(&state.moves) {
                        Some(mv) => self.lichess_service.post_move(mv),
                        None => {
                            let thinking_time = self.compute_thinking_time(n_moves)?;
                            self.compute_move(&state.moves, thinking_time)
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

    fn compute_move(&self, moves: &String, time: Duration) -> Result<GameExecutionState, String> {
        let lambda_end_instant = self.time_constraints.lambda_end_instant();
        if Instant::now().add(time) >= lambda_end_instant {
            Ok(GameExecutionState::Recurse)
        } else {
            self.compute_service
                .compute_move(moves.as_str(), time)
                .map_err(|e| format!("{}", e))
                .and_then(|mv| self.lichess_service.post_move(mv))
        }
    }

    fn get_opening_move(&mut self, current_sequence: &str) -> Option<String> {
        if self.opening_misses >= MAX_TABLE_MISSES {
            log::info!(
                "Skipping opening table check as {} checks were missed",
                MAX_TABLE_MISSES
            );
            None
        } else {
            match self.opening_service.lookup_move(current_sequence) {
                Ok(result) => {
                    if result.is_none() {
                        self.opening_misses += 1;
                    }
                    result
                }
                Err(error) => {
                    log::info!("Error retrieving opening move: {}", error);
                    self.opening_misses += 1;
                    None
                }
            }
        }
    }

    fn compute_thinking_time(&self, moves_played: u32) -> Result<Duration, String> {
        let metadata = self.get_latest_metadata()?;
        Ok(compute_thinking_time(ThinkingTimeParams {
            expected_half_move_count: self.expected_half_moves,
            half_moves_played: moves_played,
            initial: Duration::from_millis(metadata.clock.initial),
            increment: Duration::from_millis(metadata.clock.increment),
        }))
    }

    fn get_latest_metadata(&self) -> Result<&InferredGameMetadata, String> {
        self.inferred_metadata
            .as_ref()
            .ok_or(format!("Metadata not initialized"))
    }
}
