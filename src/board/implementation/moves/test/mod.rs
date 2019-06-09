use std::collections::btree_set::BTreeSet;

use itertools;

use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::{testutils::TestBoard, BoardImpl};
use crate::board::Move;

type PrototypeMoveSet = (BitBoard, BitBoard);
type MoveSet = BTreeSet<Move>;

#[cfg(test)]
mod misc;
#[cfg(test)]
mod szukstra_tal;

#[derive(Debug, Clone)]
struct TestCase {
    board: TestBoard,

    castle_moves: Vec<CastleZone>,
    enpassant_moves: Vec<BitBoard>,
    promotion_moves: Vec<PrototypeMoveSet>,
    standard_moves: Vec<PrototypeMoveSet>,

    enpassant_attacks: Vec<BitBoard>,
    promotion_attacks: Vec<PrototypeMoveSet>,
    standard_attacks: Vec<PrototypeMoveSet>,
}

impl Reflectable for PrototypeMoveSet {
    fn reflect(&self) -> Self {
        (self.0.reflect(), self.1.reflect())
    }
}

impl Reflectable for TestCase {
    fn reflect(&self) -> Self {
        TestCase {
            board: self.board.reflect(),
            castle_moves: self.castle_moves.reflect(),
            enpassant_moves: self.enpassant_moves.reflect(),
            promotion_moves: self.promotion_moves.reflect(),
            standard_moves: self.standard_moves.reflect(),
            enpassant_attacks: self.enpassant_attacks.reflect(),
            promotion_attacks: self.promotion_attacks.reflect(),
            standard_attacks: self.standard_attacks.reflect(),
        }
    }
}

fn sq(set: BitBoard) -> Square {
    set.into_iter().next().unwrap()
}

fn convert_case(case: TestCase) -> (BoardImpl, MoveSet, MoveSet) {
    let board = case.board.to_board();

    let castle_moves = case
        .castle_moves
        .iter()
        .map(|&zone| Move::Castle(zone))
        .collect::<Vec<_>>();

    let convert_enpassant = |source: Vec<BitBoard>| {
        source
            .iter()
            .map(|&set| Move::Enpassant(sq(set)))
            .collect::<Vec<_>>()
    };

    let enpassant_moves = convert_enpassant(case.enpassant_moves);
    let promotion_moves = convert_promotion(&board, case.promotion_moves);
    let standard_moves = convert_standard(&board, case.standard_moves);

    let enpassant_attacks = convert_enpassant(case.enpassant_attacks);
    let promotion_attacks = convert_promotion(&board, case.promotion_attacks);
    let standard_attacks = convert_standard(&board, case.standard_attacks);

    let moves = combine(vec![
        enpassant_moves.clone(),
        castle_moves,
        promotion_moves,
        standard_moves,
    ]);
    let attacks = combine(vec![enpassant_attacks, promotion_attacks, standard_attacks]);

    (board, moves, attacks)
}

fn combine(moves: Vec<Vec<Move>>) -> MoveSet {
    itertools::concat(moves).into_iter().collect()
}

fn convert_promotion(board: &BoardImpl, source: Vec<PrototypeMoveSet>) -> Vec<Move> {
    source
        .iter()
        .flat_map(|&(s, ts)| Move::promotions(board.active, sq(s), ts))
        .collect()
}

fn convert_standard(board: &BoardImpl, source: Vec<PrototypeMoveSet>) -> Vec<Move> {
    let piece_at = |sq: Square| board.pieces.piece_at(sq).unwrap();
    source
        .iter()
        .map(|&(s, ts)| (piece_at(sq(s)), sq(s), ts))
        .flat_map(|(p, s, ts)| Move::standards(p, s, ts))
        .collect()
}

fn execute_test(case: TestCase) {
    let reflected_case = case.reflect();
    execute_test_impl(case);
    execute_test_impl(reflected_case);
}

fn execute_test_impl(case: TestCase) {
    let (board, moves, attacks) = convert_case(case);
    let actual_moves = board.compute_moves().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        moves.clone(),
        actual_moves.clone(),
        "Differences are: {:?}",
        compute_difference(moves, actual_moves)
    );

    let actual_attacks = board
        .compute_attacks_or_escapes()
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        attacks.clone(),
        actual_attacks.clone(),
        "Differences are: {:?}",
        compute_difference(attacks, actual_attacks)
    );
}

fn compute_difference(left: MoveSet, right: MoveSet) -> (MoveSet, MoveSet) {
    (
        left.clone().difference(&right).map(|m| m.clone()).collect(),
        right.clone().difference(&left).map(|m| m.clone()).collect(),
    )
}
