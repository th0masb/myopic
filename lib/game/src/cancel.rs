use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait CancellationHook {
    async fn run(&self) -> Result<String>;
}

pub struct EmptyCancellationHook;

#[async_trait]
impl CancellationHook for EmptyCancellationHook {
    async fn run(&self) -> Result<String> {
        Ok(format!(""))
    }
}
