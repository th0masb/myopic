use std::time::{Duration, Instant};

use anyhow::{Error, Result};
use async_trait::async_trait;
use futures_util::StreamExt;

use crate::challenge::ChallengeService;
use crate::config::AppConfig;
use crate::eventprocessor::EventProcessor;
use crate::gamestart::GameStartService;
use crate::userstatus::StatusService;
use response_stream::LoopAction;

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

pub async fn stream(params: AppConfig) {
    let mut event_processor = EventProcessor {
        challenge_service: ChallengeService::new(&params),
        gamestart_service: GameStartService::new(&params),
        status_service: StatusService::new(&params),
    };
    loop {
        log::info!("Opening event stream");

        let mut handler = StreamRefreshHandler {
            start: Instant::now(),
            max_duration: params.event_loop.max_stream_life(),
            processor: &mut event_processor,
        };

        match open_event_stream(&params.lichess_bot.auth_token).await {
            Err(e) => log::warn!("Cannot connect to event stream {}", e),
            Ok(response) => match response_stream::handle(response, &mut handler).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                }
            },
        }

        let wait = params.event_loop.stream_retry_wait();
        log::info!("Sleeping for {} seconds", wait.as_secs());
        tokio::time::sleep(wait).await;
    }
}

struct StreamRefreshHandler<'a> {
    start: Instant,
    max_duration: Duration,
    processor: &'a mut EventProcessor,
}

#[async_trait]
impl response_stream::StreamHandler for StreamRefreshHandler<'_> {
    async fn handle(&mut self, line: String) -> Result<LoopAction> {
        let elapsed = self.start.elapsed();
        Ok(if elapsed > self.max_duration {
            log::info!(
                "Refreshing event stream after {} mins",
                elapsed.as_secs() / 60
            );
            LoopAction::Break
        } else {
            self.processor.handle_stream_read(line.as_str()).await
        })
    }
}

async fn open_event_stream(auth_token: &String) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .get(EVENT_STREAM_ENDPOINT)
        .bearer_auth(auth_token)
        .send()
        .await
        .map_err(Error::from)
}
