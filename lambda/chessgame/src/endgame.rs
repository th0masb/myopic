use async_trait::async_trait;
use std::time::Instant;

use reqwest::{Client, Response};
use serde_derive::Deserialize;
use tokio::time::Duration;

use myopic_brain::anyhow::{anyhow, Error, Result};
use myopic_brain::ChessBoard;

use crate::game::{InitalPosition, LookupService};

const TIMEOUT_MS: u64 = 1500;
const MAX_TABLE_MISSES: usize = 2;
const PIECE_COUNT: usize = 7;
const TABLE_ENDPOINT: &'static str = "http://tablebase.lichess.ovh/standard";

#[derive(Default)]
pub struct EndgameService {
    client: Client,
    table_misses: usize,
}

impl EndgameService {
    async fn execute_query(&self, query: &str) -> Result<Response> {
        self.client
            .get(TABLE_ENDPOINT)
            .query(&[("fen", query)])
            .timeout(Duration::from_millis(TIMEOUT_MS))
            .send()
            .await
            .map_err(Error::from)
    }

    async fn process_response(&mut self, resp: Response) -> Result<Option<String>> {
        resp.json::<EndgameTableResponse>()
            .await
            .map_err(|e| {
                self.table_misses += 1;
                log::info!("Incrementing table misses due to {}", e);
                anyhow!("Problem deserializing response: {}", e)
            })
            .and_then(|r| {
                r.moves
                    .get(0)
                    .map(|sm| {
                        log::info!("Extracted {} from endgame tables", sm.uci);
                        Some(sm.uci.clone())
                    })
                    .ok_or_else(|| {
                        self.table_misses += 1;
                        log::info!("Incrementing table misses due to unknown position");
                        anyhow!("No suggested moves for position!")
                    })
            })
    }
}

#[async_trait]
impl LookupService for EndgameService {
    async fn lookup_move(
        &mut self,
        initial_position: &InitalPosition,
        uci_sequence: &str,
    ) -> Result<Option<String>> {
        let (query, piece_count) = {
            let state = crate::position::get(initial_position, uci_sequence)?;
            (state.to_fen().replace(" ", "_"), state.all_pieces().size())
        };
        if self.table_misses >= MAX_TABLE_MISSES {
            log::info!("Max misses reached, skipping table request for {}", query);
            Ok(None)
        } else if piece_count > PIECE_COUNT {
            log::info!("Too many pieces to use endgame tables for {}", query);
            Ok(None)
        } else {
            let start = Instant::now();
            let response_result = self.execute_query(query.as_str()).await;
            let query_duration = start.elapsed();
            log::info!("Endgame table query took {}ms", query_duration.as_millis());
            match response_result {
                Ok(response) => self.process_response(response).await,
                Err(e) => {
                    self.table_misses += 1;
                    log::info!("Incrementing table misses due to {}", e);
                    Err(e)
                }
            }
        }
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
