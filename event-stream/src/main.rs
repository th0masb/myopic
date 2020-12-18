mod challenge;
mod events;
mod game_start;
mod lichess;
mod params;
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
use anyhow::{Error, Result};
use challenge::ChallengeService;
use events::LichessEvent;
use reqwest::blocking;
use simple_logger::SimpleLogger;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

fn main() -> Result<()> {
    init_environment()?;
    let parameters = ApplicationParameters::load()?;

    loop {
        let event_processor = EventProcessor {
            challenge_service: ChallengeService::new(&parameters),
            gamestart_service: GameStartService::new(&parameters),
        };

        log::info!("Opening event stream");
        for read_result in open_event_stream(&parameters.lichess_auth_token)?.lines() {
            event_processor.handle_stream_read(read_result)
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

struct EventProcessor {
    challenge_service: ChallengeService,
    gamestart_service: GameStartService,
}
impl EventProcessor {
    fn handle_stream_read(&self, read_result: std::io::Result<String>) {
        match read_result {
            Err(read_error) => log::warn!("Stream read error: {}", read_error),
            Ok(line) => {
                if !line.trim().is_empty() {
                    match serde_json::from_str::<LichessEvent>(line.as_str()) {
                        Err(parse_error) => log::warn!("Parse error: {}", parse_error),
                        Ok(event) => match event {
                            LichessEvent::Challenge { challenge } => {
                                // Idea now is in processing the challenge we simply accept it or not
                                // and we do not start the lambda function

                                log::info!("Received challenge event: {}", line);
                                match self.challenge_service.process_challenge(challenge) {
                                    Ok(message) => {
                                        log::info!("Processed challenge with message: {}", message)
                                    }
                                    Err(error) => {
                                        log::warn!("Error processing challenge: {}", error)
                                    }
                                }
                            }
                            LichessEvent::GameStart { game } => {
                                // When we receive the game start we query lichess for game stream
                                // and get the gameFull json first line. We close the stream and
                                // check the parameters before either starting lambda or aborting
                                // game.
                                //
                                // Or we could get the list of all games in progress if we didn't
                                // want to open the stream, only problem here is it limits you to
                                // 49 concurrent games.

                                log::info!("Received game start event: {}", line);
                                match self.gamestart_service.process_gamestart(game) {
                                    Ok(message) => {
                                        log::info!("Processed gamestart with message: {}", message)
                                    }
                                    Err(error) => {
                                        log::warn!("Error processing gamestart: {}", error)
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn open_event_stream(auth_token: &String) -> Result<BufReader<blocking::Response>> {
    blocking::Client::new()
        .get(EVENT_STREAM_ENDPOINT)
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(Error::from)
}
