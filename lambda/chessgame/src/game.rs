use std::cmp::max;

use lambda_payloads::chessmove::{ChooseMoveEvent, ChooseMoveEventClock};
use reqwest::StatusCode;
use rusoto_core::Region;
use tokio_util::sync::CancellationToken;

use myopic_brain::anyhow::{anyhow, Result};
use myopic_brain::{ChessBoard, EvalBoard, Side};

use crate::events::{ChatLine, Clock, GameEvent, GameFull, GameState};
use crate::lichess::{LichessChatRoom, LichessService};
use crate::{messages, MoveLambdaClient};

const STARTED_STATUS: &'static str = "started";
const CREATED_STATUS: &'static str = "created";
const MOVE_LATENCY_MS: u64 = 200;
const MIN_COMPUTE_TIME_MS: u64 = 200;

#[derive(Debug, Clone, Eq, PartialEq)]
struct InferredGameMetadata {
    lambda_side: Side,
    clock: Clock,
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub game_id: String,
    pub bot_name: String,
    pub lichess_auth_token: String,
    pub move_region: Region,
    pub move_function_name: String,
    pub cancel_token: CancellationToken,
}

pub struct Game {
    bot_name: String,
    inferred_metadata: Option<InferredGameMetadata>,
    lichess_service: LichessService,
    move_client: MoveLambdaClient,
    halfmove_count: usize,
    cancel_token: CancellationToken,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Cancelled,
}

impl From<GameConfig> for Game {
    fn from(conf: GameConfig) -> Self {
        Game {
            lichess_service: LichessService::new(conf.lichess_auth_token, conf.game_id),
            move_client: (conf.move_region, conf.move_function_name).into(),
            bot_name: conf.bot_name,
            inferred_metadata: None,
            halfmove_count: 0,
            cancel_token: conf.cancel_token,
        }
    }
}

impl Game {
    pub fn halfmove_count(&self) -> usize {
        self.halfmove_count
    }

    pub async fn abort(&self) -> Result<StatusCode> {
        self.lichess_service.abort().await
    }

    pub async fn post_introduction(&self) {
        for chatline in vec![
            messages::INTRO_1,
            messages::INTRO_2,
            messages::INTRO_3,
            messages::INTRO_4,
        ] {
            self.post_chat(chatline, LichessChatRoom::Player).await;
            self.post_chat(chatline, LichessChatRoom::Spectator).await;
        }
    }

    async fn post_chat(&self, text: &str, room: LichessChatRoom) {
        match self.lichess_service.post_chatline(text, room).await {
            Err(err) => {
                log::warn!("Failed to post chatline {} in {:?}: {}", text, room, err)
            }
            Ok(status) => {
                log::info!(
                    "Response status {} for chatline {} in room {:?}",
                    status,
                    text,
                    room
                )
            }
        }
    }

    pub async fn process_event(&mut self, event_json: &str) -> Result<GameExecutionState> {
        match serde_json::from_str(event_json) {
            Err(error) => {
                log::warn!("Error parsing event {}", error);
                Err(anyhow!("{}", error))
            }
            Ok(event) => match event {
                GameEvent::ChatLine { content } => self.process_chat(content),
                GameEvent::GameFull { content } => self.process_game(content).await,
                GameEvent::State { content } => self.process_state(content).await,
            },
        }
    }

    fn process_chat(&self, _chat_line: ChatLine) -> Result<GameExecutionState> {
        // Do nothing for now
        Ok(GameExecutionState::Running)
    }

    async fn process_game(&mut self, game: GameFull) -> Result<GameExecutionState> {
        if game.initial_fen.as_str() != "startpos" {
            return Err(anyhow!("Custom start positions not currently supported"));
        }
        // Track info required for playing future gamestates
        self.inferred_metadata = Some(InferredGameMetadata {
            clock: game.clock,
            lambda_side: if self.bot_name == game.white.name {
                log::info!("Detected lambda is playing as white");
                Side::White
            } else if self.bot_name == game.black.name {
                log::info!("Detected lambda is playing as black");
                Side::Black
            } else {
                return Err(anyhow!(
                    "Name not matched, us: {} w: {} b: {}",
                    self.bot_name,
                    game.white.name,
                    game.black.name
                ));
            },
        });
        self.process_state(game.state).await
    }

    fn get_game_state(&self, moves: &str) -> Result<(Side, u32)> {
        let mut state = EvalBoard::default();
        state.play_uci(moves)?;
        let pos_count = state.position_count();
        Ok((state.active(), pos_count as u32))
    }

    async fn process_state(&mut self, state: GameState) -> Result<GameExecutionState> {
        log::info!("Parsing previous game moves: {}", state.moves);
        let (active_side, n_moves) = self.get_game_state(state.moves.as_str())?;
        self.halfmove_count = n_moves as usize;
        match state.status.as_str() {
            STARTED_STATUS | CREATED_STATUS => {
                let metadata = self.get_latest_metadata()?.clone();
                if active_side != metadata.lambda_side {
                    log::info!("It is not our turn, waiting for opponents move");
                    Ok(GameExecutionState::Running)
                } else {
                    let (remaining, increment) = match metadata.lambda_side {
                        Side::White => (state.wtime, state.winc),
                        Side::Black => (state.btime, state.binc),
                    };
                    tokio::select! {
                        _ = self.cancel_token.cancelled() => {
                            log::info!("Move selection cancelled!");
                            Ok(GameExecutionState::Cancelled)
                        },
                        computed_move_result = self
                        .move_client
                        .compute_move(ChooseMoveEvent {
                            moves_played: state.moves.clone(),
                            clock_millis: ChooseMoveEventClock {
                                increment,
                                // Take into account the network latency for calling the lambda
                                remaining: max(MIN_COMPUTE_TIME_MS, remaining - MOVE_LATENCY_MS),
                            },
                            features: vec![],
                        }) => {
                            let computed_move = computed_move_result?;
                            self.lichess_service.post_move(computed_move).await?;
                            Ok(GameExecutionState::Running)
                        }
                    }
                }
            }
            // All other possibilities indicate the game is over
            status => {
                log::info!("Game finished with status: {}!", status);
                Ok(GameExecutionState::Finished)
            }
        }
    }

    fn get_latest_metadata(&self) -> Result<&InferredGameMetadata> {
        self.inferred_metadata
            .as_ref()
            .ok_or(anyhow!("Metadata not initialized"))
    }
}
