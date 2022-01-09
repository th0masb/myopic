use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ChooseMoveEvent {
    #[serde(rename = "openingTable")]
    pub opening_table: OpeningTable,
    #[serde(rename = "movesPlayed")]
    pub moves_played: String,
    #[serde(rename = "clockMillis")]
    pub clock_millis: Clock,
    #[serde(rename = "tableSize")]
    pub table_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Clock {
    increment: u64,
    remaining: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct OpeningTable {
    pub name: String,
    pub region: String,
    #[serde(rename = "positionKey")]
    pub position_key: String,
    #[serde(rename = "moveKey")]
    pub move_key: String,
    #[serde(rename = "maxDepth")]
    pub max_depth: u8,
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
