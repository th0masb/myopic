use std::fmt::Debug;

use crate::eval::EvalFacet;
use crate::{Board, ChessBoard};

pub fn test_facet_evolution<F>(pgn: &str, expected_states: Vec<F>)
where
    F: EvalFacet<Board> + Default + PartialEq + Debug + Clone,
{
    // Parse the pgn moves
    let mut board = Board::default();
    let moves = board.play_pgn(pgn).unwrap();

    assert_eq!(moves.len(), expected_states.len());

    // Run through the moves comparing against the expected states
    let mut board = Board::default();
    let mut under_test = F::default();
    for (expected, mv) in expected_states.into_iter().zip(moves.iter()) {
        let state_start = under_test.clone();
        let position = board.clone();
        under_test.make(mv, &position);
        assert_eq!(expected, under_test);
        under_test.unmake(mv);
        assert_eq!(state_start, under_test);
        under_test.make(mv, &position);
        board.make(mv.clone()).unwrap();
    }
}
