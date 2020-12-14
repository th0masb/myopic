mod events;
mod params;
mod service;

extern crate bytes;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

use crate::params::ApplicationParameters;
use events::LichessEvent;
use reqwest::blocking;
use service::LichessService;

use simple_logger::SimpleLogger;

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

const EVENT_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/stream/event";

fn main() -> Result<(), Box<dyn Error>> {
    init_environment()?;
    let parameters = ApplicationParameters::load()?;

    loop {
        let service = LichessService::new(parameters.clone());
        log::info!("Opening event stream");
        for read_result in open_event_stream(&parameters.lichess_auth_token)?.lines() {
            handle_stream_read(&service, read_result)
        }

        log::info!(
            "Sleeping for {} seconds",
            parameters.retry_wait_duration_secs
        );
        thread::sleep(Duration::from_secs(parameters.retry_wait_duration_secs));
    }

    Ok(())
}

fn init_environment() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    Ok(())
}

fn handle_stream_read(service: &LichessService, read_result: std::io::Result<String>) {
    match read_result {
        Err(read_error) => log::warn!("Stream read error: {}", read_error),
        Ok(line) => {
            if !line.trim().is_empty() {
                match serde_json::from_str::<LichessEvent>(line.as_str()) {
                    Err(parse_error) => log::warn!("Parse error: {}", parse_error),
                    Ok(event) => match event {
                        LichessEvent::Challenge { challenge } => {
                            log::info!("Received challenge event: {}", line);
                            match service.process_challenge(challenge) {
                                Ok(message) => {
                                    log::info!("Processed challenge with message: {}", message)
                                }
                                Err(error) => log::warn!("Error processing challenge: {}", error),
                            }
                        }
                        LichessEvent::GameStart { game: _ } => {
                            log::info!("Received game start event: {}", line);
                        }
                    },
                }
            }
        }
    }
}

fn open_event_stream(auth_token: &String) -> Result<BufReader<blocking::Response>, Box<dyn Error>> {
    blocking::Client::new()
        .get(EVENT_STREAM_ENDPOINT)
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}
