mod events;
mod game;
mod helper;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use crate::game::GameExecutionState;
use game::Game;
use lambda_runtime::{error::HandlerError, lambda, Context};
use reqwest::blocking::Response;
use simple_logger::SimpleLogger;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::env;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";

#[derive(Serialize, Deserialize, Clone)]
struct PlayGameEvent {
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "authToken")]
    auth_token: String,
    #[serde(rename = "botId")]
    bot_id: String,
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: u32,
}

#[derive(Serialize, Clone)]
struct PlayGameOutput {
    output: String
}

/// Entry point for the lambda function implementation
fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;
    lambda!(game_handler);
    Ok(())
}

fn game_handler(e: PlayGameEvent, _ctx: Context) -> Result<PlayGameOutput, HandlerError> {
    log::info!("Initializing game loop");
    let mut game = Game::new(
        e.game_id.clone(),
        e.bot_id,
        e.expected_half_moves,
        e.auth_token.clone(),
    );

    let mut reader = open_game_stream(e.game_id, e.auth_token)?;
    while let read_result = readline(&mut reader)? {
        match read_result {
            ReadResult::End => break,
            ReadResult::Line(event) => {
                if !event.is_empty() {
                    log::info!("Received event: {}", event);
                    match game
                        .process_event(event.as_str())
                        .map_err(|err| HandlerError::from(err.as_str()))?
                    {
                        GameExecutionState::Running => continue,
                        GameExecutionState::Finished => break,
                    }
                }
            }
        }
    }
    Ok(PlayGameOutput {output: format!("finished")})
}

fn open_game_stream(
    game_id: String,
    auth_token: String,
) -> Result<BufReader<Response>, HandlerError> {
    reqwest::blocking::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(|err| HandlerError::from(format!("{}", err).as_str()))
}

enum ReadResult {
    Line(String),
    End,
}

fn readline<R: Read>(bufreader: &mut BufReader<R>) -> Result<ReadResult, HandlerError> {
    let mut dest = String::new();
    match bufreader.read_line(&mut dest) {
        Ok(0) => Ok(ReadResult::End),
        Ok(_) => Ok(ReadResult::Line(String::from(dest.trim()))),
        Err(error) => Err(HandlerError::from(format!("{}", error).as_str())),
    }
}
