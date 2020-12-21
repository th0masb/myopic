use crate::game::{ComputeService, InitalPosition};
use bytes::Bytes;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use serde_derive::{Deserialize, Serialize};
use simple_error::SimpleError;
use std::error::Error;
use std::time::{Duration, Instant};

pub struct LambdaMoveComputeService {
    region: Region,
    function_name: String,
}
impl Default for LambdaMoveComputeService {
    fn default() -> Self {
        LambdaMoveComputeService {
            region: Region::EuWest2,
            function_name: format!("MyopicMove"),
        }
    }
}

#[derive(Serialize, Clone)]
struct RequestPayload {
    #[serde(rename = "type")]
    payload_type: String,
    sequence: String,
    #[serde(rename = "startFen")]
    start_fen: Option<String>,
    #[serde(rename = "timeoutMillis")]
    timeout_millis: u64,
}
impl RequestPayload {
    fn new(initial_position: &InitalPosition, sequence: &str, limit: Duration) -> RequestPayload {
        RequestPayload {
            start_fen: match initial_position {
                InitalPosition::Start => None,
                InitalPosition::CustomFen(fen) => Some(fen.clone()),
            },
            payload_type: format!("uciSequence"),
            sequence: sequence.to_string(),
            timeout_millis: limit.as_millis() as u64,
        }
    }
}

#[derive(Deserialize, Clone)]
struct ResponsePayload {
    #[serde(rename = "bestMove")]
    best_move: String,
    #[serde(rename = "depthSearched")]
    depth_searched: usize,
    #[serde(rename = "searchDurationMillis")]
    search_duration_millis: u64,
    eval: i32,
}

impl ComputeService for LambdaMoveComputeService {
    fn compute_move(
        &self,
        initial_position: &InitalPosition,
        uci_sequence: &str,
        time_limit: Duration,
    ) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::to_string(&RequestPayload::new(
            initial_position,
            uci_sequence,
            time_limit,
        ))?;
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
            None => Err(Box::new(SimpleError::new("Missing response payload!")) as Box<dyn Error>),
            Some(raw_bytes) => {
                let decoded = String::from_utf8(raw_bytes.to_vec())?;
                log::info!("Response payload: {}", decoded);
                let response = serde_json::from_str::<ResponsePayload>(decoded.as_str())?;
                Ok(response.best_move)
            }
        }
    }
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
