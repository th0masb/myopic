use crate::events::{ChatLine, Clock, GameEvent, GameFull, GameState};
use crate::helper::*;
use crate::TimeConstraints;
use myopic_board::{parse, Move, MutBoard, MutBoardImpl};
use myopic_brain::EvalBoardImpl;
use myopic_core::Side;
use reqwest::blocking::Client;
use std::ops::Add;
use std::time::{Duration, Instant};

const MOVE_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
}

#[derive(Debug, Clone)]
pub struct GameProps {
    pub game_id: String,
    pub lambda_player_id: String,
    pub expected_half_moves: u32,
    pub time_constraints: TimeConstraints,
    pub auth_token: String,
}

#[derive(Debug)]
pub struct Game {
    props: GameProps,
    inferred_metadata: Option<InferredGameMetadata>,
    client: Client,
    is_finished: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Recurse,
}

impl Game {
    pub fn new(props: GameProps) -> Game {
        Game {
            props,
            inferred_metadata: None,
            client: reqwest::blocking::Client::new(),
            is_finished: false,
        }
    }

    pub fn time_constraints(&self) -> &TimeConstraints {
        &self.props.time_constraints
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
        if self.is_finished {
            Ok(GameExecutionState::Finished)
        } else {
            Ok(GameExecutionState::Running)
        }
    }

    fn process_game_full(&mut self, game_full: GameFull) -> Result<GameExecutionState, String> {
        // Track info required for playing future gamestates
        self.inferred_metadata = Some(InferredGameMetadata {
            clock: game_full.clock,
            lambda_side: if self.props.lambda_player_id == game_full.white.id {
                log::info!("Detected lambda is playing as white");
                Side::White
            } else if self.props.lambda_player_id == game_full.black.id {
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
                self.is_finished = false;
                if board.active() != self.get_latest_metadata()?.lambda_side {
                    log::info!("It is not our turn, waiting for opponents move");
                    Ok(GameExecutionState::Running)
                } else {
                    let thinking_time = self.compute_thinking_time(n_moves)?;
                    let lambda_end_instant = self.props.time_constraints.lambda_end_instant();
                    if Instant::now().add(thinking_time) >= lambda_end_instant {
                        Ok(GameExecutionState::Recurse)
                    } else {
                        log::info!(
                            "Computed we should spend {}s thinking",
                            thinking_time.as_secs()
                        );
                        let result = myopic_brain::search(board, thinking_time)?;
                        log::info!("Completed search: {:?}", result);
                        self.post_move(result.best_move)
                    }
                }
            }
            // All other possibilities indicate the game is over
            status => {
                log::info!("Game has finished with status: {}! Terminating execution", status);
                self.is_finished = true;
                Ok(GameExecutionState::Finished)
            }
        }
    }

    fn compute_thinking_time(&self, moves_played: u32) -> Result<Duration, String> {
        let metadata = self.get_latest_metadata()?;
        Ok(compute_thinking_time(ThinkingTimeParams {
            expected_half_move_count: self.props.expected_half_moves,
            half_moves_played: moves_played,
            initial: Duration::from_millis(metadata.clock.initial),
            increment: Duration::from_millis(metadata.clock.increment),
        }))
    }

    fn get_latest_metadata(&self) -> Result<&InferredGameMetadata, String> {
        self.inferred_metadata.as_ref().ok_or(format!("Metadata not initialized"))
    }

    fn post_move(&self, mv: Move) -> Result<GameExecutionState, String> {
        // Add timeout and retry logic
        self.client
            .post(
                format!("{}/{}/move/{}", MOVE_ENDPOINT, self.props.game_id, move_to_uci(&mv))
                    .as_str(),
            )
            .bearer_auth(&self.props.auth_token)
            .send()
            .map(|_| GameExecutionState::Running)
            .map_err(|error| format!("Error posting move: {}", error))
    }
}

fn get_game_state(moves: &String) -> Result<(EvalBoardImpl<MutBoardImpl>, u32), String> {
    let moves = parse::uci(moves)?;
    let mut board = myopic_brain::eval::start();
    moves.iter().for_each(|mv| {
        board.evolve(mv);
    });
    Ok((board, moves.len() as u32))
}
