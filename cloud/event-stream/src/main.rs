extern crate bytes;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::challenge::ChallengeService;
use lichess_events::StreamParams;
use simple_logger::SimpleLogger;
use tokio::try_join;
use warp::Filter;

use crate::config::AppConfig;
use crate::eventprocessor::EventProcessorImpl;
use crate::forwarding::ChallengeRequest;
use crate::gamestart::GameStartService;

mod challenge;
mod challenge_table;
mod config;
mod eventprocessor;
mod forwarding;
mod gamestart;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap_or_else(|e| panic!("Unable to init logger: {}", e));

    let params = AppConfig::default();
    let auth = params.lichess_bot.auth_token.clone();
    let server_addr = params.challenge_server_address.clone();

    // Client instance responsible for all forwarded requests to Lichess
    let client = Arc::new(reqwest::Client::new());

    // Create the http endpoint for creating challenges more ergonomically
    let challenge_forwarding = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("challenge"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(move |user: String, req: ChallengeRequest| {
            let c = client.clone();
            let auth = auth.clone();
            async move { forwarding::challenge(c.as_ref(), auth.clone().as_str(), user, req).await }
        });

    // Concurrently run both the event stream loop and the challenge web server terminating the
    // entire program if either task panics.
    try_join!(
        tokio::task::spawn(async move {
            let params = params.clone();
            lichess_events::stream(
                StreamParams {
                    retry_wait: params.event_loop.stream_retry_wait(),
                    max_lifespan: params.event_loop.max_stream_life(),
                    status_poll_frequency: params.event_loop.status_poll_gap(),
                    our_bot_id: params.lichess_bot.bot_id.clone(),
                    auth_token: params.lichess_bot.auth_token.clone(),
                },
                EventProcessorImpl {
                    challenge_service: ChallengeService::new(&params),
                    gamestart_service: GameStartService::new(&params),
                },
            )
            .await
        }),
        tokio::task::spawn(async move {
            warp::serve(challenge_forwarding).run(server_addr.parse::<SocketAddr>().unwrap()).await
        })
    )
    .unwrap();
}
