use lambda_runtime::{error::HandlerError, lambda, Context};
use myopic_brain::negascout::SearchContext;
use myopic_brain::{Board, ChessBoard, EvalBoard, SearchParameters};
use serde_derive::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::error::Error;
use std::time::Duration;

const DEFAULT_TIMEOUT_MILLIS: u64 = 1000;
const DEFAULT_MAX_DEPTH: usize = 10;
const DEFAULT_TABLE_SIZE: usize = 50000;

fn default_tablesize() -> usize {
    DEFAULT_TABLE_SIZE
}

/// Input payload
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
enum ComputeMoveEvent {
    #[serde(rename = "fen")]
    Fen {
        #[serde(flatten)]
        terminator: SearchTerminator,
        position: String,
        #[serde(rename = "tableSize", default = "default_tablesize")]
        table_size: usize,
    },

    #[serde(rename = "uciSequence")]
    UciSequence {
        #[serde(flatten)]
        terminator: SearchTerminator,
        sequence: String,
        #[serde(rename = "startFen")]
        start_fen: Option<String>,
        #[serde(rename = "tableSize", default = "default_tablesize")]
        table_size: usize,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
struct MaxDepth(usize);
impl Default for MaxDepth {
    fn default() -> Self {
        MaxDepth(DEFAULT_MAX_DEPTH)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
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
    log::info!("Received input payload {}", serde_json::to_string(&e)?);
    let terminator = extract_params(&e);
    let position =
        extract_position(&e).map_err(|err| HandlerError::from(err.to_string().as_str()))?;
    let output_payload = myopic_brain::search(position, terminator)
        .map(|outcome| ComputeMoveOutput {
            best_move: outcome.best_move.uci_format(),
            depth_searched: outcome.depth,
            eval: outcome.eval,
            search_duration_millis: outcome.time.as_millis() as u64,
        })
        .map_err(|err| HandlerError::from(err.to_string().as_str()))?;
    log::info!(
        "Computed output payload {}",
        serde_json::to_string(&output_payload)?
    );
    Ok(output_payload)
}

fn extract_position(e: &ComputeMoveEvent) -> Result<EvalBoard<Board>, anyhow::Error> {
    match e {
        ComputeMoveEvent::Fen { position, .. } => {
            EvalBoard::builder_fen(position).map(|b| b.build())
        }
        ComputeMoveEvent::UciSequence {
            sequence,
            start_fen,
            ..
        } => {
            let fen = start_fen
                .as_ref()
                .cloned()
                .unwrap_or(myopic_brain::STARTPOS_FEN.to_string());
            let mut state = if fen.as_str() == myopic_brain::STARTPOS_FEN {
                log::info!("Constructed state from standard start position");
                EvalBoard::start()
            } else {
                log::info!("Constructed state from custom position {}", fen.as_str());
                EvalBoard::builder_fen(fen.as_str())?.build()
            };
            state.play_uci(sequence.as_str())?;
            Ok(state)
        }
    }
}

fn extract_params(e: &ComputeMoveEvent) -> SearchParameters<SearchTerminator> {
    match e {
        &ComputeMoveEvent::Fen {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator,
            table_size,
        },
        &ComputeMoveEvent::UciSequence {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator,
            table_size,
        },
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use crate::{ComputeMoveEvent, SearchTerminator, MaxDepth, TimeoutMillis};

    fn deserialize_default_tablesize() -> Result<()> {
        assert_eq!(
            ComputeMoveEvent::Fen {
                position: "pos".to_string(),
                table_size: super::DEFAULT_TABLE_SIZE,
                terminator: SearchTerminator {
                    max_depth: MaxDepth(super::DEFAULT_MAX_DEPTH),
                    timeout_millis: TimeoutMillis(super::DEFAULT_TIMEOUT_MILLIS),
                }
            },
            serde_json::from_str(r#"{"type":"fen","position":"pos"}"#)?
        );
        Ok(())
    }

}
