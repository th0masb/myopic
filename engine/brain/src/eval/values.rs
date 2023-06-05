use serde_derive::{Deserialize, Serialize};

use myopic_board::{Piece, Side};

/// Values copied from Stockfish: https://github.com/official-stockfish/Stockfish/blob/master/src/types.h
const DEFAULT_MIDGAME: [i32; 6] = [128, 782, 830, 1289, 2529, 100_000];
const DEFAULT_ENDGAME: [i32; 6] = [213, 865, 918, 1378, 2687, 100_000];

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq)]
pub struct PieceValues {
    pub midgame: [i32; 6],
    pub endgame: [i32; 6],
}

impl Default for PieceValues {
    fn default() -> Self {
        PieceValues {
            midgame: DEFAULT_MIDGAME,
            endgame: DEFAULT_ENDGAME,
        }
    }
}

impl PieceValues {
    pub fn new(midgame: [i32; 6], endgame: [i32; 6]) -> PieceValues {
        PieceValues { midgame, endgame }
    }

    /// Retrieve the midgame value of the given piece.
    pub fn midgame(&self, piece: Piece) -> i32 {
        parity(piece) * self.midgame[(piece as usize) % 6]
    }

    /// Retrieve the endgame value of the given piece.
    pub fn endgame(&self, piece: Piece) -> i32 {
        parity(piece) * self.endgame[(piece as usize) % 6]
    }
}

fn parity(piece: Piece) -> i32 {
    match piece.side() {
        Side::W => 1,
        Side::B => -1,
    }
}
