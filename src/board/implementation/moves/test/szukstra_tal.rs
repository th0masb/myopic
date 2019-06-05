use super::*;

#[test]
fn black_move_eight() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | C4 | D4 | E4 | F3 | G2 | H2,
                C3 | E2,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | E5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 1,
            hash_offset: 14,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A7, A6 | A5),
            (B8, A6 | D7),
            (B7, B6 | B5),
            (C8, D7 | E6 | F5 | G4 | H3),
            (C6, C5),
            (D8, C7 | B6 | A5 | D7 | E8 | E7),
            (D6, D5),
            (E5, D4),
            (F8, E8),
            (F6, D5 | E4 | D7 | E8 | H5 | G4),
            (G8, H8),
            (G7, H8 | H6),
            (G6, G5),
            (H7, H6 | H5),
        ],

        standard_attacks: vec![(E5, D4), (F6, E4)],
    });
}

#[test]
fn white_move_nine() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | E2,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | D4 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![CastleZone::WQ],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A1, B1 | C1 | D1),
            (A2, A3 | A4),
            (C3, B1 | A4 | B5 | D5 | D1),
            (C4, C5),
            (B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7),
            (E4, E5),
            (E1, D2 | F2 | D1),
            (E2, D4 | C1 | G1 | G3 | F4),
            (E3, D4 | D2 | C1 | F2 | G1 | F4 | G5 | H6),
            (F3, F4),
            (G2, G3 | G4),
            (H1, G1),
            (H2, H3 | H4),
        ],

        standard_attacks: vec![(B3, B7), (E2, D4), (E3, D4)],
    });
}

#[test]
fn black_move_nine() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A7, A6 | A5),
            (B8, A6 | D7),
            (B7, B6 | B5),
            (C8, D7 | E6 | F5 | G4 | H3),
            (C6, C5),
            (D8, C7 | B6 | A5 | D7 | E8 | E7),
            (D6, D5),
            (F8, E8),
            (F6, D5 | E4 | D7 | E8 | H5 | G4),
            (G8, H8),
            (G7, H8 | H6),
            (G6, G5),
            (H7, H6 | H5),
        ],

        standard_attacks: vec![(F6, E4)],
    });
}

#[test]
fn white_move_ten() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![CastleZone::WQ],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A1, B1 | C1 | D1),
            (A2, A3 | A4),
            (C3, E2 | B1 | A4 | B5 | D5 | D1),
            (C4, C5 | D5),
            (B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7),
            (E4, E5 | D5),
            (E1, E2 | D2 | F2 | D1),
            (D4, C2 | E2 | F5 | E6 | B5 | C6),
            (E3, D2 | C1 | F2 | G1 | F4 | G5 | H6),
            (F1, E2 | D3),
            (F3, F4),
            (G2, G3 | G4),
            (H1, G1),
            (H2, H3 | H4),
        ],

        standard_attacks: vec![(B3, B7), (D4, C6), (C4, D5), (E4, D5), (C3, D5)],
    });
}

#[test]
fn black_move_ten() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | D5 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A7, A6 | A5),
            (B8, A6 | D7),
            (B7, B6 | B5),
            (C8, D7 | E6 | F5 | G4 | H3),
            (C6, D5 | C5),
            (D8, D5 | D6 | C7 | B6 | A5 | D7 | E8 | E7),
            (F8, E8),
            (F6, D5 | E4 | D7 | E8 | H5 | G4),
            (G8, H8),
            (G7, H8 | H6),
            (G6, G5),
            (H7, H6 | H5),
        ],

        standard_attacks: vec![(F6, E4), (C6, D5), (F6, D5), (D8, D5)],
    });
}

#[test]
fn white_move_eleven() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | D5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![CastleZone::WQ],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A1, B1 | C1 | D1),
            (A2, A3 | A4),
            (B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7 | C4 | D5),
            (C3, E2 | B1 | A4 | B5 | D5 | D1),
            (D4, C2 | E2 | F5 | E6 | B5 | C6),
            (E4, E5 | D5),
            (E1, E2 | D2 | F2 | D1),
            (E3, D2 | C1 | F2 | G1 | F4 | G5 | H6),
            (F1, E2 | D3 | C4 | B5 | A6),
            (F3, F4),
            (G2, G3 | G4),
            (H1, G1),
            (H2, H3 | H4),
        ],

        standard_attacks: vec![(B3, B7 | D5), (E4, D5), (C3, D5)],
    });
}

#[test]
fn black_move_eleven() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | D5 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![A7 | B7 | F7 | G6 | H7, B8 | F6, C8 | G7, A8 | F8, D8, G8],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A7, A6 | A5),
            (B8, A6 | D7 | C6),
            (B7, B6 | B5),
            (C8, D7 | E6 | F5 | G4 | H3),
            (D8, D5 | D6 | D7 | C7 | B6 | A5 | E8 | E7),
            (F8, E8),
            (F6, D5 | E4 | D7 | E8 | H5 | G4),
            (G8, H8),
            (G7, H8 | H6),
            (G6, G5),
            (H7, H6 | H5),
        ],

        standard_attacks: vec![(F6, D5), (F6, D5), (D8, D5)],
    });
}

#[test]
fn white_move_twelve() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | D5 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![A7 | B7 | F7 | G6 | H7, C6 | F6, C8 | G7, A8 | F8, D8, G8],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        castle_moves: vec![CastleZone::WQ],
        enpassant_moves: vec![],
        enpassant_attacks: vec![],
        promotion_moves: vec![],
        promotion_attacks: vec![],
        standard_moves: vec![
            (A1, B1 | C1 | D1),
            (A2, A3 | A4),
            (B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7 | C4),
            (C3, E2 | B1 | A4 | B5 | E4 | D1),
            (D4, C2 | E2 | F5 | E6 | B5 | C6),
            (D5, C6 | D6),
            (E1, E2 | D2 | F2 | D1),
            (E3, D2 | C1 | F2 | G1 | F4 | G5 | H6),
            (F1, E2 | D3 | C4 | B5 | A6),
            (F3, F4),
            (G2, G3 | G4),
            (H1, G1),
            (H2, H3 | H4),
        ],

        standard_attacks: vec![(B3, B7), (D4, C6), (D5, C6)],
    });
}
