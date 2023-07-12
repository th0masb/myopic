use std::array;
use lazy_static::lazy_static;
use crate::{Corner, Piece, Side, SideMap, Square, SquareMap};

lazy_static! {
    static ref FEATURES: Features = compute_features();
}

/// Get the hash of the given piece sat on the given square
pub fn piece((side, class): Piece, square: Square) -> u64 {
    FEATURES.piece_squares[side][class][square]
}

/// Get the hash of the given side to move
pub fn side(side: Side) -> u64 {
    FEATURES.side[side]
}

/// Get the hash of enpassant on the file of the given square
pub fn enpassant(square: Square) -> u64 {
    FEATURES.enpassant[square]
}

/// Get the hash of the given castling zone
pub fn zone((side, flank): Corner) -> u64 {
    FEATURES.corner[side][flank]
}

fn compute_features() -> Features {
    let mut prng = PRNG { s: 1070372 };
    Features {
        side: [0, prng.rand64()],
        enpassant: array::from_fn(|_| prng.rand64()),
        corner: array::from_fn(|_| [prng.rand64(), prng.rand64()]),
        piece_squares: array::from_fn(|_| array::from_fn(|_| array::from_fn(|_| prng.rand64()))),
    }
}

struct Features {
    side: SideMap<u64>,
    enpassant: SquareMap<u64>,
    corner: SideMap<[u64; 2]>,
    piece_squares: [[[u64; 64]; 6]; 2],
}

// https://github.com/official-stockfish/Stockfish/blob/master/src/misc.h#L122
struct PRNG { s: u64 }

impl PRNG {
    fn rand64(&mut self) -> u64 {
        self.s ^= self.s.wrapping_shr(12);
        self.s ^= self.s.wrapping_shl(25);
        self.s ^= self.s.wrapping_shr(27);
        self.s.wrapping_mul(2685821657736338717u64)
    }
}
