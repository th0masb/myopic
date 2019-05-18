use std::collections::btree_set::BTreeSet;

use itertools;

use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::testutils::TestBoard;
use crate::board::Board;
use crate::board::Move;
use crate::base::Reflectable;

type PrototypeMoveSet = (BitBoard, BitBoard);
type MoveSet = BTreeSet<Move>;

struct TestCase {
    board: TestBoard,

    expected_castle_moves: Vec<CastleZone>,
    expected_enpassant_moves: Vec<BitBoard>,
    expected_promotion_moves: Vec<PrototypeMoveSet>,
    expected_standard_moves: Vec<PrototypeMoveSet>,

    expected_enpassant_attacks: Vec<BitBoard>,
    expected_promotion_attacks: Vec<PrototypeMoveSet>,
    expected_standard_attacks: Vec<PrototypeMoveSet>,
}

impl Reflectable for TestCase {
    fn reflect(&self) -> Self {
        //let reflect_moveset = |(left, right)| (reflect_bitboard(*left), reflect_bitboard(*right));
//        TestCase {
//            board: self.board.reflect(),
//
//        }
        unimplemented!()
    }
}

fn sq(set: BitBoard) -> Square {
    set.into_iter().next().unwrap()
}

fn convert_case(case: TestCase) -> (Board, MoveSet, MoveSet) {
    let board = case.board.to_board();

    let castle_moves = case
        .expected_castle_moves
        .iter()
        .map(|&zone| Move::Castle(zone))
        .collect::<Vec<_>>();

    let convert_enpassant = |source: Vec<BitBoard>| {
        source
            .iter()
            .map(|&set| Move::Enpassant(sq(set)))
            .collect::<Vec<_>>()
    };

    let enpassant_moves = convert_enpassant(case.expected_enpassant_moves);
    let promotion_moves = convert_promotion(&board, case.expected_promotion_moves);
    let standard_moves = convert_standard(&board, case.expected_standard_moves);

    let enpassant_attacks = convert_enpassant(case.expected_enpassant_attacks);
    let promotion_attacks = convert_promotion(&board, case.expected_promotion_attacks);
    let standard_attacks = convert_standard(&board, case.expected_standard_attacks);

    let expected_moves = combine(vec![
        enpassant_moves.clone(),
        castle_moves,
        promotion_moves,
        standard_moves,
    ]);
    let expected_attacks = combine(vec![enpassant_attacks, promotion_attacks, standard_attacks]);

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

    let actual_attacks = board
        .compute_attacks_or_escapes()
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        expected_attacks.clone(),
        actual_attacks.clone(),
        "Differences are: {:?}",
        compute_difference(expected_attacks, actual_attacks)
    );
}

fn compute_difference(left: MoveSet, right: MoveSet) -> (MoveSet, MoveSet) {
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
        expected_enpassant_attacks: vec![F5],

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

#[test]
fn case_2() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![B3 | D4 | F2 | G2, C3, A3 | F3, A1 | H1, C2, E1],
            blacks: vec![C6 | E4 | F7 | G7 | H7, B8, B5 | G6, A8 | H8, C7, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            enpassant: Some(sq(D3)),
        },

        expected_castle_moves: vec![],

        expected_enpassant_moves: vec![E4],
        expected_enpassant_attacks: vec![E4],

        expected_promotion_moves: vec![],
        expected_promotion_attacks: vec![],

        expected_standard_moves: vec![
            (A8, A7 | A6 | A5 | A4 | A3),
            (B8, A6 | D7),
            (B5, A6 | A4 | C4 | D3 | E2 | F1),
            (
                C7,
                C8 | D8 | D7 | E7 | D6 | E5 | F4 | G3 | H2 | B6 | A5 | B7 | A7,
            ),
            (C6, C5),
            (E8, D8 | D7),
            (E4, E3 | F3),
            (F7, F6 | F5),
            (G6, H5 | F5),
            (H7, H6 | H5),
            (H8, G8 | F8),
        ],

        expected_standard_attacks: vec![(E4, F3), (A8, A3)],
    });
}

