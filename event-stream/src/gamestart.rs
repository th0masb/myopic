use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};

use crate::challenge_table::ChallengeTableClient;
use crate::config::AppConfig;
use crate::events::GameStart;
use crate::lichess::LichessClient;

pub struct GameStartService {
    client: LichessClient,
    invoker: LambdaInvoker,
    challenge_table: ChallengeTableClient,
}

impl GameStartService {
    pub fn new(parameters: &AppConfig) -> GameStartService {
        GameStartService {
            client: LichessClient::new(parameters.lichess_bot.auth_token.clone()),
            invoker: LambdaInvoker::new(parameters.clone()),
            challenge_table: ChallengeTableClient::new(&parameters.rate_limits.challenge_table),
        }
    }

    pub async fn process_event(&mut self, event: GameStart) -> Result<String> {
        let (game_id, opponent_id) = (event.id.as_str(), event.opponent.id.as_str());
        if self.challenge_table.set_started(opponent_id, game_id).await? {
            log::info!("Lambda for {}/{} should be invoked", opponent_id, game_id);
            match self.invoker.trigger_lambda(game_id).await {
                Err(e) => Err(anyhow!(
                    "Unable to trigger lambda: {}, abort status: {:?}",
                    e,
                    self.client.abort_game(game_id).await
                )),
                Ok(status) => match status {
                    None => Err(anyhow!(
                        "No status for lambda invocation for {}, abort status: {:?}",
                        game_id,
                        self.client.abort_game(game_id).await
                    )),
                    Some(n) => {
                        if n == 202 {
                            Ok(format!("Lambda successfully queued for game {}", game_id))
                        } else {
                            Err(anyhow!(
                                "{} status for lambda invocation for {}, abort status: {:?}",
                                n,
                                game_id,
                                self.client.abort_game(game_id).await
                            ))
                        }
                    }
                },
            }
        } else {
            Ok(format!("Lambda for {}/{} was already invoked", opponent_id, game_id))
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
        LambdaInvoker { client: LambdaClient::new(region), params }
    }

    async fn trigger_lambda(&self, game_id: &str) -> Result<Option<i64>> {
        let payload = crate::config::extract_game_lambda_payload(&self.params, game_id);
        let request = InvokeAsyncRequest {
            function_name: self.params.game_function.id.name.clone(),
            invoke_args: bytes::Bytes::from(payload),
        };
        self.client.invoke_async(request).await.map(|response| response.status).map_err(Error::from)
    }
}
