use lambda_runtime::{error::HandlerError, lambda, Context};
use myopic_brain::negamax::SearchContext;
use myopic_brain::{EvalBoardImpl, MutBoardImpl};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger::SimpleLogger;
use std::error::Error;
use std::time::Duration;

const DEFAULT_TIMEOUT_MILLIS: u64 = 1000;
const DEFAULT_MAX_DEPTH: usize = 10;

/// Input payload
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ComputeMoveEvent {
    #[serde(rename = "fen")]
    Fen {
        #[serde(flatten)]
        terminator: SearchTerminator,
        position: String,
    },

    #[serde(rename = "uciSequence")]
    UciSequence {
        #[serde(flatten)]
        terminator: SearchTerminator,
        sequence: String,
    },
}

#[derive(Deserialize, Debug, Copy, Clone)]
struct SearchTerminator {
    #[serde(rename = "maxDepth", default)]
    max_depth: MaxDepth,
    #[serde(rename = "timeoutMillis", default)]
    timeout_millis: TimeoutMillis,
}
impl myopic_brain::SearchTerminator for SearchTerminator {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        (
            Duration::from_millis(self.timeout_millis.0),
            self.max_depth.0,
        )
            .should_terminate(ctx)
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
struct MaxDepth(usize);
impl Default for MaxDepth {
    fn default() -> Self {
        MaxDepth(DEFAULT_MAX_DEPTH)
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
struct TimeoutMillis(u64);
impl Default for TimeoutMillis {
    fn default() -> Self {
        TimeoutMillis(DEFAULT_TIMEOUT_MILLIS)
    }
}

/// Output payload
#[derive(Serialize, Clone)]
struct ComputeMoveOutput {
    #[serde(rename = "bestMove")]
    best_move: String,
    #[serde(rename = "depthSearched")]
    depth_searched: usize,
    #[serde(rename = "searchDurationMillis")]
    search_duration_millis: u64,
    eval: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    lambda!(move_compute_handler);
    Ok(())
}

fn move_compute_handler(
    e: ComputeMoveEvent,
    _ctx: Context,
) -> Result<ComputeMoveOutput, HandlerError> {
    let terminator = extract_terminator(&e);
    let position = extract_position(&e)?;
    myopic_brain::search(position, terminator)
        .map(|outcome| ComputeMoveOutput {
            best_move: outcome.best_move.uci_format(),
            depth_searched: outcome.depth,
            eval: outcome.eval,
            search_duration_millis: outcome.time.as_millis() as u64,
        })
        .map_err(|message| HandlerError::from(message.as_str()))
}

fn extract_position(e: &ComputeMoveEvent) -> Result<EvalBoardImpl<MutBoardImpl>, HandlerError> {
    match e {
        ComputeMoveEvent::Fen { position, .. } => myopic_brain::pos::from_fen(position.as_str())
            .map_err(|e| HandlerError::from(e.as_str())),
        ComputeMoveEvent::UciSequence { sequence, .. } => {
            myopic_brain::pos::from_uci(sequence.as_str())
                .map_err(|e| HandlerError::from(e.as_str()))
        }
    }
}

fn extract_terminator(e: &ComputeMoveEvent) -> SearchTerminator {
    match e {
        ComputeMoveEvent::Fen { terminator, .. } => *terminator,
        ComputeMoveEvent::UciSequence { terminator, .. } => *terminator,
    }
}
