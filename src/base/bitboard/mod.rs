use std::fmt;

use itertools::Itertools;

use crate::base::dir;
use crate::base::square::constants::H1;
use crate::base::square::Square;

pub mod constants;
mod cords;
mod iterator;
mod operators;

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

pub fn create_files() -> Vec<BitBoard> {
    (H1.search(dir::W) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::N) | sq)
        .collect()
}

pub fn create_ranks() -> Vec<BitBoard> {
    (H1.search(dir::N) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::W) | sq)
        .collect()
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct BitBoard(pub u64);
impl BitBoard {
    pub fn contains(self, square: Square) -> bool {
        self.0 & (1u64 << square.i) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn intersects(self, other: BitBoard) -> bool {
        !(self & other).is_empty()
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

    pub fn cord(source: Square, target: Square) -> BitBoard {
        unimplemented!()
    }

    pub const EMPTY: BitBoard = BitBoard(0u64);
    pub const ALL: BitBoard = BitBoard(!0u64);

    pub const RANKS: [BitBoard; 8] = [
        BitBoard(255),
        BitBoard(65280),
        BitBoard(16711680),
        BitBoard(4278190080),
        BitBoard(1095216660480),
        BitBoard(280375465082880),
        BitBoard(71776119061217280),
        BitBoard(18374686479671623680),
    ];
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

    use super::*;

    #[test]
    fn test_display() {
        let result = A1 | H7 | D5;
        assert_eq!("{A1, D5, H7}".to_owned(), format!("{}", result));
    }

    #[test]
    fn test_size() {
        assert_eq!(0, BitBoard::EMPTY.size());
        assert_eq!(64, BitBoard::ALL.size());
        assert_eq!(3, (A3 | G6 | H4).size());
    }

    #[test]
    fn test_create_files() {
        assert_eq!(H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8, create_files()[0]);
    }

    #[test]
    fn test_create_ranks() {
        assert_eq!(A3 | B3 | C3 | D3 | E3 | F3 | G3 | H3, create_ranks()[2]);
    }
}
