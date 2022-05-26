/// TODO Duplicated from "payloads" crate which cannot be added
/// TODO as a dependency due to issue with cross compilation
/// TODO when depending on local crate.
use serde_derive::{Deserialize, Serialize};

/// The input payload of this lambda
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayGameEvent {
    /// The name of the lambda function move searching is delegated to
    #[serde(rename = "moveFunctionName")]
    pub move_function_name: String,
    /// The region of the lambda function move searching is delegated to
    #[serde(rename = "moveFunctionRegion")]
    pub move_function_region: String,
    /// The lichess game id this lambda will participate in
    #[serde(rename = "lichessGameId")]
    pub lichess_game_id: String,
    /// An auth token for the lichess bot this lambda will play as
    #[serde(rename = "lichessAuthToken")]
    pub lichess_auth_token: String,
    /// The id of the lichess bot this lambda will play as
    #[serde(rename = "lichessBotId")]
    pub lichess_bot_id: String,
    /// How many seconds to wait for the first full move to take place
    /// before aborting the game
    #[serde(rename = "abortAfterSecs")]
    pub abort_after_secs: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayGameOutput {
    pub message: String,
}
