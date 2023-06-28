use std::ops::Sub;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};

use async_trait::async_trait;
use bytes::Bytes;
use lambda_runtime::{Context, Error, LambdaEvent, service_fn};
use reqwest::Response;
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use simple_logger::SimpleLogger;
use tokio_util::sync::CancellationToken;

use lambda_payloads::chessgame::*;
use lichess_game::{CancellationHook, MoveChooser};
use myopic_brain::anyhow::{anyhow, Result};

use crate::moves::MoveLambdaClient;

mod moves;

const CANCEL_PERIOD_SECS: u64 = 60;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).without_timestamps().init()?;
    lambda_runtime::run(service_fn(game_handler)).await?;
    Ok(())
}

async fn game_handler(event: LambdaEvent<PlayGameEvent>) -> Result<PlayGameOutput, Error> {
    let e = event.payload;
    let region = Region::from_str(e.move_function_region.as_str())?;
    lichess_game::play_game(
        compute_wait_until_cancel(&event.context)?,
        MoveLambdaClient::from((region.clone(), e.move_function_name.clone())),
        lichess_game::Metadata {
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

