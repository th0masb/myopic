use async_trait::async_trait;
use clap::Parser;
use lichess_api::{LichessClient, LichessEndgameClient};
use lichess_events::events::{Challenge, GameStart};
use lichess_events::{EventProcessor, LichessEvent, StreamParams};
use lichess_game::{EmptyCancellationHook, Metadata};
use myopic_brain::Engine;
use openings::{DynamoOpeningService, OpeningTable};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use simple_logger::SimpleLogger;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;
use lichess_api::ratings::{ChallengeRequest, OnlineBot, TimeLimits, TimeLimitType};
use anyhow::{anyhow, Result};
use rand::prelude::SliceRandom;
use rand::thread_rng;

const TABLE_SIZE: usize = 1_000_000;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    auth_token: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameStarted {
    id: String,
    opponent_id: String,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    let args = Args::parse();
    let client = LichessClient::new(args.auth_token.clone());
    let bot_id = client.get_our_profile().await.expect("").id;
    log::info!("Our id is \"{}\"", bot_id.as_str());
    let cloned_id = bot_id.clone();
    let cloned_token = args.auth_token.clone();
    let (tx, rx) = tokio::sync::mpsc::channel::<GameStarted>(32);
    tokio::spawn(async move { run_event_stream(cloned_token, cloned_id, tx).await });
    search_for_game(
        args.auth_token,
        bot_id.clone(),
        RatingRange { offset_below: 200, offset_above: 100 },
        2,
        TimeLimits { limit: 120, increment: 1 },
        rx
    ).await;
}

#[derive(Debug, Clone, Default)]
struct BotTracker {
    activity: HashMap<String, i32>,
}

#[derive(Debug, Clone, Default)]
struct RatingRange {
    offset_below: u32,
    offset_above: u32,
}

async fn search_for_game(
    auth_token: String,
    bot_id: String,
    rating_range: RatingRange,
    max_concurrent_games: usize,
    time_limit: TimeLimits,
    mut rx: Receiver<GameStarted>,
) {
    let client = LichessClient::new(auth_token);
    let mut poll_interval = tokio::time::interval(Duration::from_secs(45));
    let mut flush_interval = tokio::time::interval(Duration::from_secs(3600));
    let mut tracker = BotTracker::default();
    loop {
        tokio::select! {
            _ = flush_interval.tick() => {
                log::info!("Flushing bot tracker");
                tracker.activity.clear()
            }
            Some(game_id) = rx.recv() => {
                *tracker.activity.entry(game_id.opponent_id).or_insert(1) -= 1;
            }
            _ = poll_interval.tick() => {
                if let Err(e) = execute_challenge_poll(
                    &mut tracker,
                    bot_id.as_str(),
                    &client,
                    &rating_range,
                    time_limit.clone(),
                ).await {
                    log::error!("Error in challenge poll: {}", e);
                    sleep(Duration::from_secs(120)).await;
                };
            }
        }
    }
}

async fn execute_challenge_poll(
    tracker: &mut BotTracker,
    bot_id: &str,
    client: &LichessClient,
    rating_range: &RatingRange,
    time_limit: TimeLimits,
) -> Result<()> {
    let exclusions = vec!["hyperopic", "myopic-bot"];
    let time_limit_type = time_limit.get_type();
    let BotState { rating, online_bots, games_in_progress } =
        fetch_bot_state(bot_id, time_limit_type, client)
            .await
            .map_err(|e| anyhow!("Failed to fetch bot state: {}", e))?;

    if games_in_progress >= 2 {
        return Ok(());
    } else if !online_bots.iter().any(|b| b.id.as_str() == bot_id) {
        log::warn!("It does not appear that we are online!");
        return Ok(());
    }

    let min_rating = rating - rating_range.offset_below;
    let max_rating = rating + rating_range.offset_above;
    let ratings = min_rating..=max_rating;
    let candidate_bots: Vec<_> = online_bots.into_iter()
        .filter(|b| !exclusions.contains(&b.id.as_str()))
        .filter(|b| ratings.contains(&b.perfs.rating_for(time_limit_type)))
        .collect();
    log::info!("{} candidate opponents", candidate_bots.len());
    let (tested, untested): (Vec<_>, Vec<_>) =
        candidate_bots.into_iter().partition(|b| tracker.activity.contains_key(&b.id));
    log::info!("{} tested, {} untested", tested.len(), untested.len());
    let (active, inactive): (Vec<_>, Vec<_>) =
        tested.clone().into_iter().partition(|b| tracker.activity[&b.id] == 0);
    log::info!("{} active, {} inactive", active.len(), inactive.len());

    let chosen = if !untested.is_empty() {
        untested.iter()
            .max_by_key(|b| b.perfs.rating_for(time_limit_type))
            .unwrap().clone()
    } else if !active.is_empty() {
        active.choose(&mut thread_rng()).unwrap().clone()
    } else {
        inactive.into_iter().min_by_key(|b| tracker.activity[&b.id]).unwrap()
    };

    log::info!("Chose opponent: {}", chosen.id.as_str());

    let request = ChallengeRequest {
        rated: true,
        time_limit,
        target_user_id: chosen.id.clone(),
    };

    let _ = client.create_challenge(request).await
        .map_err(|e| anyhow!("Failed to create challenge {}", e))
        .and_then(|status| {
            if status.is_success() {
                Ok(())
            } else {
                Err(anyhow!("Error status {} for challenge creation", status))
            }
        })?;

    *tracker.activity.entry(chosen.id).or_insert(0) += 1;
    Ok(())
}

