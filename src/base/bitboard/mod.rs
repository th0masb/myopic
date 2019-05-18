use std::fmt;

use itertools::Itertools;

use crate::base::dir;
use crate::base::square::constants::H1;
use crate::base::square::Square;
use crate::base::Reflectable;

pub mod constants;
mod cords;
mod iterator;
mod operators;

/// A bitboard is a value type wrapping a 64 bit integer which represents
/// a set of squares on a chess board. Each bit is mapped to a particular
/// square on the board, 0 -> H1, 1 -> G1,..., 8 -> H2,..., 63 -> A8. For
/// example if we know a piece to reside on a particular square we can
/// use a bitboard to to capture the available moves for that piece.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct BitBoard(pub u64);
impl BitBoard {
    /// Check if this bitboard contains a particular square.
    pub fn contains(self, square: Square) -> bool {
        self.0 & (1u64 << square.i) != 0
    }

    /// Check if this bitboard is empty, i.e contains no squares.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Check if the intersection of this bitboard and the other is
    /// non-empty.
    pub fn intersects(self, other: BitBoard) -> bool {
        !(self & other).is_empty()
    }

    /// Computes the number of squares in this bitboard using the
    /// popcount algorithm.
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

    /// Finds the first square in this set if it is non-empty.
    pub fn first(self) -> Option<Square> {
        self.into_iter().next()
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

fn loc(sq: Square) -> u64 {
    1u64 << sq.i
}

fn create_files() -> Vec<BitBoard> {
    (H1.search(dir::W) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::N) | sq)
        .collect()
}

fn create_ranks() -> Vec<BitBoard> {
    (H1.search(dir::N) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::W) | sq)
        .collect()
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
