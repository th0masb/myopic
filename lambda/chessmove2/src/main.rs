mod lookup;

use crate::lookup::{DynamoOpeningMoveService, LookupMoveService};
use async_trait::async_trait;
use lambda_payloads::chessmove2::*;
use lambda_runtime::{handler_fn, Context, Error};
use myopic_brain::{anyhow, Board, ChessBoard, Move};
use simple_logger::SimpleLogger;

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
    let lookups: Vec<Box<dyn LookupMoveService<Board>>> = vec![
        Box::new(DynamoOpeningMoveService::from(event.opening_table.clone()))
    ];
    for service in lookups.iter() {
        service.lookup(board.clone()).await?;
    }

    todo!()
    //    let first_name = event["firstName"].as_str().unwrap_or("world");
    //
    //    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
