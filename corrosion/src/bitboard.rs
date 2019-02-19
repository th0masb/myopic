use crate::square;
use crate::square::Square;
use itertools::Itertools;

use std::fmt;
use std::fmt::Display;
use std::iter::{FromIterator, IntoIterator};
use std::ops;

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitBoard(u64);

impl BitBoard {

    pub fn new(args: &[Square]) -> BitBoard {
        args.into_iter().map(|x| *x).collect()
    }

    pub fn wrap(bitboard: u64) -> BitBoard {
        BitBoard(bitboard)
    }
}

impl ops::Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl ops::Sub for BitBoard {
    type Output = Self;

    fn sub(self, other: BitBoard) -> Self {
        BitBoard(self.0 & !other.0)
    }
}

impl ops::BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, other: BitBoard) -> Self {
        BitBoard(self.0 ^ other.0)
    }
}

impl ops::BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, other: BitBoard) -> Self {
        BitBoard(self.0 | other.0)
    }
}

impl ops::BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, other: BitBoard) -> Self {
        BitBoard(self.0 & other.0)
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.into_iter().join(", "))
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.into_iter().join(", "))
    }
}


impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator {
            src: self.0,
            counter: 0,
        }
    }
}

impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        let mut locations = 0u64;
        for square in iter {
            locations |= loc(square);
        }
        BitBoard(locations)
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::{loc, BitBoard};
    use crate::square::*;

    fn new_set(a: Square, b: Square, c: Square) -> BitBoard {
        BitBoard(loc(a) | loc(b) | loc(c))
    }

    #[test]
    fn test_from_iter() {
        assert_eq!(new_set(F1, G6, C7), vec!(F1, G6, C7).into_iter().collect());
    }

    #[test]
    fn test_into_iter() {
        assert_eq!(
            vec!(F1, G6, C7),
            new_set(F1, G6, C7).into_iter().collect::<Vec<Square>>()
        );
    }

    #[test]
    fn test_display() {
        let result = BitBoard::new(&[A1, H7, D5]);
        assert_eq!("{A1, D5, H7}".to_owned(), format!("{}", result));
    }
}

pub struct BitBoardIterator {
    src: u64,
    counter: usize,
}

impl BitBoardIterator {}

// TODO can make this more efficient.
impl Iterator for BitBoardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        while self.counter < 64 {
            let prev = self.counter;
            self.counter += 1;
            if self.src & (1u64 << prev) != 0 {
                return Some(square::ALL[prev]);
            }
        }
        None
    }
}

const RANKS: [u64; 8] = [
    (0b11111111) << 0,
    (0b11111111) << 1,
    (0b11111111) << 2,
    (0b11111111) << 3,
    (0b11111111) << 4,
    (0b11111111) << 5,
    (0b11111111) << 6,
    (0b11111111) << 7,
];

const HALVES: [u64; 2] = [
    RANKS[0] | RANKS[1] | RANKS[2] | RANKS[3],
    RANKS[4] | RANKS[5] | RANKS[6] | RANKS[7],
];

const QUARTS: [u64; 4] = [
    RANKS[0] | RANKS[1],
    RANKS[2] | RANKS[3],
    RANKS[4] | RANKS[5],
    RANKS[6] | RANKS[7],
];
