use lambda_runtime::{service_fn, Error, LambdaEvent};
use rand::prelude::SliceRandom;
use simple_logger::SimpleLogger;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub our_user_id: String,
    pub time_limit_options: Vec<TimeLimits>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeLimits {
    pub limit: u32,
    pub increment: u32,
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

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub perfs: UserDetailsPerfs
}

#[derive(Debug, Deserialize)]
pub struct UserDetailsPerfs {
    pub blitz: UserDetailsGamePerf,
    pub bullet: UserDetailsGamePerf,
}

#[derive(Debug, Deserialize)]
pub struct UserDetailsGamePerf {
    pub rating: u32
}

pub struct OnlineBot {

}

const APP_CONFIG_VAR: &str = "APP_CONFIG";

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    let config: AppConfig = serde_json::from_str(std::env::var(APP_CONFIG_VAR)?.as_str())?;

    let client = reqwest::Client::new();

    let our_ratings = client
        .get(format!("https://lichess.org/api/user/{}", config.our_user_id))
        .send()
        .await?
        .json::<UserDetails>()
        .await?;

    let mut rng = rand::thread_rng();
    let chosen_time_limit = config.time_limit_options.choose(&mut rng)
        .expect("No time limit options given!");
    let rating = match chosen_time_limit.get_type() {
        TimeLimitType::Bullet => our_ratings.perfs.bullet.rating,
        TimeLimitType::Blitz => our_ratings.perfs.blitz.rating,
    };

    let bots = client
        .get(format!("https://lichess.org/api/bot/online"))
        .send()
        .await?
        .text()
        .await?
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();


    println!("{:?}", bots.len());

    //lambda_runtime::run(service_fn(game_handler)).await?;
    Ok(())
}

pub enum TimeLimitType {
    Blitz,
    Bullet,
}



async fn game_handler(event: LambdaEvent<()>) -> Result<(), Error> {
    Ok(())
}