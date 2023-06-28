use std::fmt::{Display, Formatter};
use std::time::Instant;

use async_trait::async_trait;
use reqwest::{Client, Response};
use serde_derive::Deserialize;
use tokio::time::Duration;

use myopic_brain::anyhow::{anyhow, Result};
use myopic_brain::{Board, Move};

use crate::LookupMoveService;

const TIMEOUT_MS: u64 = 1000;
const MAX_PIECE_COUNT: usize = 7;
const TABLE_ENDPOINT: &'static str = "http://tablebase.lichess.ovh/standard";

#[derive(Default)]
pub struct LichessEndgameService {
    client: Client,
}

impl Display for LichessEndgameService {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(TABLE_ENDPOINT)
    }
}

#[async_trait]
impl LookupMoveService for LichessEndgameService {
    async fn lookup(&self, position: Board) -> Result<Option<Move>> {
        let query = position.to_fen().replace(" ", "_");
        let piece_count = position.all_pieces().size();
        if piece_count > MAX_PIECE_COUNT {
            log::info!("Too many pieces to use endgame tables for {}", query);
            Ok(None)
        } else {
            let start = Instant::now();
            let response_result = self.execute_query(query.as_str()).await;
            let query_duration = start.elapsed();
            log::info!("Endgame table query took {}ms", query_duration.as_millis());
            let raw_move = self.process_response(response_result?).await?;
            position.parse_uci(raw_move.as_str()).map(|mv| Some(mv))
        }
    }
}

impl LichessEndgameService {
    async fn execute_query(&self, query: &str) -> Result<Response> {
        Ok(self
            .client
            .get(TABLE_ENDPOINT)
            .query(&[("fen", query)])
            .timeout(Duration::from_millis(TIMEOUT_MS))
            .send()
            .await?)
    }

    async fn process_response(&self, resp: Response) -> Result<String> {
        let response_data = resp.json::<EndgameTableResponse>().await?;
        response_data
            .moves
            .get(0)
            .map(|mv| mv.uci.clone())
            .ok_or(anyhow!("No suggested moves for given position"))
    }
}

#[derive(Deserialize)]
struct EndgameTableResponse {
    moves: Vec<SuggestedMove>,
}

#[derive(Deserialize)]
struct SuggestedMove {
    uci: String,
}
