use std::iter::FromIterator;
use std::{fmt, ops};
use std::num::Wrapping;

use itertools::Itertools;
use lazy_static::lazy_static;
use enum_map::enum_map;

use crate::Square::H1;
use crate::{Dir, Square};
use crate::Matrix;

/// A bitboard is a value type wrapping a 64 bit integer which represents
/// a set of squares on a chess board. Each bit is mapped to a particular
/// square on the board, 0 -> H1, 1 -> G1,..., 8 -> H2,..., 63 -> A8. For
/// example if we know a piece to reside on a particular square we can
/// use a bitboard to to capture the available moves for that piece.
#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Check if this bitboard contains a particular square.
    pub fn contains(self, square: Square) -> bool {
        self.0 & (1u64 << (square as u64)) != 0
    }

    /// Check if this set is a superset of the other.
    pub fn subsumes(self, other: BitBoard) -> bool {
        (other - self).is_empty()
    }

    /// Check if this bitboard is empty, i.e contains no squares.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Check if this bitboard contains at least one square.
    pub fn is_populated(self) -> bool {
        self.0 != 0
    }

    /// Check if the intersection of this bitboard and the other is
    /// non-empty.
    pub fn intersects(self, other: BitBoard) -> bool {
        !(self & other).is_empty()
    }

    /// Computes the number of squares in this bitboard using the
    /// popcount algorithm.
    pub fn size(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn iter(self) -> impl Iterator<Item = Square> {
        self.into_iter()
    }

    /// Finds the first square in this set if it is non-empty.
    pub fn first(self) -> Option<Square> {
        self.into_iter().next()
    }

    /// Returns a bitboard with the least set bit of this bitboard
    /// or nothing if this bitboard is empty.
    pub fn least_set_bit(self) -> BitBoard {
        let trailing = self.0.trailing_zeros();
        if trailing == 64 {
            BitBoard::EMPTY
        } else {
            BitBoard(1u64 << trailing)
        }
    }

    /// Computes the 'cord' between two squares. Imagine a queen sat
    /// on the source square on and empty board. If the queen can move
    /// to the target square then this method returns the set of
    /// squares which the queen slides along to get to this target
    /// square (inclusive of both ends) otherwise the empty bitboard
    /// is returned.
    pub fn cord(source: Square, target: Square) -> BitBoard {
        lazy_static! {
            static ref CACHE: Matrix<Square, BitBoard> =
                enum_map! { a => enum_map! { b => compute_cord(a, b) } };
        }
        CACHE[source][target]
    }

    /// The empty bitboard (set of no squares).
    pub const EMPTY: BitBoard = BitBoard(0u64);
    /// The complete bitboard (set of all squares).
    pub const ALL: BitBoard = BitBoard(!0u64);

    /// Array of bitboards represented the eight ranks, ordered 1 to 8.
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

    /// Array of bitboards represented the eight files, ordered H to A.
    pub const FILES: [BitBoard; 8] = [
        BitBoard(72340172838076673),
        BitBoard(144680345676153346),
        BitBoard(289360691352306692),
        BitBoard(578721382704613384),
        BitBoard(1157442765409226768),
        BitBoard(2314885530818453536),
        BitBoard(4629771061636907072),
        BitBoard(9259542123273814144),
    ];

    pub const RIM: BitBoard =
        BitBoard(72340172838076673 | 9259542123273814144 | 255 | 18374686479671623680);
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::EMPTY
    }
}

/// A bitboard is a set of squares and is therefore iterable.
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIterator;
    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator(self.0)
    }
}

/// A set of squares can be built from an iterator traversing squares.
impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

/// We can collect an iterator of bitboards into a single bitboard under
/// the logical OR binary operator on sets.
impl FromIterator<BitBoard> for BitBoard {
    fn from_iter<I: IntoIterator<Item = BitBoard>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        value.lift()
    }
}

/// Operator implementations for bitboards which all use the underlying u64
/// value.
impl ops::Shr<u8> for BitBoard {
    type Output = Self;
    fn shr(self, shift: u8) -> Self {
        BitBoard(self.0 >> shift as u64)
    }
}

impl ops::Shl<u8> for BitBoard {
    type Output = Self;
    fn shl(self, shift: u8) -> Self {
        BitBoard(self.0 << shift as u64)
    }
}

impl ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl ops::Sub<BitBoard> for BitBoard {
    type Output = Self;
    fn sub(self, other: BitBoard) -> Self {
        BitBoard(self.0 & !other.0)
    }
}

impl ops::Sub<Square> for BitBoard {
    type Output = Self;
    fn sub(self, other: Square) -> Self {
        BitBoard(self.0 & !loc(other))
    }
}

impl ops::BitXor<BitBoard> for BitBoard {
    type Output = Self;
    fn bitxor(self, other: BitBoard) -> Self {
        BitBoard(self.0 ^ other.0)
    }
}

impl ops::BitXor<Square> for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Square) -> Self {
        BitBoard(self.0 ^ loc(rhs))
    }
}

