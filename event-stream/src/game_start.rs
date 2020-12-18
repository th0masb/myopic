use crate::events::{ClockTimeControl, GameStart};
use crate::lichess::LichessClient;
use crate::params::ApplicationParameters;
use anyhow::{anyhow, Error, Result};
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use std::str::FromStr;

pub struct GameStartService {
    client: LichessClient,
    invoker: LambdaInvoker,
}
impl GameStartService {
    pub fn new(parameters: &ApplicationParameters) -> GameStartService {
        GameStartService {
            client: LichessClient::new(parameters.lichess_auth_token.clone()),
            invoker: LambdaInvoker {
                parameters: parameters.clone(),
            },
        }
    }

    pub fn process_gamestart(&self, game_start: GameStart) -> Result<String> {
        let id = game_start.id.as_str();
        match self
            .client
            .get_clock(id)
            .and_then(|clock| self.invoker.trigger_lambda(id, &clock))
        {
            Err(e) => Err(anyhow!(
                "Unable to trigger lambda: {}, abort status: {:?}",
                e,
                self.client.abort_game(id)
            )),
            Ok(status) => match status {
                None => Err(anyhow!(
                    "No status for lambda invocation for {}, abort status: {:?}",
                    id,
                    self.client.abort_game(id)
                )),
                Some(n) => {
                    if n == 202 {
                        Ok(format!("Lambda successfully queued for game {}", id))
                    } else {
                        Err(anyhow!(
                            "{} status for lambda invocation for {}, abort status: {:?}",
                            n,
                            id,
                            self.client.abort_game(id)
                        ))
                    }
                }
            },
        }
    }
}

struct LambdaInvoker {
    parameters: ApplicationParameters,
}
impl LambdaInvoker {
    fn trigger_lambda(
        &self,
        game_id: &str,
        time_control: &ClockTimeControl,
    ) -> Result<Option<i64>> {
        let max_depth = self.compute_max_depth(&time_control);
        let region = Region::from_str(self.parameters.function_region.as_str())?;
        let payload = self
            .parameters
            .to_lambda_invocation_payload(game_id.to_string(), max_depth)?;

        tokio::runtime::Runtime::new()?
            .block_on(LambdaClient::new(region).invoke_async(InvokeAsyncRequest {
                function_name: self.parameters.function_name.clone(),
                invoke_args: bytes::Bytes::from(payload),
            }))
            .map(|response| response.status)
            .map_err(Error::from)
    }

    fn compute_max_depth(&self, time_control: &ClockTimeControl) -> u8 {
        let p = &self.parameters;
        let max_lambda_execution_time_secs = p.max_lambda_duration_mins as u32 * 60;
        let increment_allowance_secs = p.increment_allowance_mins as u32 * 60;
        let total_limit = 2 * time_control.limit;
        (1 + (total_limit + increment_allowance_secs) / max_lambda_execution_time_secs) as u8
    }
}