#[test]
fn case_3() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![A7 | D4 | F2 | G2, F5 | C3, A4, B1 | H1, BitBoard::EMPTY, E1],
            blacks: vec![
                A2 | D5 | F7 | G7 | H7,
                F4 | C6,
                G6 | A5,
                B8 | H8,
                BitBoard::EMPTY,
                E8,
            ],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::WK | CastleZoneSet::BK,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        expected_castle_moves: vec![CastleZone::WK],

        expected_enpassant_moves: vec![],
        expected_enpassant_attacks: vec![],

        expected_promotion_moves: vec![(A7, A8 | B8)],
        expected_promotion_attacks: vec![(A7, B8)],

        expected_standard_moves: vec![
            (A4, B3 | C2 | D1 | B5 | C6),
            (B1, B2 | B3 | B4 | B5 | B6 | B7 | B8 | A1 | C1 | D1),
            (E1, D1 | D2 | F1),
            (F2, F3),
            (G2, G3 | G4),
            (F5, E3 | G3 | H4 | H6 | G7 | E7 | D6),
            (H1, G1 | F1 | H2 | H3 | H4 | H5 | H6 | H7),
        ],

        expected_standard_attacks: vec![(A4, C6), (B1, B8), (F5, G7), (H1, H7)],
    });
}

#[test]
fn case_4() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![A7 | D4 | F2 | G2, F5 | C3, A4, B1 | H1, BitBoard::EMPTY, E1],
            blacks: vec![
                A2 | D5 | F7 | G7 | H7,
                F4 | C6,
                G6 | A5,
                B8 | H8,
                BitBoard::EMPTY,
                E8,
            ],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::WK | CastleZoneSet::BK,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        expected_castle_moves: vec![CastleZone::BK],
        expected_enpassant_moves: vec![],
        expected_enpassant_attacks: vec![],

        expected_promotion_moves: vec![(A2, A1 | B1)],
        expected_promotion_attacks: vec![(A2, B1)],

        expected_standard_moves: vec![
            (A5, B6 | C7 | D8 | B4 | C3),
            (B8, B7 | B6 | B5 | B4 | B3 | B2 | B1 | A8 | C8 | D8),
            (E8, D7 | D8 | F8),
            (F7, F6),
            (F4, D3 | E2 | G2 | H3 | H5 | E6),
            (G6, F5 | H5),
            (H8, G8 | F8),
            (H7, H6 | H5),
        ],

        expected_standard_attacks: vec![(A5, C3), (B8, B1), (F4, G2), (G6, F5)],
    });
}

#[test]
fn case_5() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![B3 | F2 | G2, C3, F3, A1 | E1, C2, F1],
            blacks: vec![C6 | F7 | G7, BitBoard::EMPTY, B5 | G6, A8 | H8, C7, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::BK | CastleZoneSet::BQ,
            white_status: Some(CastleZone::WK),
            black_status: None,
            enpassant: None,
        },

        expected_castle_moves: vec![],

        expected_enpassant_moves: vec![],
        expected_enpassant_attacks: vec![],

        expected_promotion_moves: vec![],
        expected_promotion_attacks: vec![],

        expected_standard_moves: vec![(F1, G1), (E1, E2), (C2, D3 | E2), (F3, E2), (C3, E2 | B5)],
        expected_standard_attacks: vec![(F1, G1), (E1, E2), (C2, D3 | E2), (F3, E2), (C3, E2 | B5)],
    });
}

#[test]
fn case_6() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![B3 | F2 | G2, C3, F3, A1 | E1, C2, F1],
            blacks: vec![C6 | F7 | G7, F4, B5 | G6, A8 | H8, C7, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::BK | CastleZoneSet::BQ,
            white_status: Some(CastleZone::WK),
            black_status: None,
            enpassant: None,
        },

        expected_castle_moves: vec![],

        expected_enpassant_moves: vec![],
        expected_enpassant_attacks: vec![],

        expected_promotion_moves: vec![],
        expected_promotion_attacks: vec![],

        expected_standard_moves: vec![
            (E8, F8 | D7 | D8),
            (B5, E2),
            (G6, E4),
            (C7, E5 | E7),
            (F4, E2 | E6),
        ],
        expected_standard_attacks: vec![
            (E8, F8 | D7 | D8),
            (B5, E2),
            (G6, E4),
            (C7, E5 | E7),
            (F4, E2 | E6),
        ],
    });
}
