use crate::events::{ChatLine, Clock, GameEvent, GameFull, GameState};
use crate::helper::*;
use crate::TimeConstraints;
use myopic_brain::{EvalBoard, MutBoard, Side};

use crate::dynamodb_openings::{DynamoDbOpeningService, DynamoDbOpeningServiceConfig};
use crate::lichess::LichessService;
use std::ops::Add;
use std::time::{Duration, Instant};

const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";
const MAX_TABLE_MISSES: usize = 2;

pub trait OpeningService {
    fn get_move(&self, uci_sequence: &str) -> Result<Option<String>, String>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
}

#[derive(Debug, Clone)]
pub struct DynamoDbGameConfig {
    pub game_id: String,
    pub bot_id: String,
    pub expected_half_moves: u32,
    pub time_constraints: TimeConstraints,
    pub lichess_auth_token: String,
    pub dynamodb_openings_config: DynamoDbOpeningServiceConfig,
}

#[derive(Debug)]
pub struct Game<O: OpeningService> {
    bot_id: String,
    expected_half_moves: u32,
    time_constraints: TimeConstraints,
    inferred_metadata: Option<InferredGameMetadata>,
    lichess_service: LichessService,
    opening_service: O,
    opening_misses: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Recurse,
}

pub fn new_dynamodb(props: DynamoDbGameConfig) -> Game<DynamoDbOpeningService> {
    Game {
        lichess_service: LichessService::new(props.lichess_auth_token, props.game_id),
        opening_service: DynamoDbOpeningService::new(props.dynamodb_openings_config),
        bot_id: props.bot_id,
        expected_half_moves: props.expected_half_moves,
        inferred_metadata: None,
        time_constraints: props.time_constraints,
        opening_misses: 0,
    }
}

impl<O: OpeningService> Game<O> {
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
                GameEvent::GameFull { content } => {
                    log::info!("Parsed full game information");
                    self.process_game_full(content)
                }
                GameEvent::State { content } => {
                    log::info!("Parsed individual game state");
                    self.process_game_state(content)
                }
                GameEvent::ChatLine { content } => {
                    log::info!("Parsed chat line");
                    self.process_chat_line(content)
                }
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
                        None => self.compute_move(board, self.compute_thinking_time(n_moves)?),
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

    fn compute_move<B: EvalBoard>(
        &self,
        board: B,
        time: Duration,
    ) -> Result<GameExecutionState, String> {
        let lambda_end_instant = self.time_constraints.lambda_end_instant();
        if Instant::now().add(time) >= lambda_end_instant {
            Ok(GameExecutionState::Recurse)
        } else {
            log::info!("Spending {}ms thinking", time.as_millis());
            let result = myopic_brain::search(board, time)?;
            log::info!(
                "Search output: {}",
                serde_json::to_string(&result).expect("Unable to serialise search result")
            );
            self.lichess_service
                .post_move(move_to_uci(&result.best_move))
        }
    }

    fn get_opening_move(&mut self, current_sequence: &str) -> Option<String> {
        if self.opening_misses >= MAX_TABLE_MISSES {
            log::info!("Skipping opening table check as {} checks were missed", MAX_TABLE_MISSES);
            None
        } else {
            match self.opening_service.get_move(current_sequence) {
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
