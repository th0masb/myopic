use itertools;
use std::collections::btree_set::BTreeSet;

use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::board::testutils::TestBoard;
use crate::board::Board;
use crate::board::Move;

type PrototypeMoveSet = (BitBoard, BitBoard);

struct TestCase {
    board: TestBoard,

    expected_enpassant_moves: Vec<BitBoard>,

    expected_castle_moves: Vec<CastleZone>,
    expected_promotion_moves: Vec<PrototypeMoveSet>,
    expected_standard_moves: Vec<PrototypeMoveSet>,

    expected_promotion_attacks: Vec<PrototypeMoveSet>,
    expected_standard_attacks: Vec<PrototypeMoveSet>,
}

fn sq(set: BitBoard) -> Square {
    set.into_iter().next().unwrap()
}

fn convert_case(case: TestCase) -> (Board, BTreeSet<Move>, BTreeSet<Move>) {
    let board = case.board.to_board();
    let piece_at = |sq: Square| board.pieces.piece_at(sq).unwrap();

    let enpassant_moves = case
        .expected_enpassant_moves
        .iter()
        .map(|&set| Move::Enpassant(sq(set)))
        .collect::<Vec<_>>();

    let castle_moves = case
        .expected_castle_moves
        .iter()
        .map(|&sq| Move::Castle(sq))
        .collect::<Vec<_>>();

    let promotion_moves = convert_promotion(&board, case.expected_promotion_moves);
    let standard_moves = convert_standard(&board, case.expected_standard_moves);

    let promotion_attacks = convert_promotion(&board, case.expected_promotion_attacks);
    let standard_attacks = convert_standard(&board, case.expected_standard_attacks);

    let expected_moves = combine(vec![
        enpassant_moves.clone(),
        castle_moves,
        promotion_moves,
        standard_moves,
    ]);
    let expected_attacks = combine(vec![enpassant_moves, promotion_attacks, standard_attacks]);

    (board, expected_moves, expected_attacks)
}

fn combine(moves: Vec<Vec<Move>>) -> BTreeSet<Move> {
    itertools::concat(moves).iter().collect()
}

fn convert_promotion(board: &Board, source: Vec<PrototypeMoveSet>) -> Vec<Move> {
    source
        .iter()
        .flat_map(|&(s, ts)| Move::promotions(board.active, sq(s), ts))
        .collect()
}

fn convert_standard(board: &Board, source: Vec<PrototypeMoveSet>) -> Vec<Move> {
    let piece_at = |sq: Square| board.pieces.piece_at(sq).unwrap();
    source
        .iter()
        .map(|&(s, ts)| (piece_at(sq(s)), sq(s), ts))
        .flat_map(|(p, s, ts)| Move::standards(p, s, ts))
        .collect()
}

fn execute_test(case: TestCase) {
    let (board, expected_moves, expected_attacks) = convert_case(case);
    assert_eq!(expected_moves, board.compute_moves().iter().collect::<BTreeSet<_>>());
    assert_eq!(expected_attacks, board.compute_attacks().iter().collect::<BTreeSet<_>>());
}
