use async_trait::async_trait;
use std::time::{Duration, Instant};

use bytes::Bytes;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};

use lambda_payloads::chessmove2::{ChooseMoveEvent, ChooseMoveOutput};
use myopic_brain::anyhow::{anyhow, Result};

pub struct MoveLambdaClient {
    client: LambdaClient,
    function_name: String,
}

impl From<(Region, String)> for MoveLambdaClient {
    fn from((region, name): (Region, String)) -> Self {
        MoveLambdaClient {
            function_name: name,
            client: LambdaClient::new(region),
        }
    }
}

impl MoveLambdaClient {
    pub(crate) async fn compute_move(&self, payload: ChooseMoveEvent) -> Result<String> {
        log::info!("Request payload {:?}", payload);
        let timer = Instant::now();
        let response = self.client.invoke(InvocationRequest {
            function_name: self.function_name.clone(),
            payload: Some(Bytes::from(serde_json::to_string(&payload)?)),
            client_context: None,
            invocation_type: None,
            log_type: None,
            qualifier: None,
        }).await?;
        log::info!("Response status: {:?}", response.status_code);
        log::info!("Invocation took {}ms", timer.elapsed().as_millis());
        match response.payload {
            None => Err(anyhow!("Missing response payload!")),
            Some(raw_bytes) => {
                let decoded = String::from_utf8(raw_bytes.to_vec())?;
                log::info!("Response payload: {}", decoded);
                let response = serde_json::from_str::<ChooseMoveOutput>(decoded.as_str())?;
                Ok(response.best_move)
            }
        }
    }
}
