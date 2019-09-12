use std::{fmt, ops};

use itertools::Itertools;

use crate::base::direction;
use crate::base::square::Square;
use crate::base::square::Square::H1;
use crate::base::Reflectable;
use std::iter::FromIterator;
use crate::base::bitboard::iterator::BitBoardIterator;

pub mod constants;
mod cords;
mod iterator;

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
        cords::get_cord(source, target)
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
}

impl Reflectable for BitBoard {
    fn reflect(&self) -> Self {
        self.into_iter().map(|sq| sq.reflect()).collect()
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

fn create_files() -> Vec<BitBoard> {
    (H1.search(direction::W) | H1).into_iter().map(|sq| sq.search(direction::N) | sq).collect()
}

fn create_ranks() -> Vec<BitBoard> {
    (H1.search(direction::N) | H1).into_iter().map(|sq| sq.search(direction::W) | sq).collect()
}

#[cfg(test)]
mod test {
    use crate::base::bitboard::BitBoard;
    use crate::base::square::Square::*;

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

    #[test]
    fn test_lsb() {
        assert_eq!(BitBoard::EMPTY, BitBoard::EMPTY.least_set_bit());
        assert_eq!(G1.lift(), (E4 | G1).least_set_bit());
        assert_eq!(E3.lift(), (E3 | G5).least_set_bit());
        assert_eq!(A8.lift(), A8.lift().least_set_bit());
    }
}
