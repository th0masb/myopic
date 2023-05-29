use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub our_user_id: String,
    pub time_limit_options: Vec<TimeLimits>,
    pub token: String,
    pub challenge_count: usize,
}

#[derive(Debug, Deserialize)]
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
