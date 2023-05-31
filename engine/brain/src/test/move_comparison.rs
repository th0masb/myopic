use myopic_board::{ChessBoard, UciMove};
use crate::{EvalBoard, SearchOutcome, SearchParameters};

const TABLE_SIZE: usize = 10000;

fn assert_move_better(
    pgn: &str,
    expected_better_uci_move: &str,
    expected_worse_uci_move: &str,
    depth: usize
) {
    let better_outcome = eval_move(pgn, expected_better_uci_move, depth);
    let worse_outcome = eval_move(pgn, expected_worse_uci_move, depth);
    if -better_outcome.eval < -worse_outcome.eval {
        panic!(
            "{} vs {}\n{:?} vs {:?}",
            -better_outcome.eval,
            -worse_outcome.eval,
            better_outcome.optimal_path,
            worse_outcome.optimal_path
        )
    }
}

fn eval_move(pgn: &str, mv: &str, depth: usize) -> SearchOutcome {
    let mut board = EvalBoard::default();
    board.play_pgn(pgn).expect(format!("Invalid {}", pgn).as_str());
    board.play_uci(mv).expect(format!("Invalid {} {}", pgn, mv).as_str());
    crate::search(board, SearchParameters {
        terminator: depth,
        table_size: TABLE_SIZE,
    }).map_err(|e| panic!("Could not search at {}: {}", pgn, e)).unwrap()
}

#[test]
fn case_0() {
    assert_move_better(
        "1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6",
        "g1f3",
        "g1h3",
        3,
    )
}