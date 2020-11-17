use crate::events::{Clock, GameEvent, GameFull, GameState, ChatLine};
use crate::helper::*;
use myopic_board::{parse, Move, MutBoard};
use myopic_core::Side;
use reqwest::blocking::Client;
use std::time::Duration;

const MOVE_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameMetadata {
    lambda_side: Side,
    clock: Clock,
}

#[derive(Debug)]
pub struct Game {
    id: String,
    lambda_player_id: String,
    expected_half_moves: u32,
    metadata: Vec<GameMetadata>,
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
        auth_token: &str,
    ) -> Game {
        Game {
            id,
            lambda_player_id,
            expected_half_moves,
            metadata: Vec::new(),
            auth_token: auth_token.to_owned(),
            client: reqwest::blocking::Client::new(),
            is_finished: false,
        }
    }

    pub fn process_event(&mut self, event_json: &str) -> Result<GameExecutionState, String> {
        match serde_json::from_str(event_json) {
            Err(error) => Err(format!("{}", error)),
            Ok(event) => match event {
                GameEvent::GameFull { content } => self.process_game_full(content),
                GameEvent::State { content } => self.process_game_state(content),
                GameEvent::ChatLine { content } => self.process_chat_line(content),
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
        self.metadata.push(GameMetadata {
            clock: game_full.clock,
            lambda_side: if self.lambda_player_id == game_full.white.id {
                Side::White
            } else if self.lambda_player_id == game_full.black.id {
                Side::Black
            } else {
                return Err(format!("Unrecognized names"));
            },
        });
        self.process_game_state(game_full.state)
    }

    fn process_game_state(&mut self, state: GameState) -> Result<GameExecutionState, String> {
        let metadata = self.get_latest_metadata()?;
        let moves = parse::uci(&state.moves)?;
        let mut board = myopic_brain::eval::start();
        moves.iter().for_each(|mv| {
            board.evolve(mv);
        });

        match board.termination_status() {
            Some(_) => {
                self.is_finished = true;
                Ok(GameExecutionState::Finished)
            },
            None => {
                self.is_finished = false;
                if board.active() != metadata.lambda_side {
                    Ok(GameExecutionState::Running)
                } else {
                    let thinking_time = compute_thinking_time(ThinkingTimeParams {
                        expected_half_move_count: self.expected_half_moves,
                        half_moves_played: moves.len() as u32,
                        initial: Duration::from_millis(metadata.clock.initial),
                        increment: Duration::from_millis(metadata.clock.increment),
                    });
                    let result = myopic_brain::search(board, thinking_time)?;
                    self.post_move(result.best_move)
                }
            }
        }
    }

    fn get_latest_metadata(&self) -> Result<GameMetadata, String> {
        self.metadata.get(0).cloned().ok_or(format!("Metadata not initialized"))
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
