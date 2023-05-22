use async_trait::async_trait;
use bytes::Bytes;
use std::ops::Sub;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};

use lambda_runtime::{service_fn, Error, LambdaEvent};
use myopic_brain::anyhow::{anyhow, Result};
use reqwest::Response;
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use simple_logger::SimpleLogger;

use game::Game;
use lambda_payloads::chessgame::*;
use response_stream::{LoopAction, StreamHandler};

use crate::compute::MoveLambdaClient;
use crate::game::{GameConfig, GameExecutionState};
use tokio_util::sync::CancellationToken;

mod compute;
mod events;
mod game;
mod lichess;
mod messages;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";
const CANCEL_PERIOD_SECS: u64 = 60;

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
    let token = CancellationToken::new();

    let cloned_token = token.clone();
    let cancel_wait = event
        .context
        .deadline()
        .sub(Duration::from_secs(CANCEL_PERIOD_SECS))
        .duration_since(SystemTime::now())?;

    tokio::spawn(async move {
        log::info!("Cancelling in {}s", cancel_wait.as_secs());
        tokio::time::sleep(cancel_wait).await;
        log::info!("Cancelling current lambda invocation");
        cloned_token.cancel();
    });

    // Run the game loop
    log::info!("Initializing game loop");
    let e = &event.payload;
    let game = init_game(e, token.child_token())?;
    if e.current_depth == 0 {
        game.post_introduction().await;
    }
    let mut handler = StreamLineHandler {
        game,
        start: Instant::now(),
        max_wait: Duration::from_secs(e.abort_after_secs as u64),
        cancel_token: token.child_token(),
    };
    let game_stream = open_game_stream(&e.lichess_game_id, &e.lichess_auth_token).await?;
    match response_stream::handle(game_stream, &mut handler).await? {
        None => Err(Error::from("Game stream ended unexpectedly!")),
        Some(CompletionType::GameFinished) => Ok(PlayGameOutput {
            message: format!("Game {} completed", e.lichess_game_id),
        }),
        Some(CompletionType::Cancelled) => {
            log::info!("Recursively calling this function");
            let mut payload = event.payload.clone();
            payload.current_depth += 1;
            if payload.current_depth >= payload.max_depth {
                Err(Error::from("Can not recurse any further!"))
            } else {
                let region = Region::from_str(event.payload.move_function_region.as_str())?;
                let response = LambdaClient::new(region)
                    .invoke_async(InvokeAsyncRequest {
                        function_name: event.context.invoked_function_arn.clone(),
                        invoke_args: Bytes::from(serde_json::to_string(&payload)?),
                    })
                    .await?;

                if let Some(202) = response.status {
                    Ok(PlayGameOutput {
                        message: format!("Successfully continued {}", e.lichess_game_id),
                    })
                } else {
                    Err(Error::from(format!(
                        "Recursion status {:?} for game {}",
                        response.status, e.lichess_game_id
                    )))
                }
            }
        }
    }
}

struct StreamLineHandler {
    game: Game,
    start: Instant,
    max_wait: Duration,
    cancel_token: CancellationToken,
}

enum CompletionType {
    Cancelled,
    GameFinished,
}

#[async_trait]
impl StreamHandler<CompletionType> for StreamLineHandler {
    async fn handle(&mut self, line: String) -> Result<LoopAction<CompletionType>> {
        log::info!("Stream heartbeat");
        if self.cancel_token.is_cancelled() {
            log::info!("Cancellation detected! Breaking from game stream");
            return Ok(LoopAction::Break(CompletionType::Cancelled));
        }
        if line.trim().is_empty() {
            if self.game.halfmove_count() < 2 && self.start.elapsed() > self.max_wait {
                let abort_status = self.game.abort().await?;
                if abort_status.is_success() {
                    log::info!("Successfully aborted game due to inactivity!");
                    Ok(LoopAction::Break(CompletionType::GameFinished))
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
                GameExecutionState::Finished => LoopAction::Break(CompletionType::GameFinished),
                GameExecutionState::Cancelled => LoopAction::Break(CompletionType::Cancelled),
            })
        }
    }
}

fn init_game(event: &PlayGameEvent, cancel_token: CancellationToken) -> Result<Game, Error> {
    Ok(GameConfig {
        game_id: event.lichess_game_id.clone(),
        bot_name: event.lichess_bot_id.clone(),
        lichess_auth_token: event.lichess_auth_token.clone(),
        move_region: Region::from_str(event.move_function_region.as_str())?,
        move_function_name: event.move_function_name.clone(),
        cancel_token,
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
