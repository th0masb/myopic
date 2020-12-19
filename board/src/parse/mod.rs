pub(crate) mod patterns;
mod pgn;
mod uci;

use crate::MutBoardImpl;
pub use pgn::{partial_pgn, pgn};
pub use uci::{partial_uci, uci};

/// Return the position generated from applying the moves encoded in the
/// given string sequentially to the standard start position.
pub fn position_from_pgn(pgn_moves: &str) -> Result<MutBoardImpl, String> {
    let mut board = crate::start_position();
    for mv in pgn(pgn_moves)? {
        board.evolve(&mv);
    }
    Ok(board)
}

/// Return the position generated from applying the moves encoded in the
/// given string sequentially to the standard start position.
pub fn position_from_uci(uci_moves: &str) -> Result<MutBoardImpl, String> {
    let mut board = crate::start_position();
    for mv in uci(uci_moves)? {
        board.evolve(&mv);
    }
    Ok(board)
}

#[cfg(test)]
mod test {
    use crate::{MutBoard};
    

    #[test]
    fn uci_position_1() {
        let seq = "e2e4 e7e6 d2d4 d7d5 b1d2 c7c5 e4d5 c5d4 f1b5 c8d7 d5e6 f7e6 b5d7 d8d7 \
        g1f3 b8c6 e1g1 g8f6 d2c4 g7g6 c1g5 f6e4 g5f4 e8c8";
        assert_eq!(
            "2kr1b1r/pp1q3p/2n1p1p1/8/2NpnB2/5N2/PPP2PPP/R2Q1RK1 w - - 4 13",
            super::position_from_uci(seq).unwrap().to_fen().as_str()
        )
    }

}