mod events;
mod game;
mod helper;

extern crate bytes;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

use crate::game::{GameExecutionState, GameProps};
use crate::helper::timestamp_millis;
use bytes::Bytes;
use game::Game;
use lambda_runtime::{error::HandlerError, lambda, Context};
use reqwest::blocking::Response;
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use simple_logger::SimpleLogger;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;
use std::time::{Duration, Instant};

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";

/// The input payload of this lambda
#[derive(Serialize, Deserialize, Clone)]
struct PlayGameEvent {
    /// The current call depth of the lambda invokation
    depth: u8,
    /// The region this lambda is deployed in
    region: String,
    /// The name of this lambda function
    #[serde(rename = "functionName")]
    function_name: String,
    /// The lichess game id this lambda will participate in
    #[serde(rename = "gameId")]
    game_id: String,
    /// An auth token for the lichess bot this lambda will play as
    #[serde(rename = "authToken")]
    auth_token: String,
    /// The id of the lichess bot this lambda will play as
    #[serde(rename = "botId")]
    bot_id: String,
    /// How many half moves we expect the game to last for
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: u32,
}

impl PlayGameEvent {
    fn decrement_depth(&self) -> PlayGameEvent {
        let mut new_event = self.clone();
        new_event.depth = self.depth - 1;
        new_event
    }
}

#[derive(Serialize, Clone)]
struct PlayGameOutput {
    message: String,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeConstraints {
    start_time: Instant,
    max_execution_duration: Duration,
}

impl TimeConstraints {
    pub fn lambda_end_instant(&self) -> Instant {
        self.start_time.add(self.max_execution_duration)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;
    lambda!(game_handler);
    Ok(())
}

fn init_game(e: &PlayGameEvent, ctx: &Context) -> Game {
    Game::new(GameProps {
        game_id: e.game_id.clone(),
        lambda_player_id: e.bot_id.clone(),
        expected_half_moves: e.expected_half_moves,
        auth_token: e.auth_token.clone(),
        time_constraints: TimeConstraints {
            start_time: Instant::now(),
            // Reduce the actual max duration by a constant fraction
            // to allow a buffer of time to invoke new lambda
            max_execution_duration: (4 * Duration::from_millis(
                ctx.deadline as u64 - timestamp_millis(),
            )) / 5,
        },
    })
}

fn game_handler(e: PlayGameEvent, ctx: Context) -> Result<PlayGameOutput, HandlerError> {
    log::info!("Initializing game loop");
    let mut game = init_game(&e, &ctx);

    // Enter the game loop
    let mut invoke_next = false;
    for read_result in open_game_stream(&e.game_id, &e.auth_token)?.lines() {
        match read_result {
            Err(error) => {
                log::warn!("Problem reading from game stream {}", error);
                break;
            }
            Ok(event) => {
                if event.trim().is_empty() {
                    if Instant::now() >= game.time_constraints().lambda_end_instant() {
                        invoke_next = true;
                        break;
                    }
                } else {
                    log::info!("Received event: {}", event);
                    match game
                        .process_event(event.as_str())
                        .map_err(|err| HandlerError::from(err.as_str()))?
                    {
                        GameExecutionState::Running => continue,
                        GameExecutionState::Finished => break,
                        GameExecutionState::Recurse => {
                            invoke_next = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    // If we got here then there isn't enough time in this lambda to complete the game
    if invoke_next && e.depth > 0 {
        // Async invoke lambda here
        recurse(&e)
    } else {
        Ok(PlayGameOutput {
            message: format!("No recursion required, game execution thread terminated"),
        })
    }
}

fn recurse(source_event: &PlayGameEvent) -> Result<PlayGameOutput, HandlerError> {
    // Inject region as part of the PlayGameEvent
    let next_event = source_event.decrement_depth();
    let region = Region::from_str(next_event.region.as_str())
        .map_err(|err| HandlerError::from(err.to_string().as_str()))?;
    tokio::runtime::Runtime::new()
        .map_err(|err| HandlerError::from(err.to_string().as_str()))?
        .block_on(LambdaClient::new(region).invoke_async(InvokeAsyncRequest {
            function_name: next_event.function_name.clone(),
            invoke_args: Bytes::from(serde_json::to_string(&next_event)?),
        }))
        .map_err(|err| HandlerError::from(err.to_string().as_str()))
        .map(|response| PlayGameOutput {
            message: format!(
                "Recursively invoked lambda at depth {} with status {:?}",
                next_event.depth, response.status
            ),
        })
}

fn open_game_stream(
    game_id: &String,
    auth_token: &String,
) -> Result<BufReader<Response>, HandlerError> {
    reqwest::blocking::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(|err| HandlerError::from(format!("{}", err).as_str()))
}
