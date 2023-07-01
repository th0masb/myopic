use anyhow::Result;
use async_trait::async_trait;
use myopic_brain::{Board, ComputeMoveInput, Engine};
use std::time::Duration;

#[async_trait]
pub trait MoveChooser {
    async fn choose(
        &mut self,
        moves_played: &str,
        remaining: Duration,
        increment: Duration,
    ) -> Result<String>;
}

#[async_trait]
impl MoveChooser for Engine {
    async fn choose(
        &mut self,
        moves_played: &str,
        remaining: Duration,
        increment: Duration,
    ) -> Result<String> {
        let mut position = Board::default();
        position.play_uci(moves_played)?;
        tokio::task::block_in_place(|| {
            self.compute_move(ComputeMoveInput { position, remaining, increment })
        })
        .map(|output| {
            match output.search_details {
                None => log::info!("Used move from lookup"),
                Some(details) => {
                    let formatted = serde_json::to_string(&details).unwrap_or("error".to_string());
                    log::info!("Computed: {}", formatted);
                }
            };
            output.best_move.uci_format()
        })
    }
}
