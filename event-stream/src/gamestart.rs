use std::str::FromStr;

use crate::config::AppConfig;
use anyhow::{anyhow, Error, Result};
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};

use crate::events::GameStart;
use crate::lichess::LichessClient;

pub struct GameStartService {
    client: LichessClient,
    invoker: LambdaInvoker,
}

impl GameStartService {
    pub fn new(parameters: &AppConfig) -> GameStartService {
        GameStartService {
            client: LichessClient::new(parameters.lichess_bot.auth_token.clone()),
            invoker: LambdaInvoker::new(parameters.clone()),
        }
    }

    pub async fn process_gamestart(&self, game_start: GameStart) -> Result<String> {
        let id = game_start.id.as_str();
        //let clock = self.client.get_clock(id).await?;
        match self.invoker.trigger_lambda(id).await {
            Err(e) => Err(anyhow!(
                "Unable to trigger lambda: {}, abort status: {:?}",
                e,
                self.client.abort_game(id).await
            )),
            Ok(status) => match status {
                None => Err(anyhow!(
                    "No status for lambda invocation for {}, abort status: {:?}",
                    id,
                    self.client.abort_game(id).await
                )),
                Some(n) => {
                    if n == 202 {
                        Ok(format!("Lambda successfully queued for game {}", id))
                    } else {
                        Err(anyhow!(
                            "{} status for lambda invocation for {}, abort status: {:?}",
                            n,
                            id,
                            self.client.abort_game(id).await
                        ))
                    }
                }
            },
        }
    }
}

struct LambdaInvoker {
    params: AppConfig,
    client: LambdaClient,
}

impl LambdaInvoker {
    fn new(params: AppConfig) -> LambdaInvoker {
        let region = Region::from_str(params.game_function.id.region.as_str()).unwrap();
        LambdaInvoker {
            client: LambdaClient::new(region),
            params,
        }
    }

    async fn trigger_lambda(&self, game_id: &str) -> Result<Option<i64>> {
        let payload = crate::config::extract_game_lambda_payload(&self.params, game_id);
        let request = InvokeAsyncRequest {
            function_name: self.params.game_function.id.name.clone(),
            invoke_args: bytes::Bytes::from(payload),
        };
        self.client
            .invoke_async(request)
            .await
            .map(|response| response.status)
            .map_err(Error::from)
    }
}
