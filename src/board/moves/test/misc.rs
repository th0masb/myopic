use super::*;

const EMPTY: BitBoard = BitBoard::EMPTY;

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

        castle_moves: vec![CastleZone::WQ],

        enpassant_moves: vec![F5],
        enpassant_attacks: vec![F5],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![
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

        standard_attacks: vec![(C3, B5), (F5, G6), (F3, C6), (H1, H7)],
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

        castle_moves: vec![],

        enpassant_moves: vec![E4],
        enpassant_attacks: vec![E4],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![
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

        standard_attacks: vec![(E4, F3), (A8, A3)],
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

        castle_moves: vec![CastleZone::WK],

        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![(A7, A8 | B8)],
        promotion_attacks: vec![(A7, B8)],

        standard_moves: vec![
            (A4, B3 | C2 | D1 | B5 | C6),
            (B1, B2 | B3 | B4 | B5 | B6 | B7 | B8 | A1 | C1 | D1),
            (E1, D1 | D2 | F1),
            (F2, F3),
            (G2, G3 | G4),
            (F5, E3 | G3 | H4 | H6 | G7 | E7 | D6),
            (H1, G1 | F1 | H2 | H3 | H4 | H5 | H6 | H7),
        ],

        standard_attacks: vec![(A4, C6), (B1, B8), (F5, G7), (H1, H7)],
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

        castle_moves: vec![CastleZone::BK],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![(A2, A1 | B1)],
        promotion_attacks: vec![(A2, B1)],

        standard_moves: vec![
            (A5, B6 | C7 | D8 | B4 | C3),
            (B8, B7 | B6 | B5 | B4 | B3 | B2 | B1 | A8 | C8 | D8),
            (E8, D7 | D8 | F8),
            (F7, F6),
            (F4, D3 | E2 | G2 | H3 | H5 | E6),
            (G6, F5 | H5),
            (H8, G8 | F8),
            (H7, H6 | H5),
        ],

        standard_attacks: vec![(A5, C3), (B8, B1), (F4, G2), (G6, F5)],
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

        castle_moves: vec![],

        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![(F1, G1), (E1, E2), (C2, D3 | E2), (F3, E2), (C3, E2 | B5)],
        standard_attacks: vec![(F1, G1), (E1, E2), (C2, D3 | E2), (F3, E2), (C3, E2 | B5)],
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

        castle_moves: vec![],

        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![
            (E8, F8 | D7 | D8),
            (B5, E2),
            (G6, E4),
            (C7, E5 | E7),
            (F4, E2 | E6),
        ],
        standard_attacks: vec![
            (E8, F8 | D7 | D8),
            (B5, E2),
            (G6, E4),
            (C7, E5 | E7),
            (F4, E2 | E6),
        ],
    });
}

#[test]
fn case_7() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![EMPTY, EMPTY, A1, E3, EMPTY, H1],
            blacks: vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, C3],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        castle_moves: vec![],

        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![(C3, C4 | C2 | B4 | D2)],
        standard_attacks: vec![(C3, C4 | C2 | B4 | D2)],
    });
}

#[test]
fn case_8() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![D5 | F5, EMPTY, EMPTY, EMPTY, EMPTY, H1],
            blacks: vec![E5, EMPTY, EMPTY, EMPTY, EMPTY, H8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            enpassant: Some(sq(E6)),
        },

        castle_moves: vec![],

        enpassant_moves: vec![D5, F5],
        enpassant_attacks: vec![D5, F5],

        promotion_moves: vec![],
        promotion_attacks: vec![],

        standard_moves: vec![(D5, D6), (F5, F6), (H1, G1 | G2 | H2)],
        standard_attacks: vec![],
    });
}

#[test]
fn case_9() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![C7 | G7 | C5, EMPTY, EMPTY, EMPTY, EMPTY, B7],
            blacks: vec![EMPTY, EMPTY, H8, F7, EMPTY, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        castle_moves: vec![],

        enpassant_moves: vec![],
        enpassant_attacks: vec![],

        promotion_moves: vec![(G7, G8 | H8)],
        promotion_attacks: vec![(G7, H8)],

        standard_moves: vec![(C5, C6), (B7, B8 | C8 | A8 | A7 | A6 | B6 | C6)],
        standard_attacks: vec![],
    });
}
