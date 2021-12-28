use std::time::Duration;

use lambda_runtime::{Context, error::HandlerError, lambda};
use simple_logger::SimpleLogger;

use lambda_payloads::chessmove::*;
use myopic_brain::{Board, ChessBoard, EvalBoard, SearchParameters};
use myopic_brain::negascout::SearchContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
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

struct SearchTerminatorWrapper(pub SearchTerminator);
impl myopic_brain::SearchTerminator for SearchTerminatorWrapper {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        let timeout = Duration::from_millis(self.0.timeout_millis.0);
        (timeout, self.0.max_depth.0 as usize).should_terminate(ctx)
    }
}

fn extract_params(e: &ComputeMoveEvent) -> SearchParameters<SearchTerminatorWrapper> {
    match e {
        &ComputeMoveEvent::Fen {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator: SearchTerminatorWrapper(terminator),
            table_size: table_size.0 as usize,
        },
        &ComputeMoveEvent::UciSequence {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator: SearchTerminatorWrapper(terminator),
            table_size: table_size.0 as usize,
        },
    }
}

