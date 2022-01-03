mod lookup;

use async_trait::async_trait;
use lambda_payloads::chessmove2::*;
use lambda_runtime::{handler_fn, Context, Error};
use myopic_brain::{anyhow, ChessBoard, Move, Board};
use simple_logger::SimpleLogger;
use crate::lookup::LookupMoveService;

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
    let lookups: Vec<Box<dyn LookupMoveService>> = vec![
    ];

    todo!()
    //    let first_name = event["firstName"].as_str().unwrap_or("world");
    //
    //    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
