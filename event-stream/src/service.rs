use crate::events::{Challenge, ClockTimeControl, TimeControl, Variant};
use crate::params::ApplicationParameters;
use reqwest::{blocking, StatusCode};
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;

const CHALLENGE_ENDPOINT: &'static str = "https://lichess.org/api/challenge";
const STANDARD_VARIANT_KEY: &'static str = "standard";

pub(super) struct LichessService {
    parameters: ApplicationParameters,
    client: blocking::Client,
}

impl LichessService {
    pub(super) fn new(parameters: ApplicationParameters) -> LichessService {
        LichessService { parameters, client: blocking::Client::new() }
    }

    pub(super) fn process_challenge(&self, challenge: Challenge) -> Result<String, Box<dyn Error>> {
        match challenge.time_control {
            TimeControl::Unlimited | TimeControl::Correspondence { .. } => {
                log::info!("Cannot play game without real time clock...");
                self.post_challenge_decision(&challenge, "decline")
                    .map(|status| format!("{} from challenge decline", status))
            }
            TimeControl::Clock { ref clock } => {
                if self.is_legal_challenge(clock, &challenge.variant) {
                    self.post_challenge_decision(&challenge, "accept")
                        .and_then(|status| {
                            if status.is_success() {
                                log::info!("Accepted challenge for {}", &challenge.id);
                                self.trigger_lambda(&challenge.id, clock)
                            } else {
                                log::warn!(
                                    "Failed to accept challenge for {} with status ",
                                    &challenge.id
                                );
                                Err(Box::new(SimpleError { message: "".to_owned() }))
                            }
                        })
                        .map(|status| match status {
                            None => format!("None received from lambda invocation"),
                            Some(n) => format!("{} received from lambda invocation", n),
                        })
                } else {
                    log::info!("Illegal challenge: {:?} {:?}", challenge.variant, clock);
                    self.post_challenge_decision(&challenge, "decline")
                        .map(|status| format!("{} from challenge decline", status))
                }
            }
        }
    }

    fn trigger_lambda(
        &self,
        game_id: &String,
        time_control: &ClockTimeControl,
    ) -> Result<Option<i64>, Box<dyn Error>> {
        let max_depth = self.compute_max_depth(&time_control);
        let region = Region::from_str(self.parameters.function_region.as_str())?;
        let payload = self.parameters.to_lambda_invocation_payload(game_id.clone(), max_depth)?;

        tokio::runtime::Runtime::new()?
            .block_on(LambdaClient::new(region).invoke_async(InvokeAsyncRequest {
                function_name: self.parameters.function_name.clone(),
                invoke_args: bytes::Bytes::from(payload),
            }))
            .map(|response| response.status)
            .map_err(|error| Box::new(error) as Box<dyn Error>)
    }

    fn post_challenge_decision(
        &self,
        challenge: &Challenge,
        decision: &str,
    ) -> Result<StatusCode, Box<dyn Error>> {
        self.client
            .post(format!("{}/{}/{}", CHALLENGE_ENDPOINT, challenge.id, decision).as_str())
            .bearer_auth(&self.parameters.lichess_auth_token)
            .send()
            .map(|response| response.status())
            .map_err(|error| Box::new(error) as Box<dyn Error>)
    }

    fn compute_max_depth(&self, time_control: &ClockTimeControl) -> u8 {
        let p = &self.parameters;
        let max_lambda_execution_time_secs = p.max_lambda_duration_mins as u32 * 60;
        let increment_allowance_secs = p.increment_allowance_mins as u32 * 60;
        let total_limit = 2 * time_control.limit;
        (1 + (total_limit + increment_allowance_secs) / max_lambda_execution_time_secs) as u8
    }

    fn is_legal_challenge(&self, time_control: &ClockTimeControl, variant: &Variant) -> bool {
        self.is_legal_time_control(time_control) && variant.key.as_str() == STANDARD_VARIANT_KEY
    }

    fn is_legal_time_control(&self, control: &ClockTimeControl) -> bool {
        let p = &self.parameters;
        p.min_initial_time_secs <= control.limit
            && control.limit <= p.max_initial_time_secs
            && p.min_increment_secs <= control.increment
            && control.increment <= p.max_increment_secs
    }
}

#[derive(Debug, Clone)]
struct SimpleError {
    message: String,
}

impl std::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Error[{}]", self.message).as_str())
    }
}

impl std::error::Error for SimpleError {}
