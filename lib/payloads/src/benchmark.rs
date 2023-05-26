use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BenchStartEvent {
    pub positions: usize,
    pub depth: usize,
    pub table_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct BenchOutput {
    pub positions_searched: usize,
    pub depth_searched: usize,
    pub min_search_time_millis: u64,
    pub average_search_time_millis: u64,
    pub max_search_time_millis: u64,
    pub median_search_time_millis: u64,
    pub total_search_time_secs: u64,
    pub memory_allocated_mb: usize,
}
