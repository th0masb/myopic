use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

use lambda_payloads::chessgame::PlayGameEvent;

const LICHESS_AUTH_TOKEN_VAR: &'static str = "LICHESS_AUTH_TOKEN";
const CONFIG_VAR: &'static str = "APP_CONFIG";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "gameFunction")]
    pub game_function: GameFunctionConfig,
    #[serde(rename = "moveFunction")]
    pub move_function: AwsResourceId,
    #[serde(rename = "lichessBot")]
    pub lichess_bot: LichessConfig,
    #[serde(rename = "timeConstraints", default)]
    pub time_constraints: TimeConstraints,
    #[serde(rename = "eventLoop", default)]
    pub event_loop: EventLoopConfig,
    #[serde(rename = "challengeServerAddress", default = "default_server_address")]
    pub challenge_server_address: String,
    #[serde(rename = "rateLimits")]
    pub rate_limits: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LichessConfig {
    #[serde(rename = "botId")]
    pub bot_id: String,
    #[serde(rename = "authToken", default = "get_lichess_auth_token")]
    pub auth_token: String,
    #[serde(rename = "userMatchers", default = "default_user_matchers")]
    pub user_matchers: Vec<StringMatcher>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLoopConfig {
    #[serde(rename = "retryWaitDurationSecs")]
    pub retry_wait_duration_secs: u32,
    #[serde(rename = "statusPollGapSecs")]
    pub status_poll_gap_secs: u32,
    #[serde(rename = "maxStreamLifeMins")]
    pub max_stream_life_mins: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFunctionConfig {
    pub id: AwsResourceId,
    #[serde(rename = "abortAfterSecs")]
    pub abort_after_secs: u8,
    #[serde(rename = "maxRecursionDepth")]
    pub max_recursion_depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsResourceId {
    pub name: String,
    #[serde(default = "default_region")]
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraints {
    #[serde(rename = "minInitialTimeSecs")]
    pub min_initial_time_secs: u32,
    #[serde(rename = "maxInitialTimeSecs")]
    pub max_initial_time_secs: u32,
    #[serde(rename = "minIncrementSecs")]
    pub min_increment_secs: u32,
    #[serde(rename = "maxIncrementSecs")]
    pub max_increment_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringMatcher {
    pub include: bool,
    #[serde(with = "serde_regex")]
    pub pattern: Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    #[serde(rename = "challengeTable")]
    pub challenge_table: AwsResourceId,
    #[serde(rename = "maxDailyChallenges")]
    pub max_daily_challenges: usize,
    #[serde(rename = "maxDailyUserChallenges")]
    pub max_daily_user_challenges: usize,
    #[serde(default)]
    pub excluded: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let config = get_env_var(CONFIG_VAR);
        serde_json::from_str(config.as_str())
            .or_else(|_e| {
                std::fs::read_to_string(config.as_str()).map_err(anyhow::Error::from).and_then(
                    |s| serde_json::from_str::<AppConfig>(s.as_str()).map_err(anyhow::Error::from),
                )
            })
            .expect(format!("Could not parse config from {}", config).as_str())
    }
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        EventLoopConfig {
            retry_wait_duration_secs: 30,
            status_poll_gap_secs: 60,
            max_stream_life_mins: 300,
        }
    }
}

impl EventLoopConfig {
    pub fn max_stream_life(&self) -> Duration {
        Duration::from_secs((self.max_stream_life_mins * 60) as u64)
    }

    pub fn stream_retry_wait(&self) -> Duration {
        Duration::from_secs(self.retry_wait_duration_secs as u64)
    }

    pub fn status_pool_gap(&self) -> Duration {
        Duration::from_secs(self.status_poll_gap_secs as u64)
    }
}

impl Default for TimeConstraints {
    fn default() -> Self {
        TimeConstraints {
            min_initial_time_secs: 60,
            max_initial_time_secs: 600,
            min_increment_secs: 0,
            max_increment_secs: 5,
        }
    }
}

fn get_lichess_auth_token() -> String {
    get_env_var(LICHESS_AUTH_TOKEN_VAR)
}

fn default_server_address() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_region() -> String {
    "eu-west-2".to_string()
}

fn default_user_matchers() -> Vec<StringMatcher> {
    vec![StringMatcher { include: true, pattern: Regex::new(r".*").unwrap() }]
}

fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(format!("Could not find env var \"{}\"", key).as_str())
}

pub fn extract_game_lambda_payload(config: &AppConfig, game_id: &str) -> String {
    serde_json::to_string(&PlayGameEvent {
        move_function_name: config.move_function.name.clone(),
        move_function_region: config.move_function.region.clone(),
        lichess_game_id: game_id.to_string(),
        lichess_auth_token: config.lichess_bot.auth_token.clone(),
        lichess_bot_id: config.lichess_bot.bot_id.clone(),
        abort_after_secs: config.game_function.abort_after_secs,
        max_depth: config.game_function.max_recursion_depth,
        current_depth: 0,
    })
    .unwrap()
}
