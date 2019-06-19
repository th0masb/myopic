use std::collections::btree_set::BTreeSet;

use crate::base::bitboard::BitBoard;
use crate::base::bitboard::constants::*;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::implementation::BoardImpl;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::pieces::Piece;
use crate::board::test_board::TestBoard;

type MoveSet = BTreeSet<Move>;

#[cfg(test)]
mod misc;
#[cfg(test)]
mod szukstra_tal;

#[derive(Debug, Clone)]
struct TestCase {
    board: TestBoard,

    // We use stacked vectors so we can more easily
    // write collections of moves shorthand.
    expected_all: Vec<Vec<Move>>,
    expected_attacks_checks: Vec<Vec<Move>>,
    expected_attacks: Vec<Vec<Move>>,
}

impl Reflectable for TestCase {
    fn reflect(&self) -> Self {
        TestCase {
            board: self.board.reflect(),
            expected_all: self.expected_all.reflect(),
            expected_attacks_checks: self.expected_attacks_checks.reflect(),
            expected_attacks: self.expected_attacks.reflect(),
        }
    }
}

fn s(piece: Piece, src: BitBoard, targets: BitBoard) -> Vec<Move> {
    Move::standards(piece, src.first().unwrap(), targets).collect()
}

fn p(side: Side, src: BitBoard, targets: BitBoard) -> Vec<Move> {
    Move::promotions(side, src.first().unwrap(), targets).collect()
}

fn e(src: BitBoard) -> Vec<Move> {
    src.iter().map(Move::Enpassant).collect()
}

fn c(zones: CastleZoneSet) -> Vec<Move> {
    zones.iter().map(Move::Castle).collect()
}

fn flatten(moves: Vec<Vec<Move>>) -> MoveSet {
    moves.into_iter().flat_map(|xs| xs.into_iter()).collect()
}

fn sq(set: BitBoard) -> Square {
    set.into_iter().next().unwrap()
}

fn convert_case(case: TestCase) -> (BoardImpl, Vec<(MoveComputeType, MoveSet)>) {
    let board = BoardImpl::from(case.board);
    let expected = vec![
        (MoveComputeType::All, flatten(case.expected_all)),
        (MoveComputeType::Attacks, flatten(case.expected_attacks)),
        (MoveComputeType::AttacksChecks, flatten(case.expected_attacks_checks)),
    ];
    (board, expected)
}

fn execute_test(case: TestCase) {
    let reflected_case = case.reflect();
    execute_test_impl(case);
    execute_test_impl(reflected_case);
}

fn execute_test_impl(case: TestCase) {
    let (board, results) = convert_case(case);
    for (computation_type, expected_moves) in results.into_iter() {
        let actual_moves: MoveSet = board.compute_moves(computation_type).into_iter().collect();
        assert_eq!(expected_moves.clone(),
                   actual_moves.clone(),
                   "Differences for {:?} are: {:?}",
                   computation_type,
                   compute_difference(expected_moves, actual_moves));
    }
}

fn compute_difference(left: MoveSet, right: MoveSet) -> (MoveSet, MoveSet) {
    (
        left.clone().difference(&right).map(|m| m.clone()).collect(),
        right.clone().difference(&left).map(|m| m.clone()).collect(),
    )
}
