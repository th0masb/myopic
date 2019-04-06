use std::ops;

use crate::base::bitboard::{BitBoard, loc};
use crate::base::square::Square;

/// Operator implementations for bitboards which all use the underlying u64 value.
impl ops::Shr<u8> for BitBoard {
    type Output = Self;

    fn shr(self, shift: u8) -> Self {
        BitBoard(self.0 >> shift)
    }
}

impl ops::Shl<u8> for BitBoard {
    type Output = Self;

    fn shl(self, shift: u8) -> Self {
        BitBoard(self.0 << shift)
    }
}

impl ops::Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl ops::Sub<BitBoard> for BitBoard {
    type Output = Self;

    fn sub(self, other: BitBoard) -> Self {
        BitBoard(self.0 & !other.0)
    }
}

impl ops::Sub<Square> for BitBoard {
    type Output = Self;

    fn sub(self, other: Square) -> Self {
        BitBoard(self.0 & !loc(other))
    }
}

impl ops::BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, other: BitBoard) -> Self {
        BitBoard(self.0 ^ other.0)
    }
}

impl ops::BitOr<BitBoard> for BitBoard {
    type Output = Self;

    fn bitor(self, other: BitBoard) -> Self {
        BitBoard(self.0 | other.0)
    }
}

impl ops::BitOr<Square> for BitBoard {
    type Output = Self;

    fn bitor(self, other: Square) -> Self {
        BitBoard(self.0 | loc(other))
    }
}

impl ops::BitAnd<BitBoard> for BitBoard {
    type Output = Self;

    fn bitand(self, other: BitBoard) -> Self {
        BitBoard(self.0 & other.0)
    }
}

impl ops::BitAnd<Square> for BitBoard {
    type Output = Self;

    fn bitand(self, other: Square) -> Self {
        BitBoard(self.0 & loc(other))
    }
}

impl ops::BitXorAssign<Square> for BitBoard {
    fn bitxor_assign(&mut self, rhs: Square) {
        self.0 = self.0 ^ (1u64 << rhs.i);
    }
}

