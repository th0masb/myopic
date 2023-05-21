use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Response;

pub enum LoopAction {
    Continue,
    Break,
}

#[async_trait]
pub trait StreamHandler {
    async fn handle(&mut self, line: String) -> LoopAction;
}

pub async fn handle<H: StreamHandler>(response: Response, handler: &mut H) -> Result<()> {
    let mut response_stream = response.bytes_stream();
    while let Some(Ok(raw_line)) = response_stream.next().await {
        match String::from_utf8(raw_line.to_vec()) {
            Err(e) => return Err(anyhow!("Error parsing stream bytes: {}", e)),
            Ok(line) => match handler.handle(line.trim().to_owned()).await {
                LoopAction::Continue => continue,
                LoopAction::Break => break,
            },
        }
    }
    Ok(())
}
