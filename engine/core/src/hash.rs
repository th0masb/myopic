use enum_map::{enum_map, EnumMap};
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::pieces::Piece;
use crate::{Class, File, Flank, Square};
use crate::{Corner, Side};

/// Get the hash of the given piece sat on the given square
pub fn piece(Piece(side, class): Piece, square: Square) -> u64 {
    FEATURES.piece_square[side][class][square]
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
pub fn zone(Corner(side, flank): Corner) -> u64 {
    FEATURES.corner[side][flank]
}

struct Features {
    piece_square: EnumMap<Side, EnumMap<Class, EnumMap<Square, u64>>>,
    side: EnumMap<Side, u64>,
    enpassant: EnumMap<Square, u64>,
    corner: EnumMap<Side, EnumMap<Flank, u64>>,
}

lazy_static! {
    static ref FEATURES: Features = compute_features();
}

fn compute_features() -> Features {
    let mut prng = PRNG { s: 1070372 };
    Features {
        piece_square: enum_map! { _ => enum_map! { _ => enum_map! { _ => prng.rand64() } } },
        side: enum_map! { Side::W => 0, Side::B => prng.rand64() },
        enpassant: enum_map! { _ => prng.rand64() },
        corner: enum_map! { _ => enum_map! { _ => prng.rand64() } },
    }
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
