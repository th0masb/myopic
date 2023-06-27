use std::time::Duration;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MoveChooser {
    async fn choose(
        &mut self,
        moves_played: &str,
        remaining: Duration,
        increment: Duration,
    ) -> Result<String>;
}
