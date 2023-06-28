use std::cmp::max;
use std::time::Duration;

use reqwest::StatusCode;
use tokio_util::sync::CancellationToken;
use lichess_api::LichessChatRoom;

use myopic_brain::anyhow::{anyhow, Result};
use myopic_brain::{Board, Side};

use crate::compute::MoveChooser;
use crate::events::{Clock, GameEvent, GameFull, GameState};
use crate::lichess::LichessService;
use crate::messages;

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
pub struct GameConfig<M: MoveChooser> {
    pub game_id: String,
    pub bot_name: String,
    pub auth_token: String,
    pub moves: M,
    pub cancel_token: CancellationToken,
}

pub struct Game<M: MoveChooser> {
    bot_name: String,
    inferred_metadata: Option<InferredGameMetadata>,
    lichess: LichessService,
    moves: M,
    halfmove_count: usize,
    cancel_token: CancellationToken,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameExecutionState {
    Running,
    Finished,
    Cancelled,
}

impl<M: MoveChooser> From<GameConfig<M>> for Game<M> {
    fn from(conf: GameConfig<M>) -> Self {
        Game {
            lichess: LichessService::new(conf.auth_token, conf.game_id),
            moves: conf.moves,
            bot_name: conf.bot_name,
            inferred_metadata: None,
            halfmove_count: 0,
            cancel_token: conf.cancel_token,
        }
    }
}

impl<M: MoveChooser> Game<M> {
    pub fn halfmove_count(&self) -> usize {
        self.halfmove_count
    }

    pub async fn abort(&self) -> Result<StatusCode> {
        self.lichess.client.abort_game(self.lichess.game_id.as_str()).await
    }

    pub async fn post_introduction(&self) {
        self.post_chat(messages::INTRO, LichessChatRoom::Player).await;
        self.post_chat(messages::INTRO, LichessChatRoom::Spectator).await;
    }

    async fn post_chat(&self, text: &str, room: LichessChatRoom) {
        match self.lichess.client.post_chatline(self.lichess.game_id.as_str(), text, room).await {
            Err(err) => {
                log::warn!("Failed to post chatline {} in {:?}: {}", text, room, err)
            }
            Ok(status) => {
                log::info!("Response status {} for chatline {} in room {:?}", status, text, room)
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
                GameEvent::GameFull { content } => self.process_game(content).await,
                GameEvent::State { content } => self.process_state(content).await,
                GameEvent::ChatLine { .. } | GameEvent::OpponentGone { .. } => {
                    Ok(GameExecutionState::Running)
                }
            },
        }
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
                Side::W
            } else if self.bot_name == game.black.name {
                log::info!("Detected lambda is playing as black");
                Side::B
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
        let mut state = Board::default();
        state.play_uci(moves)?;
        Ok((state.active(), state.position_count() as u32))
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
                        Side::W => (state.wtime, state.winc),
                        Side::B => (state.btime, state.binc),
                    };
                    tokio::select! {
                        _ = self.cancel_token.cancelled() => {
                            log::info!("Move selection cancelled!");
                            Ok(GameExecutionState::Cancelled)
                        },
                        computed_move_result = self.moves.choose(
                            state.moves.as_str(),
                            Duration::from_millis(max(MIN_COMPUTE_TIME_MS, remaining - MOVE_LATENCY_MS)),
                            Duration::from_millis(increment)
                        ) => {
                            self.lichess.client.post_move(
                                self.lichess.game_id.as_str(),
                                computed_move_result?.as_str()
                            ).await?;
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
        self.inferred_metadata.as_ref().ok_or(anyhow!("Metadata not initialized"))
    }
}
