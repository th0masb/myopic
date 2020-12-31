mod challenge;
mod events;
mod game_start;
mod lichess;
mod params;
mod user_status;
mod validity;

extern crate bytes;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

use crate::game_start::GameStartService;
use crate::params::ApplicationParameters;
use crate::user_status::StatusService;
use anyhow::{Error, Result};
use challenge::ChallengeService;
use events::LichessEvent;
use reqwest::blocking;
use simple_logger::SimpleLogger;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

enum LoopAction {
    Continue,
    Break,
}

fn main() -> Result<()> {
    init_environment()?;
    let parameters = ApplicationParameters::load()?;

    loop {
        let mut event_processor = EventProcessor {
            challenge_service: ChallengeService::new(&parameters),
            gamestart_service: GameStartService::new(&parameters),
            status_service: StatusService::new(&parameters),
        };

        log::info!("Opening event stream");
        let start = Instant::now();
        let max_stream_duration = Duration::from_secs(parameters.max_stream_life_mins * 60);
        match open_event_stream(&parameters.lichess_auth_token) {
            Err(e) => {
                log::warn!("Cannot connect to event stream: {}", e)
            }
            Ok(rdr) => {
                for read_result in rdr.lines() {
                    let elapsed = start.elapsed();
                    if elapsed > max_stream_duration {
                        log::info!("Refreshing event stream which has been alive for {} mins", elapsed.as_secs() / 60);
                        break;
                    }
                    match event_processor.handle_stream_read(read_result) {
                        LoopAction::Continue => continue,
                        LoopAction::Break => break,
                    }
                }

            }
        }

        log::info!(
            "Sleeping for {} seconds",
            parameters.retry_wait_duration_secs
        );
        thread::sleep(Duration::from_secs(parameters.retry_wait_duration_secs));
    }

    Ok(())
}

fn init_environment() -> Result<()> {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    Ok(())
}

fn open_event_stream(auth_token: &String) -> Result<BufReader<blocking::Response>> {
    blocking::Client::new()
        .get(EVENT_STREAM_ENDPOINT)
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(Error::from)
}

struct EventProcessor {
    challenge_service: ChallengeService,
    gamestart_service: GameStartService,
    status_service: StatusService,
}
impl EventProcessor {
    fn handle_stream_read(&mut self, read_result: std::io::Result<String>) -> LoopAction {
        match read_result {
            Err(read_error) => {
                log::warn!("Stream read error: {}", read_error);
                LoopAction::Break
            }
            Ok(line) => {
                if line.trim().is_empty() {
                    self.user_status()
                } else {
                    match serde_json::from_str::<LichessEvent>(line.as_str()) {
                        Err(parse_error) => log::warn!("Parse error: {}", parse_error),
                        Ok(event) => {
                            log::info!("Received event: {}", line.as_str());
                            self.handle_event(event)
                        }
                    };
                    LoopAction::Continue
                }
            }
        }
    }

    fn user_status(&mut self) -> LoopAction {
        match self.status_service.user_status() {
            Err(e) => {
                log::warn!("Error fetching user status: {}", e);
                LoopAction::Continue
            }
            Ok(None) => LoopAction::Continue,
            Ok(Some(status)) => {
                if status.online {
                    LoopAction::Continue
                } else {
                    LoopAction::Break
                }
            }
        }
    }

    fn handle_event(&self, event: LichessEvent) {
        match event {
            LichessEvent::Challenge { challenge } => {
                match self.challenge_service.process_challenge(challenge) {
                    Ok(message) => log::info!("Processed challenge with message: {}", message),
                    Err(error) => log::warn!("Error processing challenge: {}", error),
                }
            }
            LichessEvent::GameStart { game } => {
                match self.gamestart_service.process_gamestart(game) {
                    Ok(message) => log::info!("Processed gamestart with message: {}", message),
                    Err(error) => log::warn!("Error processing gamestart: {}", error),
                }
            }
        }
    }
}
