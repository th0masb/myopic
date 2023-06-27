use async_trait::async_trait;
use bytes::Bytes;
use std::ops::Sub;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};

use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
use myopic_brain::anyhow::{anyhow, Result};
use reqwest::Response;
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use simple_logger::SimpleLogger;

use lambda_payloads::chessgame::*;
use response_stream::{LoopAction, StreamHandler};

use crate::moves::MoveLambdaClient;
use lichess_game::game::{Game, GameConfig, GameExecutionState};
use lichess_game::MoveChooser;
use tokio_util::sync::CancellationToken;

mod moves;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";
const CANCEL_PERIOD_SECS: u64 = 60;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).without_timestamps().init()?;
    lambda_runtime::run(service_fn(game_handler)).await?;
    Ok(())
}

#[async_trait]
pub trait CancellationHook {
    async fn run(&self) -> Result<String>;
}

async fn game_handler(event: LambdaEvent<PlayGameEvent>) -> Result<PlayGameOutput, Error> {
    let e = event.payload;
    let region = Region::from_str(e.move_function_region.as_str())?;
    play_game(
        compute_wait_until_cancel(&event.context)?,
        MoveLambdaClient::from((region.clone(), e.move_function_name.clone())),
        Metadata {
            game_id: e.lichess_game_id.clone(),
            our_bot_id: e.lichess_bot_id.clone(),
            auth_token: e.lichess_auth_token.clone(),
        },
        RecursionHook {
            client: LambdaClient::new(region),
            payload: e.clone(),
            function_arn: event.context.invoked_function_arn,
        },
    )
    .await
    .map_err(Error::from)
    .map(|m| PlayGameOutput { message: m })
}

fn compute_wait_until_cancel(ctx: &Context) -> Result<Duration, Error> {
    Ok(ctx
        .deadline()
        .sub(Duration::from_secs(CANCEL_PERIOD_SECS))
        .duration_since(SystemTime::now())?)
}

struct RecursionHook {
    client: LambdaClient,
    payload: PlayGameEvent,
    function_arn: String,
}

#[async_trait]
impl CancellationHook for RecursionHook {
    async fn run(&self) -> Result<String> {
        log::info!("Recursively calling this function");
        let mut payload = self.payload.clone();
        payload.current_depth += 1;
        if payload.current_depth >= payload.max_depth {
            Err(anyhow!("Can not recurse any further!"))
        } else {
            let response = self
                .client
                .invoke_async(InvokeAsyncRequest {
                    function_name: self.function_arn.clone(),
                    invoke_args: Bytes::from(serde_json::to_string(&payload)?),
                })
                .await?;

            if let Some(202) = response.status {
                Ok(format!("Successfully continued {}", self.payload.lichess_game_id))
            } else {
                Err(anyhow!(
                    "Recursion status {:?} for game {}",
                    response.status,
                    self.payload.lichess_game_id
                ))
            }
        }
    }
}

//

//

//if e.current_depth == 0 {
//    game.post_introduction().await;
//}

struct EmptyHandler;

#[async_trait]
impl CancellationHook for EmptyHandler {
    async fn run(&self) -> Result<String> {
        Ok(format!(""))
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    game_id: String,
    our_bot_id: String,
    auth_token: String,
}

async fn play_game<M, C>(
    cancel_after: Duration,
    moves: M,
    metadata: Metadata,
    on_cancellation: C,
) -> Result<String>
where
    M: MoveChooser + Send + Sync,
    C: CancellationHook,
{
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    tokio::spawn(async move {
        log::info!("Cancelling in {}s", cancel_after.as_secs());
        tokio::time::sleep(cancel_after).await;
        log::info!("Cancelling current lambda invocation");
        cloned_token.cancel();
    });

    // Run the game loop
    log::info!("Initializing game loop");
    let game = init_game(
        moves,
        metadata.game_id.clone(),
        metadata.our_bot_id.clone(),
        metadata.auth_token.clone(),
        token.child_token(),
    )?;
    let mut handler = GameStreamHandler {
        game,
        start: Instant::now(),
        max_wait: Duration::from_secs(30),
        cancel: token.child_token(),
    };
    let game_stream = open_game_stream(&metadata.game_id, &metadata.auth_token).await?;
    match response_stream::handle(game_stream, &mut handler).await? {
        None => Err(anyhow!("Game stream ended unexpectedly!")),
        Some(CompletionType::GameFinished) => Ok(format!("Game {} completed", metadata.game_id)),
        Some(CompletionType::Cancelled) => on_cancellation.run().await,
    }
}

struct GameStreamHandler<M: MoveChooser> {
    game: Game<M>,
    start: Instant,
    max_wait: Duration,
    cancel: CancellationToken,
}

enum CompletionType {
    Cancelled,
    GameFinished,
}

#[async_trait]
impl<M: MoveChooser + Send + Sync> StreamHandler<CompletionType> for GameStreamHandler<M> {
    async fn handle(&mut self, line: String) -> Result<LoopAction<CompletionType>> {
        log::info!("Stream heartbeat");
        if self.cancel.is_cancelled() {
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
                    Err(anyhow!("Failed to abort game, lichess status: {}", abort_status))
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

fn init_game<M: MoveChooser>(
    moves: M,
    game_id: String,
    bot_name: String,
    auth_token: String,
    cancel_token: CancellationToken,
) -> Result<Game<M>> {
    Ok(GameConfig { game_id, bot_name, auth_token, moves, cancel_token }.into())
}

async fn open_game_stream(game_id: &String, auth_token: &String) -> Result<Response> {
    reqwest::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .await
        .map_err(|e| anyhow!(e))
}
