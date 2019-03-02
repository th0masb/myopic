use std::iter::{FromIterator, IntoIterator};

use crate::bitboard;
use crate::bitboard::BitBoard;
use crate::square::constants::SQUARES;
use crate::square::Square;

// Iterator related trait implementations for the BitBoard struct.
//
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator::new(self.0)
    }
}

impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

impl FromIterator<BitBoard> for BitBoard {
    fn from_iter<I: IntoIterator<Item = BitBoard>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

#[cfg(test)]
mod trait_test {
    use crate::bitboard::{BitBoard, loc};
    use crate::square::constants::*;
    use crate::square::Square;

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


// Implementation of the actual Iterator that a BitBoard produces.
//
pub struct BitBoardIterator {
    src: u64,
    counter: usize,
}

impl BitBoardIterator {
    pub fn new(src: u64) -> BitBoardIterator {
        BitBoardIterator { src, counter: 0 }
    }
}

// TODO can make this more efficient.
impl Iterator for BitBoardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        while self.counter < 64 {
            let prev = self.counter;
            self.counter += 1;
            if self.src & (1u64 << prev) != 0 {
                return Some(SQUARES[prev]);
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

const QUARTS: [u64; 4] = [
    RANKS[0] | RANKS[1],
    RANKS[2] | RANKS[3],
    RANKS[4] | RANKS[5],
    RANKS[6] | RANKS[7],
];

const HALVES: [u64; 2] = [QUARTS[0] | QUARTS[1], QUARTS[2] | QUARTS[3]];

#[cfg(test)]
mod iterator_tests {}
