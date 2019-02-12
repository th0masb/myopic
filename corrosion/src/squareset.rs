use super::square::Square;
use super::square;

pub struct SquareSet(u64);

struct SquareSetIterator { src: u64, counter: usize }

impl SquareSetIterator {
    fn compute_remaining(&self) -> u64 {
        panic!();
    }

}

impl Iterator for SquareSetIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {

        Some(square::ALL[0])
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
