use myopic_core::Reflectable;
use crate::{Board, ChessBoard, TerminalState};

fn test_fen(expected: Option<TerminalState>, fen: &str) {
    let board = fen.parse::<Board>().unwrap();
    assert_eq!(expected, board.terminal_state());
    assert_eq!(expected, board.reflect().terminal_state());
}

fn test_pgn(expected: Option<TerminalState>, pgn: &str) {
    let mut board = Board::default();
    board.play_pgn(pgn).unwrap();
    assert_eq!(expected, board.terminal_state());
}

#[test]
fn checkmate() {
    test_fen(Some(TerminalState::Loss), "5R1k/pp2R2p/8/1b2r3/3p3q/8/PPB3P1/6K1 b - - 0 36");
}

#[test]
fn not_terminal() {
    test_fen(None, "r1b1qrk1/pp5p/1np2b2/3nNP2/3P2p1/1BN5/PP1BQ1P1/4RRK1 b - - 0 18");
}

#[test]
fn not_terminal2() {
    test_fen(None, "4R3/1p4rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2Q4 b - - 0 2");
}

#[test]
fn not_terminal3() {
    test_fen(None, "2R2bk1/5p1p/5p1P/3N4/3K2P1/8/8/3r4 w - - 51 100");
}

#[test]
fn not_terminal4() {
    test_fen(None, "8/1p3B2/1n6/p3Pkp1/3P1pPp/1K3P1P/8/8 b - g3 0 41");
}

#[test]
fn stalemate() {
    test_fen(Some(TerminalState::Draw), "6k1/6p1/7p/8/1p6/p1qp4/8/3K4 w - - 0 45");
}

#[test]
fn fifty_moves_1() {
    test_fen(Some(TerminalState::Draw), "8/8/8/8/3B4/7K/2k1Q3/1q6 b - - 100 120")
}

#[test]
fn repetition_1() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5")
}

#[test]
fn repetition_2() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5")
}

#[test]
fn repetition_3() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3")
}

#[test]
fn repetition_4() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7")
}

#[test]
fn repetition_5() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7 12. Bd2")
}

#[test]
fn repetition_6() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7 12. Bd2 Bc5")
}

#[test]
fn repetition_7() {
    test_pgn(None, "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 5. Re1 Nd6 6. Nxe5 Be7 \
        7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7 12. Bd2 Bc5 12. Be3")
}

#[test]
fn repetition_8() {
    test_pgn(Some(TerminalState::Draw), "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 \
        5. Re1 Nd6 6. Nxe5 Be7 7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7 \
        12. Bd2 Bc5 13. Be3 Bb4 14. Bd2 Bc5 15. Be3")
}

#[test]
fn repetition_9() {
    test_pgn(Some(TerminalState::Draw), "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Nxe4 \
        5. Re1 Nd6 6. Nxe5 Be7 7. Bf1 Nxe5 8. Rxe5 O-O 9. d4 Ne8 10. d5 Bc5 11. Be3 Be7 \
        12. Bd2 Bc5 13. Be3 Bb4 14. Bd2 Bc5 15. Be3 d6")
}
