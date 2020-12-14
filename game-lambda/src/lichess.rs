use crate::game::GameExecutionState;
use reqwest::blocking;

const MOVE_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";

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

    pub fn post_move(&self, mv: String) -> Result<GameExecutionState, String> {
        // Add timeout and retry logic
        self.client
            .post(format!("{}/{}/move/{}", MOVE_ENDPOINT, self.game_id, mv).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map(|_| GameExecutionState::Running)
            .map_err(|error| format!("Error posting move: {}", error))
    }
}
