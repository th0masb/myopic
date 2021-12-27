use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};

use lambda_payloads::chessmove::*;

use crate::game::{ComputeService, InitalPosition};

pub struct LambdaMoveComputeService {
    pub region: Region,
    pub function_name: String,
}

impl ComputeService for LambdaMoveComputeService {
    fn compute_move(
        &self,
        initial_position: &InitalPosition,
        uci_sequence: &str,
        time_limit: Duration,
    ) -> Result<String> {
        let payload = new_request_payload(initial_position, uci_sequence, time_limit)?;
        log::info!("Request payload {}", payload);
        let timer = Instant::now();
        let invocation = tokio::runtime::Runtime::new().unwrap().block_on(
            LambdaClient::new(self.region.clone()).invoke(InvocationRequest {
                function_name: self.function_name.clone(),
                payload: Some(Bytes::from(payload)),
                client_context: None,
                invocation_type: None,
                log_type: None,
                qualifier: None,
            }),
        )?;
        log::info!("Response status: {:?}", invocation.status_code);
        log::info!("Invocation took {}ms", timer.elapsed().as_millis());
        match invocation.payload {
            None => Err(anyhow!("Missing response payload!")),
            Some(raw_bytes) => {
                let decoded = String::from_utf8(raw_bytes.to_vec())?;
                log::info!("Response payload: {}", decoded);
                let response = serde_json::from_str::<ComputeMoveOutput>(decoded.as_str())?;
                Ok(response.best_move)
            }
        }
    }
}

fn new_request_payload(
    initial_position: &InitalPosition,
    sequence: &str,
    limit: Duration
) -> serde_json::Result<String> {
    serde_json::to_string(&ComputeMoveEvent::UciSequence {
        table_size: Default::default(),
        sequence: sequence.to_string(),
        terminator: SearchTerminator {
            max_depth: Default::default(),
            timeout_millis: TimeoutMillis(limit.as_millis() as u64)
        },
        start_fen: match initial_position {
            InitalPosition::Start => None,
            InitalPosition::CustomFen(fen) => Some(fen.clone()),
        },
    })
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    #[test]
    fn check_bytes_conversion() {
        let input = r#"{"someJson": "string", "n": 5}"#.to_string();
        let bytes = Bytes::from(input.clone());
        assert_eq!(
            input,
            String::from_utf8(bytes.to_vec()).expect("Unable to decode bytes!")
        )
    }
}
