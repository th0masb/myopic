use super::square;
use super::square::Square;
use std::iter::{FromIterator, IntoIterator};

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SquareSet(u64);

impl IntoIterator for SquareSet {
    type Item = Square;
    type IntoIter = SquareSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareSetIterator {
            src: self.0,
            counter: 0,
        }
    }
}

impl FromIterator<Square> for SquareSet {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        let mut locations = 0u64;
        for square in iter {
            locations |= loc(square);
        }
        SquareSet(locations)
    }
}

#[cfg(test)]
mod iterationtests {
    use crate::square::*;
    use crate::squareset::loc;
    use crate::squareset::SquareSet;

    fn new_set(a: Square, b: Square, c: Square) -> SquareSet {
        SquareSet(loc(a) | loc(b) | loc(c))
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
}

pub struct SquareSetIterator {
    src: u64,
    counter: usize,
}

// TODO can make this more efficient.
impl Iterator for SquareSetIterator {
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
