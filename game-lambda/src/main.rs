mod events;
mod helper;
mod game;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use lambda_runtime::{error::HandlerError, lambda, Context};
use std::io::{Read, BufReader, BufRead};
use events::GameEvent;
use helper::*;
use game::Game;
use std::error::Error;
use simple_logger::SimpleLogger;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";

#[derive(Deserialize, Clone)]
struct PlayGameEvent {
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "authToken")]
    auth_token: String,
    #[serde(rename = "botId")]
    bot_id: String,
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: String,
}

#[derive(Serialize, Clone)]
struct PlayGameOutput {
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;
    lambda!(game_handler);
    Ok(())
}

fn game_handler(
    e: PlayGameEvent,
    _ctx: Context,
) -> Result<PlayGameOutput, HandlerError> {

    let game_stream = reqwest::blocking::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, e.game_id).as_str())
        .bearer_auth(e.auth_token)
        .send()
        .map_err(|err| HandlerError::from(format!("{}", err).as_str()))?;

    let mut game = Game::new(
        e.game_id.to_owned(),
        e.bot_id.to_owned(),
        e.expected_half_moves.parse::<u32>()?,
    );

    let mut reader = BufReader::new(game_stream);
    while let read_result = readline(&mut reader) {
        match read_result {
            ReadResult::End => break,
            ReadResult::Err => continue,
            ReadResult::Line(s) => {
                if !s.is_empty() {
                    // Need to add chat event support
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
    };
    Ok(PlayGameOutput {})
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
