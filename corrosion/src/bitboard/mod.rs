use std::fmt;

use itertools::Itertools;

use crate::square::Square;

pub mod simple;
mod operators;
mod iterator;

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(args: &[Square]) -> BitBoard {
        args.into_iter().map(|x| *x).collect()
    }

    pub fn wrap(bitboard: u64) -> BitBoard {
        BitBoard(bitboard)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn size(self) -> usize {
        // Can make this faster with bit twiddling
       self.into_iter().count()
    }

    pub const EMPTY: BitBoard = BitBoard(0u64);
    pub const ALL: BitBoard = BitBoard(!0u64);

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


#[cfg(test)]
mod test {
    use crate::bitboard::BitBoard;
    use crate::square::constants::*;

    #[test]
    fn test_new() {
        assert_eq!(BitBoard(0b11u64), BitBoard::new(&[H1, G1]))
    }

    #[test]
    fn test_display() {
        let result = BitBoard::new(&[A1, H7, D5]);
        assert_eq!("{A1, D5, H7}".to_owned(), format!("{}", result));
    }
}

