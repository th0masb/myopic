use std::time::Instant;

use itertools::Itertools;
use lambda_runtime::{handler_fn, Context, Error};
use simple_logger::SimpleLogger;

use lambda_payloads::benchmark::*;
use myopic_brain::SearchParameters;

mod positions;

const LOG_GAP: usize = 2;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(handler_fn(handler)).await?;
    Ok(())
}

async fn handler(e: BenchStartEvent, ctx: Context) -> Result<BenchOutput, Error> {
    let roots = positions::get(e.positions)?;
    let n = roots.len();
    let start = Instant::now();
    let mut moves = vec![];
    for (i, root) in roots.into_iter().enumerate() {
        if i % LOG_GAP == 0 {
            log::info!(
                "[Position {}, Elapsed {}ms]",
                i,
                start.elapsed().as_millis()
            );
        }
        moves.push(myopic_brain::search(
            root,
            SearchParameters {
                terminator: e.depth,
                table_size: e.table_size,
            },
        )?);
    }

    let execution_times = moves
        .iter()
        .map(|o| o.time.as_millis() as u64)
        .sorted()
        .collect::<Vec<_>>();

    let output = BenchOutput {
        depth_searched: e.depth,
        positions_searched: n,
        memory_allocated_mb: ctx.env_config.memory as usize,
        min_search_time_millis: execution_times[0],
        max_search_time_millis: execution_times[n - 1],
        median_search_time_millis: execution_times[n / 2],
        average_search_time_millis: execution_times.iter().sum::<u64>() / n as u64,
        total_search_time_secs: execution_times.iter().sum::<u64>() / 1000u64,
    };

    log::info!("{}", serde_json::to_string(&output)?);
    Ok(output)
}
