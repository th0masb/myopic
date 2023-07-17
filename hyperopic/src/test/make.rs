use crate::constants::square::*;
use crate::constants::{corner, piece, side};
use crate::moves::Move;
use crate::position::Position;

fn execute_test(from_fen: &str, m: Move, dest_fen: &str) {
    let mut from: Position = from_fen.parse().unwrap();
    let mut dest: Position = dest_fen.parse().unwrap();
    dest.history.push((from.create_discards(), m.clone()));
    let from_clone = from.clone();
    from.make(m.clone()).unwrap();
    assert_eq!(from, dest);
    from.unmake().unwrap();
    assert_eq!(from, from_clone);
}

#[test]
fn white_kingside_castle() {
    execute_test(
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Castle { corner: corner::WK },
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R4RK1 b kq - 1 1",
    );
}

#[test]
fn white_queenside_castle() {
    execute_test(
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Castle { corner: corner::WQ },
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/2KR3R b kq - 1 1",
    );
}

#[test]
fn black_kingside_castle() {
    execute_test(
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R b KQkq - 0 1",
        Move::Castle { corner: corner::BK },
        "r4rk1/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQ - 1 2",
    );
}

#[test]
fn black_queenside_castle() {
    execute_test(
        "r3k2r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R b KQkq - 0 1",
        Move::Castle { corner: corner::BQ },
        "2kr3r/p2qpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQ - 1 2",
    );
}

#[test]
fn test_white_rook_taking_black_rook_removing_kingside_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Normal { moving: piece::WR, from: H1, dest: H8, capture: Some(piece::BR) },
        "r3k2R/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K3 b Qq - 0 1",
    );
}

#[test]
fn test_black_rook_taking_white_rook_removing_kingside_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R b KQkq - 0 1",
        Move::Normal { moving: piece::BR, from: H8, dest: H1, capture: Some(piece::WR) },
        "r3k3/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2r w Qq - 0 2",
    );
}

#[test]
fn test_white_rook_taking_black_rook_removing_queenside_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Normal { moving: piece::WR, from: A1, dest: A8, capture: Some(piece::BR) },
        "R3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/4K2R b Kk - 0 1",
    );
}

#[test]
fn test_black_rook_taking_white_rook_removing_queenside_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R b KQkq - 0 1",
        Move::Normal { moving: piece::BR, from: A8, dest: A1, capture: Some(piece::WR) },
        "4k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/r3K2R w Kk - 0 2",
    );
}

#[test]
fn test_white_king_moving_removes_castling_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQkq - 0 1",
        Move::Normal { moving: piece::WK, from: E1, dest: E2, capture: None },
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q1KPP1/R6R b kq - 1 1",
    );
}

#[test]
fn test_black_king_moving_removes_castling_rights() {
    execute_test(
        "r3k2r/4pp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R b KQkq - 0 1",
        Move::Normal { moving: piece::BK, from: E8, dest: D7, capture: None },
        "r6r/3kpp2/1n1b4/2p5/2B5/1N6/2Q2PP1/R3K2R w KQ - 1 2",
    );
}

#[test]
fn test_white_pawn_moves_forward_two() {
    execute_test(
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP3/2N5/PPP2PPP/R2QKB1R w KQkq - 12 1",
        Move::Normal { moving: piece::WP, from: G2, dest: G4, capture: None },
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP1P1/2N5/PPP2P1P/R2QKB1R b KQkq g3 0 1",
    );
}

#[test]
fn test_black_pawn_moves_forward_two() {
    execute_test(
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP1P1/2N5/PPP2P1P/R2QKB1R b KQkq - 12 1",
        Move::Normal { moving: piece::BP, from: B7, dest: B5, capture: None },
        "r1bqkb1r/p3pppp/2np1n2/1p4B1/3NP1P1/2N5/PPP2P1P/R2QKB1R w KQkq b6 0 2",
    );
}

#[test]
fn test_white_pawn_moves_forward_one() {
    execute_test(
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP3/2N5/PPP2PPP/R2QKB1R w KQkq - 12 1",
        Move::Normal { moving: piece::WP, from: G2, dest: G3, capture: None },
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP3/2N3P1/PPP2P1P/R2QKB1R b KQkq - 0 1",
    );
}

#[test]
fn test_black_pawn_moves_forward_one() {
    execute_test(
        "r1bqkb1r/pp2pppp/2np1n2/6B1/3NP3/2N3P1/PPP2P1P/R2QKB1R b KQkq - 11 1",
        Move::Normal { moving: piece::BP, from: B7, dest: B6, capture: None },
        "r1bqkb1r/p3pppp/1pnp1n2/6B1/3NP3/2N3P1/PPP2P1P/R2QKB1R w KQkq - 0 2",
    );
}

#[test]
fn test_white_enpassant() {
    execute_test(
        "r1bqkb1r/p3p1pp/2np3B/3nPp2/3N3P/P1N3P1/2P2P2/R2QKB1R w KQkq f6 12 7",
        Move::Enpassant { side: side::W, from: E5, dest: F6, capture: F5 },
        "r1bqkb1r/p3p1pp/2np1P1B/3n4/3N3P/P1N3P1/2P2P2/R2QKB1R b KQkq - 0 7",
    );
}

#[test]
fn test_black_enpassant() {
    execute_test(
        "r1bqkb1r/p3pppp/2np1n2/4P1B1/Pp1N3P/2N3P1/1PP2P2/R2QKB1R b KQkq a3 12 4",
        Move::Enpassant { side: side::B, from: B4, dest: A3, capture: A4 },
        "r1bqkb1r/p3pppp/2np1n2/4P1B1/3N3P/p1N3P1/1PP2P2/R2QKB1R w KQkq - 0 5",
    );
}

#[test]
fn test_white_promotion() {
    execute_test(
        "r1bqkb1r/p5Pp/2np3B/4p3/1n1N3P/P1N3P1/2P2P2/R2QKB1R w KQkq - 11 9",
        Move::Promote { from: G7, dest: H8, promoted: piece::WR, capture: Some(piece::BR) },
        "r1bqkb1R/p6p/2np3B/4p3/1n1N3P/P1N3P1/2P2P2/R2QKB1R b KQq - 0 9",
    );
}

#[test]
fn test_black_promotion() {
    execute_test(
        "r1bqkb1R/p6p/2np4/8/1n1N3P/P1N1B1P1/2P1pP2/R1KQ1B1R b q - 1 12",
        Move::Promote { from: E2, dest: E1, promoted: piece::BN, capture: None },
        "r1bqkb1R/p6p/2np4/8/1n1N3P/P1N1B1P1/2P2P2/R1KQnB1R w q - 0 13",
    );
}
