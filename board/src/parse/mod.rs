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
