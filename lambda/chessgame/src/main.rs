use async_trait::async_trait;
use std::str::FromStr;
use std::time::{Duration, Instant};

use lambda_runtime::{service_fn, Error, LambdaEvent};
use myopic_brain::anyhow::{anyhow, Result};
use reqwest::Response;
use rusoto_core::Region;
use simple_logger::SimpleLogger;

use game::Game;
use lambda_payloads::chessgame::*;
use response_stream::{LoopAction, StreamHandler};

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
    let game = init_game(e)?;
    game.post_introduction().await;
    let mut handler = StreamLineHandler {
        game,
        start: Instant::now(),
        max_wait: Duration::from_secs(e.abort_after_secs as u64),
    };
    let game_stream = open_game_stream(&e.lichess_game_id, &e.lichess_auth_token).await?;
    match response_stream::handle(game_stream, &mut handler).await? {
        None => {}
        Some(_) => {}
    }
    Ok(PlayGameOutput {
        message: format!("Game {} completed", e.lichess_game_id),
    })
}

struct StreamLineHandler {
    game: Game,
    start: Instant,
    max_wait: Duration,
}

enum CompletionType {
    Recursive,
    Terminal,
}

#[async_trait]
impl StreamHandler<CompletionType> for StreamLineHandler {
    async fn handle(&mut self, line: String) -> Result<LoopAction<CompletionType>> {
        if line.trim().is_empty() {
            if self.game.halfmove_count() < 2 && self.start.elapsed() > self.max_wait {
                let abort_status = self.game.abort().await?;
                if abort_status.is_success() {
                    log::info!("Successfully aborted game due to inactivity!");
                    Ok(LoopAction::Break(CompletionType::Terminal))
                } else {
                    Err(anyhow!(
                        "Failed to abort game, lichess status: {}",
                        abort_status
                    ))
                }
            } else {
                Ok(LoopAction::Continue)
            }
        } else {
            log::info!("Received event: {}", line);
            Ok(match self.game.process_event(line.as_str()).await? {
                GameExecutionState::Running => LoopAction::Continue,
                GameExecutionState::Finished => LoopAction::Break(CompletionType::Terminal),
            })
        }
    }
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

async fn open_game_stream(game_id: &String, auth_token: &String) -> Result<Response, Error> {
    reqwest::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .await
        .map_err(Error::from)
}
