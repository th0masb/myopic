use std::num::Wrapping;

use crate::base::square::Square;

/// The iterator implementation struct produced by a bitboard. It simply
/// wraps a long value used to track the remaining set bits.
pub struct BitBoardIterator(pub u64);

/// The implementation uses the 'de bruijn' forward bitscan method for
/// determining the LSB of the encapsulated u64 value. The LSB represents
/// the next square to be returned.
impl Iterator for BitBoardIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let lsb = bitscan(self.0);
            self.0 ^= 1u64 << lsb as u64;
            Some(Square::from_index(lsb))
        }
    }
}

fn bitscan(x: u64) -> usize {
    let wx = Wrapping::<u64>(x ^ (x - 1));
    BITSCAN[((wx * DEBRUIJN64).0 >> 58) as usize]
}

const BITSCAN: [usize; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

const DEBRUIJN64: Wrapping<u64> = Wrapping(0x03f79d71b4cb0a89u64);

#[cfg(test)]
mod bitscan_test {
    use super::bitscan;

    #[test]
    fn test() {
        assert_eq!(0, bitscan(1u64));
        assert_eq!(1, bitscan(2u64));
        assert_eq!(1, bitscan(0b10u64));
        assert_eq!(10, bitscan(0b1001110000000000u64));
        assert_eq!(21, bitscan(0b1001111011000000000000000000000));
    }
}
