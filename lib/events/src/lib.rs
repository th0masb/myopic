use std::time::{Duration, Instant};

use anyhow::{Error, Result};
use async_trait::async_trait;

use response_stream::LoopAction;

pub use crate::events::LichessEvent;
use crate::processor::StreamLineProcessor;
use crate::userstatus::StatusService;

pub mod events;
pub mod lichess;
pub mod processor;
pub mod userstatus;

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

pub struct StreamParams {
    pub status_poll_frequency: Duration,
    pub max_lifespan: Duration,
    pub retry_wait: Duration,
}

#[async_trait]
pub trait EventProcessor {
    async fn process(&mut self, event: LichessEvent);
}

pub async fn stream<E: EventProcessor + Send + Sync>(
    our_bot_id: String,
    stream_params: StreamParams,
    processor: E,
) {
    let mut event_processor = StreamLineProcessor {
        status_service: StatusService::new(
            our_bot_id.as_str(),
            stream_params.status_poll_frequency,
        ),
        event_processor: processor,
    };
    loop {
        log::info!("Opening event stream");

        let mut handler = StreamRefreshHandler {
            start: Instant::now(),
            max_duration: stream_params.max_lifespan,
            processor: &mut event_processor,
        };

        match open_event_stream(our_bot_id.as_str()).await {
            Err(e) => log::warn!("Cannot connect to event stream {}", e),
            Ok(response) => match response_stream::handle(response, &mut handler).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                }
            },
        }

        log::info!("Sleeping for {} seconds", stream_params.retry_wait.as_secs());
        tokio::time::sleep(stream_params.retry_wait).await;
    }
}

struct StreamRefreshHandler<'a, E: EventProcessor> {
    start: Instant,
    max_duration: Duration,
    processor: &'a mut StreamLineProcessor<E>,
}

#[async_trait]
impl<E: EventProcessor + Sync + Send> response_stream::StreamHandler<()>
    for StreamRefreshHandler<'_, E>
{
    async fn handle(&mut self, line: String) -> Result<LoopAction<()>> {
        let elapsed = self.start.elapsed();
        Ok(if elapsed > self.max_duration {
            log::info!("Refreshing event stream after {} mins", elapsed.as_secs() / 60);
            LoopAction::Break(())
        } else {
            self.processor.handle_stream_read(line.as_str()).await
        })
    }
}

async fn open_event_stream(auth_token: &str) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .get(EVENT_STREAM_ENDPOINT)
        .bearer_auth(auth_token)
        .send()
        .await
        .map_err(Error::from)
}
