use crate::square::{Square, ALL};

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
                return Some(ALL[prev]);
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

const HALVES: [u64; 2] = [
    QUARTS[0] | QUARTS[1],
    QUARTS[2] | QUARTS[3],
];
