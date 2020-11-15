mod events;
mod helper;
mod game;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::{Read, BufReader, BufRead};
use events::GameEvent;
use helper::*;
use game::Game;

const EXPECTED_HALF_MOVE_COUNT: u32 = 60;
const LAMBDA_ID: &'static str = "myopic-bot";


fn main() {
    let client = reqwest::blocking::Client::new();
    let game_id = "xxx";

    let response = client.get("https://lichess.org/api/bot/game/stream/mtjmNyVe")
        .bearer_auth("xxx")
        .send()
        .unwrap();

    let mut game = Game::new(
        game_id.to_owned(),
        LAMBDA_ID.to_owned(),
        EXPECTED_HALF_MOVE_COUNT
    );
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
