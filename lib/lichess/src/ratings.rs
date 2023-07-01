use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct ChallengeRequest {
    pub token: String,
    pub rated: bool,
    pub time_limit: TimeLimits,
    pub target_user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub perfs: UserDetailsPerfs,
}

#[derive(Debug, Deserialize)]
pub struct UserDetailsPerfs {
    pub blitz: UserDetailsGamePerf,
    pub bullet: UserDetailsGamePerf,
}

impl UserDetailsPerfs {
    pub fn rating_for(&self, time_limit_type: TimeLimitType) -> u32 {
        match time_limit_type {
            TimeLimitType::Bullet => self.bullet.rating,
            TimeLimitType::Blitz => self.blitz.rating,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserDetailsGamePerf {
    pub rating: u32,
}

#[derive(Debug, Deserialize)]
pub struct OnlineBot {
    pub id: String,
    pub perfs: UserDetailsPerfs,
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
