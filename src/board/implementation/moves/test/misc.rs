use super::*;
const EMPTY: BitBoard = BitBoard::EMPTY;

#[test]
fn case_1() {
    execute_test(TestCase {
        board: "rn1k3r/2q2ppp/2p3b1/1b2pP2/8/BPN2B2/2Q2PP1/R3K2R w KQkq e6 3 10",
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
            e(F5, Square::E6),
            c(CastleZone::WQ.lift()),
        ],
        expected_attacks: vec![
            s(Piece::WN, C3, B5),
            s(Piece::WB, F3, C6),
            s(Piece::WP, F5, G6),
            s(Piece::WR, H1, H7),
            e(F5, Square::E6),
        ],
        expected_attacks_checks: vec![
            s(Piece::WR, A1, D1),
            s(Piece::WB, A3, E7),
            s(Piece::WN, C3, B5),
            s(Piece::WB, F3, C6),
            s(Piece::WQ, C2, D1 | D2 | D3),
            s(Piece::WP, F5, G6),
            s(Piece::WR, H1, H7),
            e(F5, Square::E6),
        ],
    });
}

#[test]
fn case_2() {
    execute_test(TestCase {
        board: "1r5r/P1k2ppp/2n3b1/b2p4/B2P4/2N5/p4PP1/1R2K2R w Kk - 5 20",
        expected_all: vec![
            s(Piece::WB, A4, B3 | C2 | D1 | B5 | C6),
            s(Piece::WR, B1, B2 | B3 | B4 | B5 | B6 | B7 | B8 | A1 | C1 | D1),
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
            p(Side::White, A7, A8 | B8),
        ],
    });
}

#[test]
fn case_3() {
    execute_test(TestCase {
        board: "r3k2r/2q2pp1/2p3b1/1b6/8/1PN2B2/2Q2PP1/R3RK2 w kq - 3 10",
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
        board: "r3k2r/2q2pp1/2p3b1/1b6/8/1PN2B2/2Q2PP1/R3RK2 b kq - 3 10",
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
        board: "8/8/8/8/8/2k1R3/8/B6K b - - 3 10",
        expected_all: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
        expected_attacks: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
        expected_attacks_checks: vec![s(Piece::BK, C3, C2 | D2 | C4 | B4)],
    });
}

#[test]
fn case_7() {
    execute_test(TestCase {
        board: "7k/8/8/3PpP2/8/8/8/7K w - e6 3 10",
        expected_all: vec![
            e(D5, Square::E6),
            e(F5, Square::E6),
            s(Piece::WP, D5, D6),
            s(Piece::WP, F5, F6),
            s(Piece::WK, H1, G1 | G2 | H2),
        ],
        expected_attacks: vec![e(D5, Square::E6), e(F5, Square::E6)],
        expected_attacks_checks: vec![e(D5, Square::E6), e(F5, Square::E6)],
    });
}

#[test]
fn case_8() {
    execute_test(TestCase {
        board: "4k2b/1KP2rP1/8/2P5/8/8/8/8 w - - 3 10",
        expected_all: vec![
            p(Side::White, G7, G8 | H8),
            s(Piece::WP, C5, C6),
            s(Piece::WK, B7, B8 | A8 | A7 | A6 | B6 | C6 | C8),
        ],
        expected_attacks: vec![p(Side::White, G7, H8)],
        expected_attacks_checks: vec![p(Side::White, G7, H8 | G8)],
    });
}

#[test]
fn case_10() {
    execute_test(TestCase {
        board: "6k1/8/1K6/2Pp4/8/4b3/8/8 w - - 3 10",
        expected_all: vec![s(Piece::WK, B6, B5 | C6 | C7 | B7 | A7 | A6 | A5)],
        expected_attacks: vec![],
        expected_attacks_checks: vec![],
    });
}

#[test]
fn case_11() {
    execute_test(TestCase {
        board: "r1bk2br/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q7 b - - 3 10",
        expected_all: vec![],
        expected_attacks: vec![],
        expected_attacks_checks: vec![],
    });
}

#[test]
fn case_12() {
    execute_test(TestCase {
        board: "6rk/5p2/4pPp1/8/8/6P1/6PK/7R w - - 3 10",
        expected_all: vec![
            s(Piece::WK, H2, H3 | G1),
            s(Piece::WP, G3, G4),
            s(Piece::WR, H1, G1 | F1 | E1 | D1 | C1 | B1 | A1),
        ],
        expected_attacks: vec![],
        expected_attacks_checks: vec![s(Piece::WK, H2, G1)],
    })
}

#[test]
fn case_13() {
    execute_test(TestCase {
        board: "4R3/1p1Q2rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2q4 w - - 2 2",
        expected_all: vec![s(Piece::WQ, D7, D1), s(Piece::WK, A1, A2)],
        expected_attacks: vec![s(Piece::WQ, D7, D1), s(Piece::WK, A1, A2)],
        expected_attacks_checks: vec![s(Piece::WQ, D7, D1), s(Piece::WK, A1, A2)],
    })
}

#[test]
fn case_14() {
    execute_test(TestCase {
        board: "8/8/3p4/2pP3R/2Pk1pPQ/3B4/2K2P2/8 b - g3 0 1",
        expected_all: vec![s(Piece::BP, F4, F3)],
        expected_attacks: vec![],
        expected_attacks_checks: vec![],
    })
}

#[test]
fn case_15() {
    execute_test(TestCase {
        board: "1k6/b2P1r2/p5p1/4qp1p/7P/7K/8/8 w - - 0 1",
        expected_all: vec![p(Side::White, D7, D8), s(Piece::WK, H3, G2)],
        expected_attacks: vec![],
        expected_attacks_checks: vec![p(Side::White, D7, D8)],
    })
}

#[test]
fn case_16() {
    execute_test(TestCase {
        board: "8/bk1P1r2/p5p1/4qp1p/7P/7K/8/8 w - - 0 1",
        expected_all: vec![p(Side::White, D7, D8), s(Piece::WK, H3, G2)],
        expected_attacks: vec![],
        expected_attacks_checks: vec![p(Side::White, D7, D8)],
    })
}

#[test]
fn case_17() {
    execute_test(TestCase {
        board: "1r4rk/5pBp/ppbp1P2/5P2/2P3R1/3n3p/PP5P/5NK1 b - - 1 3",
        expected_all: vec![s(Piece::BR, G8, G7)],
        expected_attacks: vec![s(Piece::BR, G8, G7)],
        expected_attacks_checks: vec![s(Piece::BR, G8, G7)],
    })
}