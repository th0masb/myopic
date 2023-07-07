use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct ChallengeRequest {
    pub rated: bool,
    pub time_limit: TimeLimits,
    pub target_user_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDetails {
    pub perfs: UserDetailsPerfs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDetailsPerfs {
    pub blitz: Option<UserDetailsGamePerf>,
    pub bullet: Option<UserDetailsGamePerf>,
    pub rapid: Option<UserDetailsGamePerf>,
    #[serde(rename = "ultraBullet")]
    pub ultra_bullet: Option<UserDetailsGamePerf>,
    pub classical: Option<UserDetailsGamePerf>,
}

impl UserDetailsPerfs {
    pub fn rating_for(&self, time_limit_type: TimeLimitType) -> Option<UserDetailsGamePerf> {
        match time_limit_type {
            TimeLimitType::Bullet => self.bullet.clone(),
            TimeLimitType::Blitz => self.blitz.clone(),
            TimeLimitType::Rapid => self.rapid.clone(),
            TimeLimitType::UltraBullet => self.ultra_bullet.clone(),
            TimeLimitType::Classical => self.classical.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDetailsGamePerf {
    pub rating: u32,
    pub prov: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OnlineBot {
    pub id: String,
    #[serde(rename = "tosViolation")]
    pub tos_violation: Option<bool>,
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
    Rapid,
    UltraBullet,
    Classical,
}

impl TimeLimits {
    pub fn get_type(&self) -> TimeLimitType {
        // https://lichess.org/forum/lichess-feedback/why-10-minute-game-is-rapid
        match self.increment * 40 + self.limit {
            0..=29 => TimeLimitType::UltraBullet,
            30..=179 => TimeLimitType::Bullet,
            180..=479 => TimeLimitType::Blitz,
            480..=1499 => TimeLimitType::Rapid,
            _ => TimeLimitType::Classical,
        }
    }
}
