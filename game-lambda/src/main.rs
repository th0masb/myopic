mod events;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use reqwest::blocking::{Client, Request, Response};
use std::io::{Read, BufReader, BufRead};
use events::{GameEvent, GameState, Player};
use myopic_core::Side;
use myopic_board::{parse, MutBoard};
use myopic_board::start_position;
use std::time::Duration;

struct Tracking {
    side: Side,
}



const LAMBDA_ID: &'static str = "myopic-bot";

fn main() {
    let client = reqwest::blocking::Client::new();

    let response = client.get("https://lichess.org/api/bot/game/stream/mtjmNyVe")
        .bearer_auth("xxx")
        .send()
        .unwrap();

    let mut tracking = Tracking { side: Side::White };
    let mut reader = BufReader::new(response);
    while let readResult = readline(&mut reader) {
        match readResult {
            ReadResult::End => break,
            ReadResult::Err => continue,
            ReadResult::Line(s) => {
                if !s.is_empty() {
                    match serde_json::from_str(s.as_str()) {
                        Err(error) => Err(format!("Error during parse: {:?}", error)),
                        Ok(event) => match event {
                            GameEvent::GameFull {
                                white, black, clock,  state
                            } => process_game_full(white, black, state, &mut tracking),
                            GameEvent::State {
                                state
                            } => process_game_state(tracking.side, state),
                        }
                    };
                }
            },
        }
    }
}

fn process_game_full(
    white: Player,
    black: Player,
    state: GameState,
    tracking: &mut Tracking
) -> Result<(), String> {
    // Track the side this lambda function is playing as for future gamestates
    match (white.id.as_str(), black.id.as_str()) {
        (LAMBDA_ID, _) => tracking.side = Side::White,
        (_, LAMBDA_ID) => tracking.side = Side::Black,
        _              => return Err(format!("Unrecognized names")),
    }
    process_game_state(tracking.side, state)
}

fn process_game_state(lambda_side: Side, state: GameState) -> Result<(), String> {
    let moves = parse::uci(&state.moves)?;
    let mut board = start_position();
    moves.iter().for_each(|mv| { board.evolve(mv); });

    if board.active() == lambda_side {
        let thinking_time = compute_thinking_time(moves.len(), &state);
        unimplemented!()
    } else {
        Ok(())
    }
}

fn compute_thinking_time(move_count: usize, state: &GameState) -> Duration {
    unimplemented!()
}

//fn main() {
//    let client = reqwest::blocking::Client::new();
//
//    let response = client.get("https://lichess.org/api/stream/event")
//        .bearer_auth("")
//        .send()
//        .unwrap();
//
//    let mut reader = BufReader::new(response);
//    while let readResult = readline(&mut reader) {
//        match readResult {
//            ReadResult::End => break,
//            ReadResult::Err => continue,
//            ReadResult::Line(s) => {
//                if !s.is_empty() {
//                    println!("{}\n", y)
//                }
//            },
//        }
//    }
//}

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
