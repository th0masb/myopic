use std::iter::{FromIterator, IntoIterator};
use std::num::Wrapping;

use crate::base::bitboard::BitBoard;
use crate::base::square::Square;

/// A bitboard is a set of squares and is therefore iterable.
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator { src: self.0 }
    }
}

/// A set of squares can be built from an iterator traversing squares.
impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

/// We can collect an iterator of bitboards into a single bitboard under
/// the logical OR binary operator on sets.
impl FromIterator<BitBoard> for BitBoard {
    fn from_iter<I: IntoIterator<Item = BitBoard>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

#[cfg(test)]
mod iter_test {
    use crate::base::bitboard::{loc, BitBoard};
    use crate::base::square::Square;
    use crate::base::square::Square::*;

    fn new_set(a: Square, b: Square) -> BitBoard {
        BitBoard(loc(a) | loc(b))
    }

    #[test]
    fn test_from_square_iter() {
        assert_eq!(new_set(F1, G6), vec!(F1, G6).into_iter().collect());
    }

    #[test]
    fn test_into_iter() {
        assert_eq!(
            vec!(F1, G6),
            new_set(F1, G6).into_iter().collect::<Vec<Square>>()
        );
    }
}

/// The iterator implementation struct produced by a bitboard. It simply
/// wraps a long value used to track the remaining set bits.
pub struct BitBoardIterator {
    src: u64,
}

/// The implementation uses the 'de bruijn' forward bitscan method for
/// determining the LSB of the encapsulated u64 value. The LSB represents
/// the next square to be returned.
impl Iterator for BitBoardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.src == 0 {
            None
        } else {
            let lsb = bitscan(self.src);
            self.src ^= 1u64 << lsb;
            Some(Square::from_index(lsb))
        }
    }
}

fn bitscan(x: u64) -> usize {
    let wx = Wrapping::<u64>(x ^ (x - 1));
    BITSCAN[((wx * DEBRUIJN64).0 >> 58) as usize]
}

const BITSCAN: [usize; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

const DEBRUIJN64: Wrapping<u64> = Wrapping(0x03f79d71b4cb0a89u64);

#[cfg(test)]
mod bitscan_test {
    use super::bitscan;

    #[test]
    fn test() {
        assert_eq!(0, bitscan(1u64));
        assert_eq!(1, bitscan(2u64));
        assert_eq!(1, bitscan(0b10u64));
        assert_eq!(10, bitscan(0b1001110000000000u64));
        assert_eq!(21, bitscan(0b1001111011000000000000000000000));
    }
}
