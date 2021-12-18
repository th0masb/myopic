use std::time::{Duration, Instant};

use anyhow::{Error, Result};
use futures_util::StreamExt;

use crate::challenge::ChallengeService;
use crate::config::AppConfig;
use crate::eventprocessor::EventProcessor;
use crate::gamestart::GameStartService;
use crate::userstatus::StatusService;

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

pub enum LoopAction {
    Continue,
    Break,
}

pub async fn stream(params: AppConfig) {
    loop {
        let mut event_processor = EventProcessor {
            challenge_service: ChallengeService::new(&params),
            gamestart_service: GameStartService::new(&params),
            status_service: StatusService::new(&params),
        };

        log::info!("Opening event stream");
        let start = Instant::now();
        let max_stream_duration = Duration::from_secs((params.event_loop.max_stream_life_mins * 60) as u64);

        match open_event_stream(&params.lichess_bot.auth_token).await {
            Err(e) => log::warn!("Cannot connect to event stream {}", e),
            Ok(eventstream_resp) => {
                let mut eventstream = eventstream_resp.bytes_stream();
                while let Some(Ok(raw_line)) = eventstream.next().await {
                    let elapsed = start.elapsed();
                    if elapsed > max_stream_duration {
                        log::info!(
                            "Refreshing event stream after {} mins",
                            elapsed.as_secs() / 60
                        );
                        break;
                    }
                    match String::from_utf8(raw_line.to_vec()) {
                        Err(e) => {
                            log::error!("Error parsing stream bytes: {}", e);
                            break;
                        }
                        Ok(line) => match event_processor.handle_stream_read(line.trim()).await {
                            LoopAction::Continue => continue,
                            LoopAction::Break => break,
                        },
                    }
                }
            }
        }

        log::info!("Sleeping for {} seconds", params.event_loop.retry_wait_duration_secs);
        let sleep_duration = Duration::from_secs(params.event_loop.retry_wait_duration_secs as u64);
        tokio::time::sleep(sleep_duration).await;
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
