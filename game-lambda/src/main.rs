mod events;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use reqwest::blocking::{Client, Request, Response};
use std::io::{Read, BufReader, BufRead};
use events::{GameEvent, GameState, Player, GameFull};
use myopic_core::Side;
use myopic_board::{parse, MutBoard, Move};
use myopic_board::start_position;
use myopic_brain;
use std::time::Duration;
use crate::events::Clock;
use myopic_core::pieces::Piece;

const EXPECTED_MOVE_COUNT: u64 = 30;
const LAMBDA_ID: &'static str = "myopic-bot";

fn move_to_uci(mv: &Move) -> String {
    match mv {
        &Move::Standard(_, src, dest) => format!("{}{}", src, dest),
        &Move::Enpassant(src, dest) => format!("{}{}", src, dest),
        &Move::Promotion(src, dest, piece) => format!("{}{}{}", src, dest, match piece {
            Piece::WQ | Piece::BQ => "q",
            Piece::WR | Piece::BR => "r",
            Piece::WB | Piece::BB => "b",
            Piece::WN | Piece::BN => "n",
            _ => ""
        }),
        &Move::Castle(zone) => {
            let (_, src, dest) = zone.king_data();
            format!("{}{}", src, dest)
        }
    }.to_lowercase().to_owned()
}

#[cfg(test)]
mod uci_conversion_test {
    use super::move_to_uci;
    use myopic_board::Move;
    use myopic_core::{pieces::Piece, Square};

    #[test]
    fn test_standard_conversion() {
        assert_eq!("e2e4", move_to_uci(&Move::Standard(Piece::WP, Square::E2, Square::E4)).as_str());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameMetadata {
    lambda_side: Side,
    lambda_player: Player,
    clock: Clock,
}

struct Game {
    metadata: Vec<GameMetadata>,
    client: Client
}

impl Game {
    fn new() -> Game {
        Game {
            metadata: Vec::new(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_latest_metadata(&self) -> Result<GameMetadata, String> {
        self.metadata.get(0).cloned().ok_or(format!("Metadata not initialized"))
    }

    fn process_game_full(&mut self, game_full: GameFull) -> Result<(), String> {
        // Track info required for playing future gamestates
        self.metadata.push(match (game_full.white.id.as_str(), game_full.black.id.as_str()) {
            (LAMBDA_ID, _) => GameMetadata {
                lambda_side: Side::White,
                lambda_player: game_full.white,
                clock: game_full.clock
            },
            (_, LAMBDA_ID) => GameMetadata {
                lambda_side: Side::Black,
                lambda_player: game_full.black,
                clock: game_full.clock
            },
            _ => return Err(format!("Unrecognized names")),
        });
        self.process_game_state(game_full.state)
    }

    fn process_game_state(&self, state: GameState) -> Result<(), String> {
        let metadata = self.get_latest_metadata()?;
        let moves = parse::uci(&state.moves)?;
        let mut board = myopic_brain::eval::start();
        moves.iter().for_each(|mv| { board.evolve(mv); });

        if board.active() == metadata.lambda_side {
            let thinking_time = Game::compute_thinking_time(moves.len(), &metadata.clock);
            let result = myopic_brain::search(board, thinking_time)?;
            unimplemented!()
        } else {
            Ok(())
        }
    }

    fn compute_thinking_time(move_count: usize, clock: &Clock) -> Duration {
        // Lets say we predict a chess game will last n moves before the game. Lets say the
        // time limit for the whole game is T seconds. Then for the first n / 2 moves allocate
        // T / n seconds. For the next n / 2 moves allocate T / 2n seconds and so on.
        //
        // So for move m, define i = m // (n / 2) and we allocate t_m = T / (i + 1)n seconds
        //
        // This can be modified to make the decrease in thinking time less sharp
        let n = EXPECTED_MOVE_COUNT;
        let m = move_count as u64 / 2;
        let T = clock.initial;
        let i = m / (n / 2);
        Duration::from_millis(clock.increment + (T / ((i + 1) * n)))
    }
}



fn main() {
    let client = reqwest::blocking::Client::new();

    let response = client.get("https://lichess.org/api/bot/game/stream/mtjmNyVe")
        .bearer_auth("xxx")
        .send()
        .unwrap();

    let mut game = Game::new();
    let mut reader = BufReader::new(response);
    while let read_result = readline(&mut reader) {
        match read_result {
            ReadResult::End => break,
            ReadResult::Err => continue,
            ReadResult::Line(s) => {
                if !s.is_empty() {
                    match serde_json::from_str(s.as_str()) {
                        Err(error) => Err(format!("Error during parse: {:?}", error)),
                        Ok(event) => match event {
                            GameEvent::GameFull {
                                content
                            } => game.process_game_full(content),
                            GameEvent::State {
                                content
                            } => game.process_game_state(content),
                        }
                    };
                }
            },
        }
    }
}

enum ReadResult {
    Line(String),
    Err,
    End,
}

fn readline<R: Read>(bufreader: &mut BufReader<R>) -> ReadResult {
    let mut dest = String::new();
    match bufreader.read_line(&mut dest) {
        Ok(0) => ReadResult::End,
        Ok(_) => ReadResult::Line(String::from(dest.trim())),
        _ => ReadResult::Err,
    }
}
