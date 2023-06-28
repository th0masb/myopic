use crate::config::TimeLimitType;
use serde_derive::Deserialize;

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
