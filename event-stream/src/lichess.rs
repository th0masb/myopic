use crate::events::{Challenge, ClockTimeControl};
use anyhow::{anyhow, Error, Result};
use reqwest::{blocking, StatusCode};
use std::io::{BufRead, BufReader};

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const CHALLENGE_ENDPOINT: &'static str = "https://lichess.org/api/challenge";

pub struct LichessClient {
    auth_token: String,
    client: blocking::Client,
}
impl LichessClient {
    pub fn new(auth_token: String) -> LichessClient {
        LichessClient {
            auth_token,
            client: blocking::Client::new(),
        }
    }

    pub fn get_clock(&self, game_id: &str) -> Result<ClockTimeControl> {
        self.client
            .get(format!("{}/stream/{}", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(self.auth_token.as_str())
            .send()
            .map_err(Error::from)
            .map(|response| BufReader::new(response))
            .map(|reader| reader.lines().filter_map(|x| x.ok()))
            .map(|lines| lines.take(10).skip_while(|l| l.is_empty()).next())
            .and_then(|line| line.ok_or(anyhow!("No line could be read from the stream")))
            .and_then(|line| {
                serde_json::from_str::<GameFull>(line.as_str())
                    .map(|game| game.clock.convert())
                    .map_err(|e| anyhow!("Couldn't parse {}: {}", line, e))
            })
            .map_err(|e| anyhow!("Unable to get clock for game: {}", e))
    }

    pub fn abort_game(&self, game_id: &str) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/abort", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map(|response| response.status())
            .map_err(Error::from)
    }

    pub fn post_challenge_decision(
        &self,
        challenge: &Challenge,
        decision: &str,
    ) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/{}", CHALLENGE_ENDPOINT, challenge.id, decision).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map(|response| response.status())
            .map_err(Error::from)
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct GameFull {
    clock: Clock,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct Clock {
    #[serde(rename = "initial")]
    initial_millis: u32,
    #[serde(rename = "increment")]
    increment_millis: u32,
}
impl Clock {
    fn convert(self) -> ClockTimeControl {
        ClockTimeControl {
            limit: self.initial_millis / 1000,
            increment: self.increment_millis / 1000,
        }
    }
}
