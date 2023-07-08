use itertools::Itertools;
use lazy_static::lazy_static;
use std::num::Wrapping;

use crate::pieces::Piece;
use crate::square::Square;
use crate::{Corner, Side};

// Total number of hashing features
const N_FEATURES: usize = 64 * 12 + 8 + 4 + 1;

lazy_static! {
    static ref FEATURES: [u64; N_FEATURES] = compute_features();
}

fn compute_features() -> [u64; N_FEATURES] {
    let mut prng = PRNG { s: 1070372 };
    let result = std::array::from_fn(|_| prng.rand64());
    #[cfg(debug_assertions)]
    assert_eq!(N_FEATURES, result.iter().dedup().count());
    result
}

// https://github.com/official-stockfish/Stockfish/blob/master/src/misc.h#L122
struct PRNG {
    s: u64,
}

impl PRNG {
    fn rand64(&mut self) -> u64 {
        self.s ^= self.s.wrapping_shr(12);
        self.s ^= self.s.wrapping_shl(25);
        self.s ^= self.s.wrapping_shr(27);
        self.s.wrapping_mul(2685821657736338717u64)
    }
}

/// Get the hash of the given piece sat on the given square
pub fn piece(Piece(side, class): Piece, square: Square) -> u64 {
    FEATURES[((side as usize) * 6 + class as usize) * 64 + (square as usize)]
}

/// Get the hash of the given side to move
pub fn side(side: Side) -> u64 {
    match side {
        Side::B => FEATURES[N_FEATURES - 1],
        Side::W => 0,
    }
}

/// Get the hash of enpassant on the file of the given square
pub fn enpassant(square: Square) -> u64 {
    FEATURES[N_FEATURES - 6 - square.file_index()]
}

/// Get the hash of the given castling zone
pub fn zone(Corner(side, flank): Corner) -> u64 {
    FEATURES[N_FEATURES - 2 - (2 * side as usize + flank as usize)]
}
