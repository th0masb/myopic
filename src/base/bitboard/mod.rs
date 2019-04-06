use std::fmt;

use itertools::Itertools;

use crate::base::square::Square;

mod iterator;
mod operators;
pub mod simple;

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitBoard(pub u64);

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
        // Uses popcount algorithm.
        let mut x = self.0;
        let mut count = 0;
        while x > 0 {
            x &= x - 1;
            count += 1;
        }
        count
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
    use crate::base::bitboard::BitBoard;
    use crate::base::square::constants::*;

    #[test]
    fn test_new() {
        assert_eq!(BitBoard(0b11u64), BitBoard::new(&[H1, G1]))
    }

    #[test]
    fn test_display() {
        let result = BitBoard::new(&[A1, H7, D5]);
        assert_eq!("{A1, D5, H7}".to_owned(), format!("{}", result));
    }

    #[test]
    fn test_size() {
        assert_eq!(0, BitBoard::EMPTY.size());
        assert_eq!(64, BitBoard::ALL.size());
        assert_eq!(3, (A3 | G6 | H4).size());
    }
}
