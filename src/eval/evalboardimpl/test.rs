use crate::board::Move;
use crate::board::Board;
use crate::board;
use crate::base::Reflectable;
use crate::eval::evalboardimpl::SimpleEvalBoard;
use crate::eval::tables;

#[derive(Clone, Eq, PartialEq)]
struct TestCase<B: Board> {
    start_position: B,
    moves: Vec<Move>,
}

impl<B: Board> Reflectable for TestCase<B> {
    fn reflect(&self) -> Self {
        TestCase {
            start_position: self.start_position.reflect(),
            moves: self.moves.reflect(),
        }
    }
}

fn execute_test<B: Board>(test_case: TestCase<B>) {
    execute_test_impl(test_case.clone());
    execute_test_impl(test_case.reflect());
}

fn execute_test_impl<B: Board>(test_case: TestCase<B>) {
    let mut start = SimpleEvalBoard::new(test_case.start_position);
    for evolution in test_case.moves {
        let discards = start.evolve(&evolution);
        assert_eq!(tables::total_midgame(&start), start.mid_eval);
        assert_eq!(tables::total_endgame(&start), start.end_eval);
        start.devolve(&evolution, discards);
        assert_eq!(tables::total_midgame(&start), start.mid_eval);
        assert_eq!(tables::total_endgame(&start), start.end_eval);
        start.evolve(&evolution);
    }
}

fn test(start_fen: &'static str, moves: Vec<Move>) {
    execute_test(TestCase {
        start_position: board::from_fen(start_fen).unwrap(),
        moves,
    })
}