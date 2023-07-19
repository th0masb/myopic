use crate::node::EvalFacet;
use crate::position::Position;
use std::fmt::Debug;

pub fn test_facet_evolution<F>(pgn: &str, expected_states: Vec<F>)
where
    F: EvalFacet + Default + PartialEq + Debug + Clone,
{
    // Parse the pgn moves
    let board: Position = pgn.parse().unwrap();
    let moves: Vec<_> = board.history.iter().map(|(_, m)| m.clone()).collect();

    assert_eq!(moves.len(), expected_states.len());

    // Run through the moves comparing against the expected states
    let mut board = Position::default();
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
