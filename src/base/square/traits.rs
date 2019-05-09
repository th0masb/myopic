use std::{fmt, ops};

use crate::base::bitboard::BitBoard;
use crate::base::square::Square;

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ops::Not for Square {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        !self.lift()
    }
}

impl ops::BitOr<Square> for Square {
    type Output = BitBoard;

    fn bitor(self, other: Square) -> Self::Output {
        self.lift() | other.lift()
    }
}

impl ops::BitOr<BitBoard> for Square {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> Self::Output {
        self.lift() | other
    }
}

impl ops::BitAnd<BitBoard> for Square {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> Self::Output {
        self.lift() & other
    }
}

impl ops::Sub<BitBoard> for Square {
    type Output = BitBoard;

    fn sub(self, other: BitBoard) -> Self::Output {
        self.lift() - other
    }
}
