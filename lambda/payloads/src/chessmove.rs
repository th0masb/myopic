use serde_derive::{Serialize, Deserialize};

const DEFAULT_TIMEOUT_MILLIS: u64 = 1000;
const DEFAULT_MAX_DEPTH: u8 = 10;
const DEFAULT_TABLE_SIZE: u32 = 50000;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ComputeMoveEvent {
    #[serde(rename = "fen")]
    Fen {
        #[serde(flatten)]
        terminator: SearchTerminator,
        position: String,
        #[serde(rename = "tableSize", default)]
        table_size: TableSize,
    },

    #[serde(rename = "uciSequence")]
    UciSequence {
        #[serde(flatten)]
        terminator: SearchTerminator,
        sequence: String,
        #[serde(rename = "startFen")]
        start_fen: Option<String>,
        #[serde(rename = "tableSize", default)]
        table_size: TableSize,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SearchTerminator {
    #[serde(rename = "maxDepth", default)]
    pub max_depth: MaxDepth,
    #[serde(rename = "timeoutMillis", default)]
    pub timeout_millis: TimeoutMillis,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ComputeMoveOutput {
    #[serde(rename = "bestMove")]
    pub best_move: String,
    #[serde(rename = "depthSearched")]
    pub depth_searched: usize,
    #[serde(rename = "searchDurationMillis")]
    pub search_duration_millis: u64,
    pub eval: i32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct TableSize(pub u32);
impl Default for TableSize {
    fn default() -> Self {
        TableSize(DEFAULT_TABLE_SIZE)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct MaxDepth(pub u8);
impl Default for MaxDepth {
    fn default() -> Self {
        MaxDepth(DEFAULT_MAX_DEPTH)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct TimeoutMillis(pub u64);
impl Default for TimeoutMillis {
    fn default() -> Self {
        TimeoutMillis(DEFAULT_TIMEOUT_MILLIS)
    }
}

#[cfg(test)]
mod test {
    use super::{ComputeMoveEvent, MaxDepth, SearchTerminator, TimeoutMillis, TableSize};

    #[test]
    fn deserialize_default_tablesize() {
        assert_eq!(
            ComputeMoveEvent::Fen {
                position: "pos".to_string(),
                table_size: TableSize(super::DEFAULT_TABLE_SIZE),
                terminator: SearchTerminator {
                    max_depth: MaxDepth(super::DEFAULT_MAX_DEPTH),
                    timeout_millis: TimeoutMillis(super::DEFAULT_TIMEOUT_MILLIS),
                }
            },
            serde_json::from_str(r#"{"type":"fen","position":"pos"}"#).unwrap()
        );
    }
}
