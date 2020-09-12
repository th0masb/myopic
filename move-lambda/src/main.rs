use lambda_runtime::{error::HandlerError, lambda, Context};
use myopic_board::Move;
use myopic_brain::{eval::new_board, search};
use myopic_core::castlezone::CastleZone;
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use std::cmp::min;
use std::time::Duration;
use simple_logger::SimpleLogger;
use std::error::Error;

// Five minutes in ms, don't want this lambda to take too long.
const MAX_EXECUTION_MILLIS: u64 = 5 * 60 * 1000;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;
    lambda!(move_compute_handler);
    Ok(())
}

#[derive(Deserialize, Clone)]
struct ComputeMoveEvent {
    position: String,
    #[serde(rename = "maxTimeMillis")]
    max_time_millis: u64,
    #[serde(rename = "maxDepth")]
    max_depth: usize,
}

#[derive(Serialize, Clone)]
struct ComputeMoveOutput {
    #[serde(rename = "bestMove")]
    best_move: String,
    #[serde(rename = "depthSearched")]
    depth_searched: usize,
    #[serde(rename = "searchDurationMillis")]
    search_duration_millis: u64,
    evaluation: i32,
}

fn move_compute_handler(
    e: ComputeMoveEvent,
    _ctx: Context,
) -> Result<ComputeMoveOutput, HandlerError> {
    let max_duration = Duration::from_millis(min(e.max_time_millis, MAX_EXECUTION_MILLIS));
    let search_terminator = (max_duration, e.max_depth);
    match new_board(e.position.as_str()).and_then(|board| search(board, search_terminator)) {
        Err(message) => bail!(message.as_str()),
        Ok(outcome) => Ok(ComputeMoveOutput {
            best_move: stringify_move(outcome.best_move),
            depth_searched: outcome.depth,
            evaluation: outcome.eval,
            search_duration_millis: outcome.time.as_millis() as u64
        }),
    }
}

fn stringify_move(mv: Move) -> String {
    match mv {
        Move::Standard(_, src, dest) =>
            format!("{}{}", src, dest).to_lowercase(),
        Move::Promotion(src, dest, piece) =>
            format!("{}{}{:?}", src, dest, piece).to_lowercase(),
        Move::Enpassant(src, dest) =>
            format!("{}{}", src, dest).to_lowercase(),
        Move::Castle(zone) => match zone {
            CastleZone::WK | CastleZone::BK => String::from("O-O"),
            CastleZone::WQ | CastleZone::BQ => String::from("O-O-O"),
        },
    }
}
