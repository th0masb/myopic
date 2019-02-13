use super::square::Square;
use super::square;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SquareSet(u64);

struct SquareSetIterator { src: u64, counter: usize }

impl SquareSetIterator {
    fn compute_remaining(&self) -> u64 {
        panic!();
    }

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

