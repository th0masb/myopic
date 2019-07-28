use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::board::test_board::TestBoard;
use crate::board::BoardImpl;
use crate::eval::see::See;
use crate::pieces::Piece;

/// Dummy piece values
fn value(piece: Piece) -> i32 {
    let values = [1, 3, 3, 5, 9, 100];
    values[(piece as usize) % 6]
}

#[derive(Clone, Debug)]
struct TestCase {
    board: TestBoard,
    expected: Vec<(Square, Square, i32)>,
}

impl Reflectable for TestCase {
    fn reflect(&self) -> Self {
        let mut reflected_expected = Vec::new();
        for (src, targ, result) in self.expected.iter() {
            reflected_expected.push((src.reflect(), targ.reflect(), *result));
        }
        TestCase {
            board: self.board.reflect(),
            expected: reflected_expected,
        }
    }
}

fn execute_case(test_case: TestCase) {
    execute_case_impl(test_case.clone());
    execute_case_impl(test_case.reflect())
}

fn execute_case_impl(test_case: TestCase) {
    let board = BoardImpl::from(test_case.board);
    for (source, target, expected_value) in test_case.expected.into_iter() {
        let see = See {
            board: &board,
            source,
            target,
            value,
        };
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
        board: TestBoard::positions(
            vec![C5, ZERO, ZERO, D3, D2, B2],
            vec![D6, F7, B8, ZERO, G6, H8],
        ),
        expected: vec![(Square::C5, Square::D6, 0), (Square::D3, Square::D6, -2)],
    })
}

#[test]
fn case_2() {
    execute_case(TestCase {
        board: TestBoard::positions(
            vec![ZERO, G7, E6, ZERO, C6, A8],
            vec![B5 | F5, F4, ZERO, H6, E3, H1],
        ),
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
