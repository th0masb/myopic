pub(crate) mod patterns;
pub mod pgn;
pub mod uci;

#[cfg(test)]
mod test {
    use crate::ChessBoard;

    #[test]
    fn uci_position_1() {
        let mut board = crate::start();
        board
            .play_uci(
                "e2e4 e7e6 d2d4 d7d5 b1d2 c7c5 e4d5 c5d4 f1b5 c8d7 d5e6 f7e6 b5d7 d8d7 \
        g1f3 b8c6 e1g1 g8f6 d2c4 g7g6 c1g5 f6e4 g5f4 e8c8",
            )
            .unwrap();
        assert_eq!(
            "2kr1b1r/pp1q3p/2n1p1p1/8/2NpnB2/5N2/PPP2PPP/R2Q1RK1 w - - 4 13",
            board.to_fen().as_str()
        )
    }
}
