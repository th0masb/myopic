use serde_derive::Deserialize;
use lichess_api::ratings::TimeLimits;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    #[serde(rename = "ourUserId")]
    pub our_user_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ChallengeEvent {
    Specific { challenges: Vec<KnownUserChallenge> },
    Random { time_limit_options: Vec<TimeLimits>, challenge_count: usize, rated: bool },
}

#[derive(Debug, Deserialize)]
pub struct KnownUserChallenge {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "timeLimits")]
    pub time_limits: TimeLimits,
    pub rated: bool,
    pub repeat: usize,
}

