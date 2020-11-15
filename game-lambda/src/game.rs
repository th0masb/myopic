use myopic_core::Side;
use crate::events::{Player, Clock, GameFull, GameState};
use reqwest::blocking::Client;
use crate::helper::*;
use myopic_board::{MutBoard, Move, parse};
use std::time::Duration;

const MOVE_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameMetadata {
    lambda_side: Side,
    clock: Clock,
}

pub struct Game {
    id: String,
    lambda_player_id: String,
    expected_half_moves: u32,
    metadata: Vec<GameMetadata>,
    client: Client
}

impl Game {
    pub fn new(id: String, lambda_player_id: String, expected_half_moves: u32) -> Game {
        Game {
            id,
            lambda_player_id,
            expected_half_moves,
            metadata: Vec::new(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn process_game_full(&mut self, game_full: GameFull) -> Result<(), String> {
        // Track info required for playing future gamestates
        self.metadata.push(GameMetadata {
            clock: game_full.clock,
            lambda_side: if self.lambda_player_id == game_full.white.id {
                Side::White
            } else if self.lambda_player_id == game_full.black.id {
                Side::Black
            } else {
                return Err(format!("Unrecognized names"))
            }
        });
        self.process_game_state(game_full.state)
    }

    pub fn process_game_state(&self, state: GameState) -> Result<(), String> {
        let metadata = self.get_latest_metadata()?;
        let moves = parse::uci(&state.moves)?;
        let mut board = myopic_brain::eval::start();
        moves.iter().for_each(|mv| { board.evolve(mv); });

        if board.active() == metadata.lambda_side {
            let thinking_time = compute_thinking_time(ThinkingTimeParams {
                expected_half_move_count: self.expected_half_moves,
                half_moves_played: moves.len() as u32,
                initial: Duration::from_millis(metadata.clock.initial),
                increment: Duration::from_millis(metadata.clock.increment),
            });
            let result = myopic_brain::search(board, thinking_time)?;
            self.post_move(result.best_move)
        } else {
            Ok(())
        }
    }

    fn get_latest_metadata(&self) -> Result<GameMetadata, String> {
        self.metadata.get(0).cloned().ok_or(format!("Metadata not initialized"))
    }

    fn post_move(&self, mv: Move) -> Result<(), String> {
        let uci_result = move_to_uci(&mv);
        // Add timeout and retry logic
        match self.client
            .post(format!("{}/{}/move/{}", MOVE_ENDPOINT, self.id, uci_result).as_str())
            .bearer_auth("xxx")
            .send() {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Error posting move: {}", error))
        }
    }
}
