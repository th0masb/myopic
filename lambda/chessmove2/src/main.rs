mod openings;

use crate::openings::DynamoOpeningMoveService;
use async_trait::async_trait;
use lambda_payloads::chessmove2::*;
use lambda_runtime::{handler_fn, Context, Error};
use myopic_brain::{anyhow, Board, ChessBoard, Move};
use simple_logger::SimpleLogger;

#[async_trait]
pub trait LookupMoveService<B: ChessBoard + Send> {
    async fn lookup(&self, position: B) -> anyhow::Result<Option<Move>>;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(handler_fn(func)).await?;
    Ok(())
}

async fn func(event: ChooseMoveEvent, _: Context) -> Result<ChooseMoveOutput, Error> {
    let mut board = Board::default();
    board.play_uci(event.moves_played.as_str())?;

    // Attempt to lookup precomputed moves from various sources
    let lookup_services: Vec<Box<dyn LookupMoveService<Board>>> = vec![Box::new(
        DynamoOpeningMoveService::try_from(event.opening_table.clone())?,
    )];
    for service in lookup_services.iter() {
        match service.lookup(board.clone()).await? {
            None => {}
            Some(mv) => {
                return Ok(ChooseMoveOutput {
                    best_move: mv.uci_format(),
                    // TODO Alter return payload to support this being optional
                    depth_searched: 0,
                    search_duration_millis: 0,
                    eval: 0,
                });
            }
        }
    }

    // Otherwise we need to perform a search
    todo!()
}
