use crate::events::{Clock, GameEvent, GameFull, GameState, ChatLine};
use crate::helper::*;
use myopic_board::{parse, Move, MutBoard, MutBoardImpl};
use myopic_core::Side;
use reqwest::blocking::Client;
use std::time::Duration;
use myopic_brain::EvalBoardImpl;

const MOVE_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
}

#[derive(Debug)]
pub struct Game {
    id: String,
    lambda_player_id: String,
    expected_half_moves: u32,
    inferred_metadata: Option<InferredGameMetadata>,
    auth_token: String,
    client: Client,
    is_finished: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
}

impl Game {
    pub fn new(
        id: String,
        lambda_player_id: String,
        expected_half_moves: u32,
        auth_token: String,
    ) -> Game {
        Game {
            id,
            lambda_player_id,
            expected_half_moves,
            inferred_metadata: None,
            auth_token,
            client: reqwest::blocking::Client::new(),
            is_finished: false,
        }
    }

    pub fn process_event(&mut self, event_json: &str) -> Result<GameExecutionState, String> {
        match serde_json::from_str(event_json) {
            Err(error) => {
                log::warn!("Error parsing event {}", error);
                Err(format!("{}", error))
            },
            Ok(event) => match event {
                GameEvent::GameFull { content } => {
                    log::info!("Parsed full game information");
                    self.process_game_full(content)
                },
                GameEvent::State { content } => {
                    log::info!("Parsed individual game state");
                    self.process_game_state(content)
                },
                GameEvent::ChatLine { content } => {
                    log::info!("Parsed chat line");
                    self.process_chat_line(content)
                },
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
            lambda_side: if self.lambda_player_id == game_full.white.id {
                log::info!("Detected lambda is playing as white");
                Side::White
            } else if self.lambda_player_id == game_full.black.id {
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
            STARTED_STATUS | CREATED_STATUS =>  {
                self.is_finished = false;
                if board.active() != self.get_latest_metadata()?.lambda_side {
                    log::info!("It is not our turn, waiting for opponents move");
                    Ok(GameExecutionState::Running)
                } else {
                    let thinking_time = self.compute_thinking_time(n_moves)?;
                    log::info!("Computed we should spend {}s thinking", thinking_time.as_secs());
                    let result = myopic_brain::search(board, thinking_time)?;
                    log::info!("Completed search: {:?}", result);
                    self.post_move(result.best_move)
                }
            },
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
            expected_half_move_count: self.expected_half_moves,
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
            .post(format!("{}/{}/move/{}", MOVE_ENDPOINT, self.id, move_to_uci(&mv)).as_str())
            .bearer_auth(&self.auth_token)
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
