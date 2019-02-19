mod operators;
mod iterator;

use std::iter::{FromIterator, IntoIterator};
use std::{fmt, ops};
use itertools::Itertools;

use crate::square;
use crate::square::Square;
use crate::bitboard::iterator::BitBoardIterator;


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
        BitBoardIterator::new(self.0)
    }
}

impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        iter.into_iter().fold(EMPTY, |a, b| a | b)
    }
}

const EMPTY: BitBoard = BitBoard(0u64);
const ALL: BitBoard = BitBoard(!0u64);

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

