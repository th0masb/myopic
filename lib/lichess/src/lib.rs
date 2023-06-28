use std::collections::HashMap;
use anyhow::{anyhow, Error, Result};
use reqwest::StatusCode;
use serde_derive::Deserialize;

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";
const CHALLENGE_ENDPOINT: &'static str = "https://lichess.org/api/challenge";

pub struct LichessClient {
    auth_token: String,
    client: reqwest::Client,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LichessChatRoom {
    Player,
    Spectator,
}

impl LichessClient {
    pub fn new(auth_token: String) -> LichessClient {
        LichessClient { auth_token, client: reqwest::Client::new() }
    }

    pub async fn post_challenge_response(
        &self,
        challenge_id: &str,
        decision: &str,
    ) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/{}", CHALLENGE_ENDPOINT, challenge_id, decision).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .await
            .map(|response| response.status())
            .map_err(Error::from)
    }

    pub async fn abort_game(&self, game_id: &str) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/abort", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .await
            .map_err(|error| anyhow!("Error aborting game {}: {}", game_id, error))
            .map(|response| response.status())
    }

    pub async fn post_move(&self, game_id: &str, mv: &str) -> Result<StatusCode> {
        // Add timeout and retry logic
        self.client
            .post(format!("{}/{}/move/{}", GAME_ENDPOINT, game_id, mv).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .await
            .map_err(|error| anyhow!("Error posting move: {}", error))
            .and_then(|response| {
                if response.status().is_success() {
                    Ok(response.status())
                } else {
                    Err(anyhow!(
                        "Lichess api responded with error {} during move post",
                        response.status()
                    ))
                }
            })
    }

    pub async fn post_chatline(
        &self,
        game_id: &str,
        text: &str,
        room: LichessChatRoom
    ) -> Result<StatusCode> {
        let mut params = HashMap::new();
        params.insert(
            "room",
            match room {
                LichessChatRoom::Player => "player",
                LichessChatRoom::Spectator => "spectator",
            },
        );
        params.insert("text", text);
        self.client
            .post(format!("{}/{}/chat", GAME_ENDPOINT, game_id).as_str())
            .bearer_auth(&self.auth_token)
            .form(&params)
            .send()
            .await
            .map_err(|error| anyhow!("Error posting chatline: {}", error))
            .map(|response| response.status())
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
