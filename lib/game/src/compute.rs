use anyhow::Result;
use async_trait::async_trait;
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
