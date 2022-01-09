mod endings;
mod openings;

use crate::endings::LichessEndgameService;
use crate::openings::DynamoOpeningService;
use async_trait::async_trait;
use lambda_payloads::chessmove2::*;
use lambda_runtime::{handler_fn, Context, Error};
use log;
use myopic_brain::{anyhow, Board, ChessBoard, Move};
use simple_logger::SimpleLogger;
use std::fmt::Display;

#[async_trait]
pub trait LookupMoveService<B: ChessBoard + Send>: Display {
    async fn lookup(&self, position: B) -> anyhow::Result<Option<Move>>;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(handler_fn(move_handler)).await?;
    Ok(())
}

async fn move_handler(event: ChooseMoveEvent, _: Context) -> Result<ChooseMoveOutput, Error> {
    // Setup the current game position
    let mut board = Board::default();
    board.play_uci(event.moves_played.as_str())?;

    match perform_lookups(&event, board.clone()).await? {
        Some(mv) => Ok(ChooseMoveOutput {
            best_move: mv.uci_format(),
            search_details: None,
        }),
        None => {
            todo!()
        }
    }
}

/// Attempt to lookup precomputed moves from various sources
async fn perform_lookups<B>(event: &ChooseMoveEvent, position: B) -> anyhow::Result<Option<Move>>
where
    B: 'static + ChessBoard + Clone + Send,
{
    let lookup_services: Vec<Box<dyn LookupMoveService<B>>> = vec![
        Box::new(DynamoOpeningService::try_from(event.opening_table.clone())?),
        Box::new(LichessEndgameService::default()),
    ];
    for service in lookup_services.iter() {
        match service.lookup(position.clone()).await {
            Ok(None) => {
                log::info!("{} could not find a move for this position", service);
            }
            Err(e) => {
                log::error!("Error during lookup for {}: {}", service, e);
            }
            Ok(Some(mv)) => {
                log::info!("{} retrieved {}", service, mv);
                return Ok(Some(mv))
            }
        }
    }
    Ok(None)
}

