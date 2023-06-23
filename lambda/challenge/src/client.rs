use crate::config::{TimeLimitType, TimeLimits};
use crate::ratings::{OnlineBot, UserDetails};
use lambda_runtime::Error;
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Default)]
pub struct LichessClient {
    inner: reqwest::Client,
}

pub struct ChallengeRequest {
    pub token: String,
    pub rated: bool,
    pub time_limit: TimeLimits,
    pub target_user_id: String,
}

impl LichessClient {
    pub async fn create_challenge(&self, request: ChallengeRequest) -> Result<StatusCode, Error> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("rated", request.rated.to_string());
        params.insert("clock.limit", request.time_limit.limit.to_string());
        params.insert("clock.increment", request.time_limit.increment.to_string());
        self.inner
            .post(format!("https://lichess.org/api/challenge/{}", request.target_user_id))
            .bearer_auth(request.token.as_str())
            .form(&params)
            .send()
            .await
            .map(|r| r.status())
            .map_err(|e| {
                Error::from(format!("Error challenging {}: {}", request.target_user_id, e))
            })
    }

    pub async fn fetch_rating(
        &self,
        user_id: &str,
        time_limit_type: TimeLimitType,
    ) -> Result<u32, Error> {
        Ok(self
            .inner
            .get(format!("https://lichess.org/api/user/{}", user_id))
            .send()
            .await?
            .json::<UserDetails>()
            .await?
            .perfs
            .rating_for(time_limit_type))
    }

    pub async fn fetch_online_bots(&self) -> Result<Vec<OnlineBot>, Error> {
        Ok(self
            .inner
            .get(format!("https://lichess.org/api/bot/online"))
            .send()
            .await?
            .text()
            .await?
            .split('\n')
            .filter_map(|s| serde_json::from_str::<OnlineBot>(s).ok())
            .collect::<Vec<_>>())
    }
}
