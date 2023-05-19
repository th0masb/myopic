use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::{Duration, Instant};

use lambda_runtime::{Error, LambdaEvent, service_fn};
use reqwest::blocking::Response;
use rusoto_core::Region;
use simple_logger::SimpleLogger;

use game::Game;
use lambda_payloads::chessgame::*;

use crate::compute::MoveLambdaClient;
use crate::game::{GameConfig, GameExecutionState};

mod compute;
mod events;
mod game;
mod lichess;
mod messages;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(service_fn(game_handler)).await?;
    Ok(())
}

async fn game_handler(event: LambdaEvent<PlayGameEvent>) -> Result<PlayGameOutput, Error> {
    log::info!("Initializing game loop");
    let e = &event.payload;
    let mut game = init_game(e)?;
    game.post_introduction().await;
    let (start, wait_duration) = (
        Instant::now(),
        Duration::from_secs(e.abort_after_secs as u64),
    );
    for read_result in open_game_stream(&e.lichess_game_id, &e.lichess_auth_token)?.lines() {
        match read_result {
            Err(error) => {
                log::error!("Problem reading from game stream {}", error);
                return Err(Box::new(error));
            }
            Ok(event) => {
                if event.trim().is_empty() {
                    if game.halfmove_count() < 2 && start.elapsed() > wait_duration {
                        match game.abort().await {
                            Err(error) => {
                                log::error!("Failed to abort game: {}", error);
                                return Err(error.into());
                            }
                            Ok(status) => {
                                if status.is_success() {
                                    log::info!("Successfully aborted game due to inactivity!");
                                    break;
                                } else {
                                    log::warn!("Failed to abort game, lichess status: {}", status)
                                }
                            }
                        }
                    }
                } else {
                    log::info!("Received event: {}", event);
                    match game.process_event(event.as_str()).await? {
                        GameExecutionState::Running => continue,
                        GameExecutionState::Finished => break,
                    }
                }
            }
        }
    }
    Ok(PlayGameOutput {
        message: format!("Game {} completed", e.lichess_game_id),
    })
}

fn init_game(e: &PlayGameEvent) -> Result<Game, Error> {
    Ok(GameConfig {
        game_id: e.lichess_game_id.clone(),
        bot_name: e.lichess_bot_id.clone(),
        lichess_auth_token: e.lichess_auth_token.clone(),
        move_region: Region::from_str(e.move_function_region.as_str())?,
        move_function_name: e.move_function_name.clone(),
    }
    .into())
}

fn open_game_stream(game_id: &String, auth_token: &String) -> Result<BufReader<Response>, Error> {
    Ok(reqwest::blocking::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))?)
}