impl ops::BitOr<BitBoard> for BitBoard {
    type Output = Self;
    fn bitor(self, other: BitBoard) -> Self {
        BitBoard(self.0 | other.0)
    }
}

impl ops::BitOr<Square> for BitBoard {
    type Output = Self;
    fn bitor(self, other: Square) -> Self {
        BitBoard(self.0 | loc(other))
    }
}

impl ops::BitAnd<BitBoard> for BitBoard {
    type Output = Self;
    fn bitand(self, other: BitBoard) -> Self {
        BitBoard(self.0 & other.0)
    }
}

impl ops::BitAnd<Square> for BitBoard {
    type Output = Self;
    fn bitand(self, other: Square) -> Self {
        BitBoard(self.0 & loc(other))
    }
}

impl ops::BitXorAssign<BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 ^ rhs.0;
    }
}

impl ops::BitXorAssign<Square> for BitBoard {
    fn bitxor_assign(&mut self, rhs: Square) {
        self.0 = self.0 ^ (1u64 << (rhs as u64));
    }
}

impl ops::BitOrAssign<BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 | rhs.0;
    }
}

impl ops::BitOrAssign<Square> for BitBoard {
    fn bitor_assign(&mut self, rhs: Square) {
        self.0 = self.0 | (1u64 << (rhs as u64));
    }
}

impl ops::BitAndAssign<BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 & rhs.0;
    }
}

impl ops::BitAndAssign<Square> for BitBoard {
    fn bitand_assign(&mut self, rhs: Square) {
        self.0 = self.0 & (1u64 << (rhs as u64));
    }
}

impl ops::SubAssign<BitBoard> for BitBoard {
    fn sub_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 & !rhs.0;
    }
}

impl ops::SubAssign<Square> for BitBoard {
    fn sub_assign(&mut self, rhs: Square) {
        self.0 = self.0 & !(1u64 << (rhs as u64));
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

fn loc(sq: Square) -> u64 {
    1u64 << (sq as u64)
}

#[allow(dead_code)]
fn create_files() -> Vec<BitBoard> {
    (H1.search(Dir::W) | H1).into_iter().map(|sq| sq.search(Dir::N) | sq).collect()
}

#[allow(dead_code)]
fn create_ranks() -> Vec<BitBoard> {
    (H1.search(Dir::N) | H1).into_iter().map(|sq| sq.search(Dir::W) | sq).collect()
}

#[cfg(test)]
mod test {
    use crate::bitboard::BitBoard;
    use crate::Square::*;

    use super::*;

    #[test]
    fn test_from_square_iter() {
        assert_eq!(F1 | G6, vec!(F1, G6).into_iter().collect());
    }

    #[test]
    fn test_into_iter() {
        assert_eq!(vec![F1, G6], (F1 | G6).into_iter().collect::<Vec<Square>>());
    }

    #[test]
    fn test_display() {
        let result = A1 | H7 | D5;
        assert_eq!("{a1, d5, h7}".to_owned(), format!("{}", result));
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

    #[test]
    fn test_lsb() {
        assert_eq!(BitBoard::EMPTY, BitBoard::EMPTY.least_set_bit());
        assert_eq!(G1.lift(), (E4 | G1).least_set_bit());
        assert_eq!(E3.lift(), (E3 | G5).least_set_bit());
        assert_eq!(A8.lift(), A8.lift().least_set_bit());
    }
}

fn compute_cord(source: Square, target: Square) -> BitBoard {
    [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW]
        .iter()
        .find(|&d| source.search(*d).contains(target))
        .map_or(BitBoard::EMPTY, |&d| takewhile_inc(source, target, d) | source)
}

fn takewhile_inc(source: Square, target: Square, dir: Dir) -> BitBoard {
    source.search_vec(dir).into_iter().take_while(|&sq| sq != target).collect::<BitBoard>() | target
}

#[cfg(test)]
mod cord_test {
    use crate::Square::*;

    use super::*;

    #[test]
    fn test_compute_cord() {
        assert_eq!(H1 | H2 | H3, compute_cord(H1, H3));
    }

    #[test]
    fn test_get_cord() {
        assert_eq!(H1 | H2 | H3, BitBoard::cord(H1, H3));
        assert_eq!(H1 | H2 | H3, BitBoard::cord(H3, H1));
        assert_eq!(F3 | E3 | D3 | C3, BitBoard::cord(C3, F3));
        assert_eq!(D5 | E6 | F7, BitBoard::cord(D5, F7));
        assert_eq!(A8 | B7, BitBoard::cord(A8, B7));
        assert_eq!(B8 | A8, BitBoard::cord(B8, A8));
    }
}

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
            Some(lsb.into())
        }
    }
}

fn bitscan(x: u64) -> usize {
    BITSCAN[(x ^ x - 1).wrapping_mul(DEBRUIJN64).wrapping_shr(58)as usize]
}

const BITSCAN: [usize; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89u64;

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
