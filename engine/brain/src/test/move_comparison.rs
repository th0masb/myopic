use crate::{Evaluator, SearchOutcome, SearchParameters, TranspositionsImpl};

#[test]
fn sanity_case() {
    assert_move_better("1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6", "f1e2", "c3d5", 3)
}
#[test]
fn knight_avoid_rim_white() {
    assert_move_better("1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6", "g1f3", "g1h3", 3)
}

#[test]
fn knight_avoid_rim_black() {
    assert_move_better(
        "1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6 6. Nf3",
        "b8c6",
        "b8a6",
        3,
    )
}

#[test]
fn development_preferred_white() {
    assert_move_better("1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6", "f1d3", "c3b5", 3)
}

#[test]
fn development_preferred_black() {
    assert_move_better(
        "1. d4 f5 2. Nc3 Nf6 3. Bg5 d5 4. Bxf6 exf6 5. e3 Be6 6. Bd3",
        "f8d6",
        "c7c6",
        3,
    )
}

#[test]
fn promotion_eval_bug() {
    assert_move_better(
        "1. d4 d5 2. e3 Nf6 3. c4 c6 4. Nc3 e6 5. Bd3 dxc4 6. Bxc4 b5 7. Be2 Bd6 8. e4 b4 9. e5 bxc3 10. exf6 O-O 11. fxg7",
        "f8e8",
        "c3b2",
        4
    )
}

const TABLE_SIZE: usize = 10000;

fn assert_move_better(
    pgn: &str,
    expected_better_uci_move: &str,
    expected_worse_uci_move: &str,
    depth: usize,
) {
    let outcome_from_better_move = search_after_move(pgn, expected_better_uci_move, depth);
    let outcome_from_worse_move = search_after_move(pgn, expected_worse_uci_move, depth);

    // These are measurements of how good the move is for the opponent, so we want to minimise
    if outcome_from_better_move.relative_eval > outcome_from_worse_move.relative_eval {
        panic!(
            "After better: {}\nAfter worse:  {}",
            serde_json::to_string(&outcome_from_better_move).unwrap(),
            serde_json::to_string(&outcome_from_worse_move).unwrap(),
        )
    }
}

fn search_after_move(pgn: &str, mv: &str, depth: usize) -> SearchOutcome {
    let mut board = Evaluator::default();
    board.play_pgn(pgn).expect(format!("Invalid {}", pgn).as_str());
    board.play_uci(mv).expect(format!("Invalid {} {}", pgn, mv).as_str());
    crate::search(
        board,
        SearchParameters { terminator: depth, table: &mut TranspositionsImpl::new(TABLE_SIZE) },
    )
    .map_err(|e| panic!("Could not search at {}: {}", pgn, e))
    .unwrap()
}
