use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::board;
use crate::board::Board;
use crate::eval::see::See;
use crate::pieces::Piece;

/// Dummy piece values
fn value(piece: Piece) -> i32 {
    let values = [1, 3, 3, 5, 9, 100];
    values[(piece as usize) % 6]
}

#[derive(Clone, Debug)]
struct TestCase<B: Board> {
    board: B,
    expected: Vec<(Square, Square, i32)>,
}

impl<B: Board> Reflectable for TestCase<B> {
    fn reflect(&self) -> Self {
        let mut reflected_expected = Vec::new();
        for (src, targ, result) in self.expected.iter() {
            reflected_expected.push((src.reflect(), targ.reflect(), *result));
        }
        TestCase { board: self.board.reflect(), expected: reflected_expected }
    }
}

fn execute_case<B: Board>(test_case: TestCase<B>) {
    execute_case_impl(test_case.clone());
    execute_case_impl(test_case.reflect())
}

fn execute_case_impl<B: Board>(test_case: TestCase<B>) {
    let board = test_case.board;
    for (source, target, expected_value) in test_case.expected.into_iter() {
        let see = See { board: &board, source, target, value };
        assert_eq!(
            expected_value,
            see.exchange_value(),
            "Source: {:?}, target: {:?}",
            source,
            target
        )
    }
}

const ZERO: BitBoard = BitBoard(0);

#[test]
fn case_1() {
    execute_case(TestCase {
        board: board::from_fen("1b5k/5n2/3p2q1/2P5/8/3R4/1K1Q4/8 w KQkq - 5 20").unwrap(),
        expected: vec![(Square::C5, Square::D6, 0), (Square::D3, Square::D6, -2)],
    })
}

#[test]
fn case_2() {
    execute_case(TestCase {
        board: board::from_fen("k7/6n1/2q1b2R/1P3P2/5N2/4Q3/8/K7 w KQkq - 10 30").unwrap(),
        expected: vec![
            (Square::B5, Square::C6, 9),
            (Square::C6, Square::B5, 1),
            (Square::E3, Square::E6, -3),
            (Square::F5, Square::E6, 3),
            (Square::F4, Square::E6, 3),
            (Square::H6, Square::E6, 1),
            (Square::E6, Square::F5, 1),
        ],
    })
}
