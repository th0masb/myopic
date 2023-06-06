use anyhow::{Error, Result};
use reqwest::StatusCode;

use crate::events::Challenge;
use crate::ChallengeRequest;

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const CHALLENGE_ENDPOINT: &'static str = "https://lichess.org/api/challenge";

pub struct LichessClient {
    auth_token: String,
    client: reqwest::Client,
}

impl LichessClient {
    pub fn new(auth_token: String) -> LichessClient {
        LichessClient { auth_token, client: reqwest::Client::new() }
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

    pub async fn post_challenge(
        &self,
        username: &str,
        challenge_params: &ChallengeRequest,
    ) -> Result<(StatusCode, String)> {
        let response = self
            .client
            .post(format!("{}/{}", CHALLENGE_ENDPOINT, username))
            .bearer_auth(&self.auth_token)
            .form(challenge_params)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        Ok((status, body))
    }

    pub async fn post_challenge_response(
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
