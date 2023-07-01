use async_trait::async_trait;
use clap::Parser;
use lichess_api::{LichessClient, LichessEndgameClient};
use lichess_events::events::{Challenge, GameStart};
use lichess_events::{EventProcessor, LichessEvent, StreamParams};
use lichess_game::{EmptyCancellationHook, Metadata, MoveChooser};
use myopic_brain::{Board, ComputeMoveInput, Engine};
use openings::{DynamoOpeningService, OpeningTable};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::time::Duration;
use simple_logger::SimpleLogger;
use tokio::{join, try_join};
use tokio::time::Instant;

const TABLE_SIZE: usize = 1_000_000;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    auth_token: String,
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
    tokio::spawn(async move { run_event_stream(cloned_token, cloned_id) });
    search_for_game(args.auth_token, 1500..2000, 2).await;
}

struct BotTracker {
    accepting: HashSet<String>,
    rejecting: HashSet<String>,
    last_flush: Instant,
}

// Within a predefined rating range:
// Define accepting/rejecting sets:
//  if exists online bot in range not in either try it, add to corresponding set
//  if exists offline bot or bot out of range in either remove it
//  otherwise choose randomly from the accepting set, if doesn't accept move to rejecting
//  flush rejecting set after n hours
//  Prefer higher rated bots
async fn search_for_game(
    auth_token: String,
    rating_range: Range<usize>,
    max_concurrent_games: usize,
) {
    let client = LichessClient::new(auth_token);
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;


    }
}

async fn run_event_stream(auth_token: String, bot_id: String) {
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
            LichessEvent::GameStart { game: GameStart { id, .. } } => {
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
