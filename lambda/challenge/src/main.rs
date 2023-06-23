use crate::config::AppConfig;
use crate::ratings::{OnlineBot, UserDetails};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rand::prelude::SliceRandom;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

mod config;
mod ratings;

const APP_CONFIG_VAR: &str = "APP_CONFIG";

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).without_timestamps().init()?;
    lambda_runtime::run(service_fn(game_handler)).await
}

async fn game_handler(_event: LambdaEvent<()>) -> Result<(), Error> {
    let config: AppConfig = serde_json::from_str(std::env::var(APP_CONFIG_VAR)?.as_str())?;

    let chosen_time_limit = config
        .time_limit_options
        .choose(&mut rand::thread_rng())
        .expect("No time limit options given!");

    let client = reqwest::Client::new();

    let our_rating = client
        .get(format!("https://lichess.org/api/user/{}", config.our_user_id))
        .send()
        .await?
        .json::<UserDetails>()
        .await?
        .perfs
        .rating_for(chosen_time_limit.get_type());

    let mut bots = client
        .get(format!("https://lichess.org/api/bot/online"))
        .send()
        .await?
        .text()
        .await?
        .split('\n')
        .filter_map(|s| serde_json::from_str::<OnlineBot>(s).ok())
        .filter(|b| b.id != config.our_user_id)
        .collect::<Vec<_>>();

    bots.sort_by_key(|bot| bot.perfs.rating_for(chosen_time_limit.get_type()));

    let lower_count = (config.challenge_count * 3) / 4;
    let upper_count = config.challenge_count - lower_count;

    let mut opponents = bots
        .iter()
        .filter(|b| b.perfs.rating_for(chosen_time_limit.get_type()) <= our_rating)
        .rev()
        .take(lower_count)
        .collect::<Vec<_>>();

    opponents.extend(
        bots.iter()
            .filter(|b| b.perfs.rating_for(chosen_time_limit.get_type()) > our_rating)
            .take(upper_count),
    );

    for opponent in opponents {
        log::info!("Creating challenge for {}", opponent.id);
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("rated", "true".to_owned());
        params.insert("clock.limit", chosen_time_limit.limit.to_string());
        params.insert("clock.increment", chosen_time_limit.increment.to_string());
        client
            .post(format!("https://lichess.org/api/challenge/{}", opponent.id))
            .bearer_auth(config.token.as_str())
            .form(&params)
            .send()
            .await
            .map(|r| log::info!("Challenge response {} for {}", r.status(), opponent.id))
            .map_err(|e| log::error!("Error challenging {}: {}", opponent.id, e))
            .ok();
    }
    Ok(())
}
