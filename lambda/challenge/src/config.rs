use serde_derive::Deserialize;

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
    pub time_limits: TimeLimits,
    pub rated: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimeLimits {
    pub limit: u32,
    pub increment: u32,
}

#[derive(Copy, Clone)]
pub enum TimeLimitType {
    Blitz,
    Bullet,
}

impl TimeLimits {
    const BLITZ_THRESHOLD: u32 = 180;

    pub fn get_type(&self) -> TimeLimitType {
        if self.limit < TimeLimits::BLITZ_THRESHOLD {
            TimeLimitType::Bullet
        } else {
            TimeLimitType::Blitz
        }
    }
}
