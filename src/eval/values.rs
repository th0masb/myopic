use crate::base::Side;
use crate::pieces::Piece;

/// Retrieve the midgame value of the given piece.
pub fn midgame(piece: Piece) -> i32 {
    parity(piece) * MIDGAME[(piece as usize) % 6]
}

/// Retrieve the absolute value of the given piece in the midgame.
pub fn abs_midgame(piece: Piece) -> i32 {
    MIDGAME[(piece as usize) % 6]
}

/// Retrieve the endgame value of the given piece.
pub fn endgame(piece: Piece) -> i32 {
    parity(piece) * ENDGAME[(piece as usize) % 6]
}

fn parity(piece: Piece) -> i32 {
    match piece.side() {
        Side::White => 1,
        Side::Black => -1,
    }
}

/// Values copied from Stockfish: https://github.com/official-stockfish/Stockfish/blob/master/src/types.h
const MIDGAME: [i32; 6] = [128, 782, 830, 1289, 2529, 0];
const ENDGAME: [i32; 6] = [213, 865, 918, 1378, 2687, 0];
