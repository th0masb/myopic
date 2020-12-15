use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use std::time::Instant;

#[rustfmt::skip]
/// Run on system76
/// ------------------------------------------------------------------------------------------------
/// 15/12/20 | 4(8)(2) | 100   | 0      | 3,324,402          | This is a control run on master to test
///          |         |       |        |                    | the addition of first attempt at
///          |         |       |        |                    | heuristically ordering all moves
///          |         |       |        |                    | according to their quality during the
///          |         |       |        |                    | negamax search.
/// ------------------------------------------------------------------------------------------------
/// 15/12/20 | 4(8)(2) | 100   | 0      | 3,266,659          | Run with heuristic move ordering
///          |         |       |        |                    | changes. No significat changes at all!
///          |         |       |        |                    | I tried running the benchmark at depth 3
///          |         |       |        |                    | and ~90% of cases passed which explains
///          |         |       |        |                    | the lack of increase as the principle
///          |         |       |        |                    | variation from depth 3 was the optimal
///          |         |       |        |                    | move in most cases.
/// ------------------------------------------------------------------------------------------------
#[test]
#[ignore]
fn benchmark() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let data_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        std::env::var("MIDDLEGAME_INPUT_DATA").unwrap(),
    );
    let max_positions = std::env::var("MIDDLEGAME_MAX_CASES")?.parse::<usize>()?;
    let depth = std::env::var("MIDDLEGAME_DEPTH")?.parse::<usize>()?;

    let positions = BufReader::new(File::open(&data_path)?)
        .lines()
        .take(max_positions)
        .map(|l| l.unwrap())
        .map(|l| match crate::position(l.as_str()) {
            Err(message) => panic!("{}", message),
            Ok(position) => position,
        })
        .collect::<Vec<_>>();

    let start = Instant::now();
    let mut best_moves = vec![];
    for (i, position) in positions.into_iter().enumerate() {
        if i % 5 == 0 {
            println!("[Position {}, Duration {}ms]", i, start.elapsed().as_millis());
        }
        best_moves.push(crate::search(position, depth))
    }
    println!("Successfully computed {} moves at depth {} in {}ms", best_moves.len(), depth, start.elapsed().as_millis());
    Ok(())
}
