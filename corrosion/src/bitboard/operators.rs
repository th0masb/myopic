use crate::bitboard::{BitBoard, loc};
use crate::square::Square;

use std::ops;


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

impl ops::BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, other: BitBoard) -> Self {
        BitBoard(self.0 & other.0)
    }
}
