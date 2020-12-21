use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[rustfmt::skip]
/// Run on system76
/// ------------------------------------------------------------------------------------------------
/// 15/12/20 | 4(8)(2) | 100   | 0      | 737,690            | This is a control run on master to test
///          |         |       |        |                    | the addition of first attempt at
///          |         |       |        |                    | heuristically ordering all moves
///          |         |       |        |                    | according to their quality during the
///          |         |       |        |                    | negamax search.
/// ------------------------------------------------------------------------------------------------
/// 15/12/20 | 4(8)(2) | 100   | 0      | 182,397!!          | Massive difference in adding the move
///          |         |       |        |                    | ordering :)
/// ------------------------------------------------------------------------------------------------
///
/// Again on System76, I think the positions changed since last run but not too much difference
/// in the control run.
/// ------------------------------------------------------------------------------------------------
/// 21/12/20 | 4(8)(2) | 100   | 0      | 179,573            | This is a control run on master to test
///          |         |       |        |                    | the switch to negascout and the
///          |         |       |        |                    | accompanying shallow eval move ordering
/// ------------------------------------------------------------------------------------------------
/// 21/12/20 | 4(8)(2) | 100   | 0      | 17,769             | Order of magnitude quicker!
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
        .map(|l| match crate::pos::from_fen(l.as_str()) {
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