async fn fetch_bot_state(
    bot_id: &str,
    time_limit_type: TimeLimitType,
    client: &LichessClient
) -> Result<BotState> {
    Ok(BotState {
        rating: client.fetch_rating(bot_id, time_limit_type).await?,
        online_bots: client.fetch_online_bots().await?,
        games_in_progress: client.get_our_live_games().await?.now_playing.len(),
    })
}

struct BotState {
    pub rating: u32,
    pub online_bots: Vec<OnlineBot>,
    pub games_in_progress: usize,
}

async fn run_event_stream(auth_token: String, bot_id: String, tx: Sender<GameStarted>) {
    lichess_events::stream(
        StreamParams {
            status_poll_frequency: Duration::from_secs(300),
            max_lifespan: Duration::from_secs(120 * 60 * 60),
            retry_wait: Duration::from_secs(10),
            our_bot_id: bot_id.clone(),
            auth_token: auth_token.clone(),
        },
        EventProcessorImpl {
            our_bot_id: bot_id.clone(),
            auth_token: auth_token.clone(),
            lichess: LichessClient::new(auth_token.clone()),
            games_started: Default::default(),
            table_size: TABLE_SIZE,
            tx,
        }
    ).await;
}

fn opening_table() -> DynamoOpeningService {
    OpeningTable {
        name: "MyopicOpenings".to_string(),
        region: "eu-west-2".to_string(),
        position_key: "PositionFEN".to_string(),
        move_key: "Moves".to_string(),
        max_depth: 10,
    }
    .try_into()
    .expect("Bad opening table config")
}

struct EventProcessorImpl {
    our_bot_id: String,
    auth_token: String,
    lichess: LichessClient,
    games_started: HashSet<String>,
    table_size: usize,
    tx: Sender<GameStarted>,
}

#[async_trait]
impl EventProcessor for EventProcessorImpl {
    async fn process(&mut self, event: LichessEvent) {
        match event {
            // Decline incoming challenges for now
            LichessEvent::Challenge { challenge: Challenge { id, challenger, .. } } => {
                if challenger.id != self.our_bot_id {
                    log::info!("Declining challenge from {}", challenger.id);
                    self.lichess.post_challenge_response(id.as_str(), "decline").await.ok();
                }
            }
            // Span a new task to play the game if we haven't already done so
            LichessEvent::GameStart { game: GameStart { id, opponent } } => {
                if self.games_started.insert(id.clone()) {
                    let metadata = Metadata {
                        game_id: id,
                        our_bot_id: self.our_bot_id.clone(),
                        auth_token: self.auth_token.clone(),
                    };
                    let engine = Engine::new(
                        self.table_size,
                        vec![Box::new(opening_table()), Box::new(LichessEndgameClient::default())],
                    );
                    self.tx.send(
                        GameStarted {
                            id: metadata.game_id.clone(),
                            opponent_id: opponent.id.clone(),
                        }
                    ).await.ok();
                    tokio::spawn(async move {
                        let game_id = metadata.game_id.clone();
                        log::info!("Starting game {}", game_id);
                        lichess_game::play(Duration::MAX, engine, metadata, EmptyCancellationHook)
                            .await
                            .map_err(|e| {
                                log::error!("Game id {} failed: {}", game_id, e);
                            })
                            .ok();
                    });
                }
            }
        }
    }
}
