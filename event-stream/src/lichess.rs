use anyhow::{anyhow, Error, Result};
use futures_util::StreamExt;
use reqwest::StatusCode;

use crate::events::{Challenge, ClockTimeControl};

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const CHALLENGE_ENDPOINT: &'static str = "https://lichess.org/api/challenge";

pub struct LichessClient {
    auth_token: String,
    client: reqwest::Client,
}

impl LichessClient {
    pub fn new(auth_token: String) -> LichessClient {
        LichessClient {
            auth_token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_clock(&self, game_id: &str) -> Result<ClockTimeControl> {
        let mut gamestream = self
            .client
            .get(format!("{}/stream/{}", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(self.auth_token.as_str())
            .send()
            .await
            .map_err(Error::from)?
            .bytes_stream()
            .take(5);

        while let Some(Ok(chunk)) = gamestream.next().await {
            let line = String::from_utf8(chunk.to_vec()).map_err(Error::from)?;
            if !line.trim().is_empty() {
                return serde_json::from_str::<GameFull>(line.trim())
                    .map(|game| game.clock.convert())
                    .map_err(Error::from);
            }
        }
        Err(anyhow!("No game description found for game: {}", game_id))
    }

    pub async fn abort_game(&self, game_id: &str) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/abort", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .await
            .map(|response| response.status())
            .map_err(Error::from)
    }

    pub async fn post_challenge_decision(
        &self,
        challenge: &Challenge,
        decision: &str,
    ) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/{}", CHALLENGE_ENDPOINT, challenge.id, decision).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .await
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
