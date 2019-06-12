use super::*;
const EMPTY: BitBoard = BitBoard::EMPTY;

#[test]
fn case_1() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![B3 | F5 | F2 | G2, C3, A3 | F3, A1 | H1, C2, E1],
            blacks: vec![C6 | E5 | F7 | G7 | H7, B8, B5 | G6, A8 | H8, C7, D8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            enpassant: Some(sq(E6)),
        },

        expected_all: vec![
            s(Piece::WR, A1, A2 | B1 | C1 | D1),
            s(Piece::WB, A3, B2 | C1 | B4 | C5 | D6 | E7 | F8),
            s(Piece::WP, B3, B4),
            s(Piece::WN, C3, A2 | A4 | B5 | D5 | E4 | E2 | B1 | D1),
            s(Piece::WQ, C2, B2 | B1 | A2 | C1 | D1 | D2 | E2 | D3 | E4),
            s(Piece::WK, E1, D1 | D2),
            s(Piece::WB, F3, E2 | D1 | E4 | D5 | C6 | G4 | H5),
            s(Piece::WP, F5, F6 | G6),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1 | F1 | H2 | H3 | H4 | H5 | H6 | H7),
            e(F5),
            c(CastleZone::WQ.lift()),
        ],

        expected_attacks: vec![
            s(Piece::WN, C3, B5),
            s(Piece::WB, F3, C6),
            s(Piece::WP, F5, G6),
            s(Piece::WR, H1, H7),
            e(F5),
        ],

        expected_attacks_checks: vec![
            s(Piece::WR, A1, D1),
            s(Piece::WB, A3, E7),
            s(Piece::WN, C3, B5),
            s(Piece::WB, F3, C6),
            s(Piece::WQ, C2, D1 | D2 | D3),
            s(Piece::WP, F5, G6),
            s(Piece::WR, H1, H7),
            e(F5),
        ],
    });
}

#[test]
fn case_2() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![A7 | D4 | F2 | G2, C3, A4, B1 | H1, BitBoard::EMPTY, E1],
            blacks: vec![
                A2 | D5 | F7 | G7 | H7,
                C6,
                G6 | A5,
                B8 | H8,
                BitBoard::EMPTY,
                C7,
            ],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::WK | CastleZoneSet::BK,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::WB, A4, B3 | C2 | D1 | B5 | C6),
            s(
                Piece::WR,
                B1,
                B2 | B3 | B4 | B5 | B6 | B7 | B8 | A1 | C1 | D1,
            ),
            s(Piece::WK, E1, E2 | D1 | D2 | F1),
            s(Piece::WP, F2, F3 | F4),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1 | F1 | H2 | H3 | H4 | H5 | H6 | H7),
            c(CastleZone::WK.lift()),
            p(Side::White, A7, A8 | B8),
        ],

        expected_attacks: vec![
            s(Piece::WB, A4, C6),
            s(Piece::WR, B1, B8),
            s(Piece::WR, H1, H7),
            p(Side::White, A7, B8),
        ],

        expected_attacks_checks: vec![
            s(Piece::WB, A4, C6),
            s(Piece::WR, B1, B7 | B8),
            s(Piece::WR, H1, H7),
            p(Side::White, A7, B8),
        ],
    });
}

#[test]
fn case_3() {
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

        expected_all: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],
        expected_attacks: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],

        expected_attacks_checks: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],
    });
}
#[test]
fn case_4() {
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

        expected_all: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],
        expected_attacks: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],

        expected_attacks_checks: vec![
            s(Piece::WN, C3, B5 | E2),
            s(Piece::WQ, C2, D3 | E2),
            s(Piece::WB, F3, E2),
            s(Piece::WR, E1, E2),
            s(Piece::WK, F1, G1),
        ],
    });
}

#[test]
fn case_5() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![B3 | F2 | G2, C3, F3, A1 | E1, C2, F1],
            blacks: vec![C6 | F7 | G7, BitBoard::EMPTY, B5 | G6, A8 | H8, C7, E8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::BK | CastleZoneSet::BQ,
            white_status: Some(CastleZone::WK),
            black_status: None,
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::BB, B5, E2),
            s(Piece::BB, G6, E4),
            s(Piece::BQ, C7, E7 | E5),
            s(Piece::BK, E8, D7 | D8 | F8),
        ],

        expected_attacks: vec![
            s(Piece::BB, B5, E2),
            s(Piece::BB, G6, E4),
            s(Piece::BQ, C7, E7 | E5),
            s(Piece::BK, E8, D7 | D8 | F8),
        ],

        expected_attacks_checks: vec![
            s(Piece::BB, B5, E2),
            s(Piece::BB, G6, E4),
            s(Piece::BQ, C7, E7 | E5),
            s(Piece::BK, E8, D7 | D8 | F8),
        ],
    });
}

#[test]
fn case_6() {
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

        expected_all: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
        expected_attacks: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
        expected_attacks_checks: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
    });
}

#[test]
fn case_7() {
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

        expected_all: vec![
            e(D5),
            e(F5),
            s(Piece::WP, D5, D6),
            s(Piece::WP, F5, F6),
            s(Piece::WK, H1, G1 | G2 | H2),
        ],
        expected_attacks: vec![e(D5), e(F5)],
        expected_attacks_checks: vec![e(D5), e(F5)],
    });
}

#[test]
fn case_8() {
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

        expected_all: vec![
            p(Side::White, G7, G8 | H8),
            s(Piece::WP, C5, C6),
            s(Piece::WK, B7, B8 | A8 | A7 | A6 | B6 | C6 | C8),
        ],
        expected_attacks: vec![p(Side::White, G7, H8)],
        expected_attacks_checks: vec![p(Side::White, G7, H8)],
    });
}

#[test]
fn case_10() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![C5, EMPTY, EMPTY, EMPTY, EMPTY, B6],
            blacks: vec![D5, EMPTY, E3, EMPTY, EMPTY, G8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            enpassant: Some(sq(D6)),
        },

        expected_all: vec![s(Piece::WK, B6, B5 | C6 | C7 | B7 | A7 | A6 | A5)],
        expected_attacks: vec![],
        expected_attacks_checks: vec![],
    });
}


#[test]
fn case_11() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![A2 | C2 | D3 | E5 | G4 | H5, D5 | G7, E7, EMPTY, EMPTY, E2],
            blacks: vec![A7 | B5 | D7 | F7 | H7, A6 | F6, C8 | G1, A8 | H8, A1, D8],
            clock: 20,
            hash_offset: 20,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            enpassant: None,
        },

        expected_all: vec![],
        expected_attacks: vec![],
        expected_attacks_checks: vec![],
    });
}
