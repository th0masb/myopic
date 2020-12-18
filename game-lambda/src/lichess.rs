use crate::game::GameExecutionState;
use reqwest::{blocking, StatusCode};

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";

#[derive(Debug)]
pub struct LichessService {
    client: blocking::Client,
    auth_token: String,
    game_id: String,
}

impl LichessService {
    pub fn new(auth_token: String, game_id: String) -> LichessService {
        LichessService {
            client: blocking::Client::new(),
            auth_token,
            game_id,
        }
    }

    pub fn abort(&self) -> Result<StatusCode, String> {
        self.client
            .post(format!("{}/{}/abort", GAME_ENDPOINT, self.game_id).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map_err(|error| format!("Error aborting game: {}", error))
            .map(|response| response.status())
    }

    pub fn post_move(&self, mv: String) -> Result<GameExecutionState, String> {
        // Add timeout and retry logic
        self.client
            .post(format!("{}/{}/move/{}", GAME_ENDPOINT, self.game_id, mv).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map_err(|error| format!("Error posting move: {}", error))
            .and_then(|response| if response.status().is_success() {
                Ok(GameExecutionState::Running)
            } else {
                Err(format!("Lichess api responded with error {} during move post", response.status()))
            })
    }
}
