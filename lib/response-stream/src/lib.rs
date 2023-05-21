use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Response;

pub enum LoopAction<T> {
    Continue,
    Break(T),
}

#[async_trait]
pub trait StreamHandler<T> {
    async fn handle(&mut self, line: String) -> Result<LoopAction<T>>;
}

pub async fn handle<T, H>(response: Response, handler: &mut H) -> Result<Option<T>>
where
    H: StreamHandler<T>,
{
    let mut response_stream = response.bytes_stream();
    while let Some(bytes) = response_stream.next().await {
        let line = String::from_utf8(bytes?.to_vec())?;
        match handler.handle(line.trim().to_owned()).await? {
            LoopAction::Continue => continue,
            LoopAction::Break(result) => return Ok(Some(result)),
        }
    }
    Ok(None)
}
