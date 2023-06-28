use lichess_api::LichessClient;

pub struct LichessService {
    pub client: LichessClient,
    pub game_id: String,
}

impl LichessService {
    pub fn new(auth_token: String, game_id: String) -> LichessService {
        LichessService { client: LichessClient::new(auth_token), game_id }
    }
}
