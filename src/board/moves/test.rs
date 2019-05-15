use itertools;
use std::collections::btree_set::BTreeSet;

use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::testutils::TestBoard;
use crate::board::Board;
use crate::board::Move;
use std::cmp;

type PrototypeMoveSet = (BitBoard, BitBoard);
type MoveSet = BTreeSet<Move>;

struct TestCase {
    board: TestBoard,

    expected_castle_moves: Vec<CastleZone>,
    expected_enpassant_moves: Vec<BitBoard>,

    expected_promotion_moves: Vec<PrototypeMoveSet>,
    expected_standard_moves: Vec<PrototypeMoveSet>,

    expected_promotion_attacks: Vec<PrototypeMoveSet>,
    expected_standard_attacks: Vec<PrototypeMoveSet>,
}

fn sq(set: BitBoard) -> Square {
    set.into_iter().next().unwrap()
}

fn convert_case(case: TestCase) -> (Board, MoveSet, MoveSet) {
    let board = case.board.to_board();

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

fn combine(moves: Vec<Vec<Move>>) -> MoveSet {
    itertools::concat(moves).into_iter().collect()
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
    let actual_moves = board.compute_moves().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        expected_moves.clone(),
        actual_moves.clone(),
        "Differences are: {:?}",
        compute_difference(expected_moves, actual_moves)
    );

    let actual_attacks = board.compute_attacks().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        expected_attacks.clone(),
        actual_attacks.clone(),
        "Differences are: {:?}",
        compute_difference(expected_attacks, actual_attacks)
    );
}

fn compute_difference(left: MoveSet, right: MoveSet) -> (MoveSet, MoveSet)
{
    (
        left.clone().difference(&right).map(|m| m.clone()).collect(),
        right.clone().difference(&left).map(|m| m.clone()).collect(),
    )
}

#[test]
fn case_1() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![B3 | F5 | F2 | G2, C3, A3 | F3, A1 | H1, C2, E1],
            blacks: vec![C6 | E5 | F7 | G7 | H7, B8, B5 | G6, A8 | H8, C7, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            enpassant: Some(sq(E6)),
        },

        expected_castle_moves: vec![CastleZone::WQ],
        expected_enpassant_moves: vec![F5],

        expected_promotion_moves: vec![],
        expected_promotion_attacks: vec![],

        expected_standard_moves: vec![
            (A1, A2 | B1 | C1 | D1),
            (A3, B2 | C1 | B4 | C5 | D6 | E7 | F8),
            (B3, B4),
            (C3, A2 | A4 | B5 | D5 | E4 | E2 | B1 | D1),
            (C2, B2 | B1 | A2 | C1 | D1 | D2 | E2 | D3 | E4),
            (E1, D1 | D2),
            (F3, E2 | D1 | E4 | D5 | C6 | G4 | H5),
            (F5, F6 | G6),
            (G2, G3 | G4),
            (H1, G1 | F1 | H2 | H3 | H4 | H5 | H6 | H7),
        ],

        expected_standard_attacks: vec![(C3, B5), (F5, G6), (F3, C6), (H1, H7)],
    });
}
