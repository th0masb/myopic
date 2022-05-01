use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ChooseMoveEvent {
    #[serde(rename = "movesPlayed")]
    pub moves_played: String,
    #[serde(rename = "clockMillis")]
    pub clock_millis: ChooseMoveEventClock,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ChooseMoveEventClock {
    pub increment: u64,
    pub remaining: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ChooseMoveOutput {
    #[serde(rename = "bestMove")]
    pub best_move: String,
    #[serde(rename = "searchDetails")]
    pub search_details: Option<SearchDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SearchDetails {
    #[serde(rename = "depthSearched")]
    pub depth_searched: usize,
    #[serde(rename = "searchDurationMillis")]
    pub search_duration_millis: u64,
    pub eval: i32,
}
