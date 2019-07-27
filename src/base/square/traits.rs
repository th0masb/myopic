use std::{fmt, ops};

use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;

impl Reflectable for Square {
    fn reflect(&self) -> Self {
        let (fi, ri) = (self.file_index(), self.rank_index());
        Square::from_index((8 * (7 - ri) + fi) as usize)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl ops::Shl<usize> for Square {
    type Output = Square;

    fn shl(self, rhs: usize) -> Self::Output {
        unimplemented!()
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
