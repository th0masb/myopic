use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};
use serde_derive::Deserialize;

use myopic_brain::anyhow::{anyhow, Result};
use myopic_brain::{Board, LookupMoveService, Move};

const TIMEOUT_MS: u64 = 1000;
const MAX_PIECE_COUNT: usize = 7;
const TABLE_ENDPOINT: &'static str = "http://tablebase.lichess.ovh/standard";

#[derive(Default)]
pub struct LichessEndgameClient {
    client: Client,
}

impl LookupMoveService for LichessEndgameClient {
    fn lookup(&mut self, position: Board) -> Result<Option<Move>> {
        let query = position.to_fen().replace(" ", "_");
        let piece_count = position.all_pieces().size();
        if piece_count > MAX_PIECE_COUNT {
            log::info!("Too many pieces to use endgame tables for {}", query);
            Ok(None)
        } else {
            let start = Instant::now();
            let response_result = self.execute_query(query.as_str());
            let query_duration = start.elapsed();
            log::info!("Endgame table query took {}ms", query_duration.as_millis());
            let raw_move = self.process_response(response_result?)?;
            position.parse_uci(raw_move.as_str()).map(|mv| Some(mv))
        }
    }
}

impl LichessEndgameClient {
    fn execute_query(&self, query: &str) -> Result<Response> {
        Ok(self
            .client
            .get(TABLE_ENDPOINT)
            .query(&[("fen", query)])
            .timeout(Duration::from_millis(TIMEOUT_MS))
            .send()?)
    }

    fn process_response(&self, resp: Response) -> Result<String> {
        let response_data = resp.json::<EndgameTableResponse>()?;
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
