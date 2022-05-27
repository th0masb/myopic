use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use enum_map::Enum;

use crate::{BitBoard, Dir};

/// Type representing a square on a chessboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Enum)]
#[rustfmt::skip]
pub enum Square {
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        Square::iter()
            .find(|sq| sq.to_string() == lower)
            .ok_or(anyhow!("Cannot parse {} as a Square", s))
    }
}

impl Square {
    /// Return an iterator traversing all squares in order.
    pub fn iter() -> impl Iterator<Item = Square> {
        ALL.iter().cloned()
    }

    /// Retrieve a square by it's corresponding index.
    pub fn from_index(i: usize) -> Square {
        ALL[i]
    }

    /// Return the index of the rank on which this square resides.
    pub const fn rank_index(self) -> usize {
        (self as usize) / 8
    }

    /// Return the index of the file on which this square resides.
    pub const fn file_index(self) -> usize {
        (self as usize) % 8
    }

    /// Return a bitboard representing the rank on which this square
    /// resides.
    pub fn rank(self) -> BitBoard {
        BitBoard::RANKS[self.rank_index()]
    }

    /// Return a bitboard representing the file on which this square
    /// resides.
    pub fn file(self) -> BitBoard {
        BitBoard::FILES[self.file_index()]
    }

    /// 'Lifts' this square to a singleton set of squares.
    pub const fn lift(self) -> BitBoard {
        BitBoard(1u64 << (self as u64))
    }

    /// Finds the next square on a chessboard from this square in a
    /// given direction if it exists.
    pub fn next(self, dir: Dir) -> Option<Square> {
        let dr = match dir {
            Dir::E | Dir::W => 0,
            Dir::N | Dir::NE | Dir::NEE | Dir::NW | Dir::NWW => 1,
            Dir::NNE | Dir::NNW => 2,
            Dir::S | Dir::SE | Dir::SEE | Dir::SW | Dir::SWW => -1,
            Dir::SSE | Dir::SSW => -2,
        };
        let df = match dir {
            Dir::N | Dir::S => 0,
            Dir::W | Dir::NW | Dir::NNW | Dir::SW | Dir::SSW => 1,
            Dir::NWW | Dir::SWW => 2,
            Dir::E | Dir::NE | Dir::NNE | Dir::SE | Dir::SSE => -1,
            Dir::NEE | Dir::SEE => -2,
        };
        let new_rank = (self.rank_index() as i8) + dr;
        let new_file = (self.file_index() as i8) + df;
        if -1 < new_rank && new_rank < 8 && -1 < new_file && new_file < 8 {
            Some(ALL[(8 * new_rank + new_file) as usize])
        } else {
            None
        }
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a set.
    pub fn search(self, dir: Dir) -> BitBoard {
        self.search_vec(dir).into_iter().collect()
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a vector where the squares are ordered in
    /// increasing distance from this square.
    pub fn search_vec(self, dir: Dir) -> Vec<Square> {
        itertools::iterate(Some(self), |op| op.and_then(|sq| sq.next(dir)))
            .skip(1)
            .take_while(|op| op.is_some())
            .map(|op| op.unwrap())
            .collect()
    }

    /// Find all squares in all directions in a given vector and
    /// returns them as a set.
    pub fn search_all(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter().flat_map(|&dir| self.search(dir)).collect()
    }

    /// Find the squares adjacent to this square in all of the
    /// given directions and returns them as a set.
    pub fn search_one(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter()
            .flat_map(|&dir| self.next(dir).into_iter())
            .collect()
    }
}

impl std::ops::Shl<usize> for Square {
    type Output = Square;
    fn shl(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize + rhs)
    }
}

impl std::ops::Shr<usize> for Square {
    type Output = Square;
    fn shr(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize - rhs)
    }
}

impl std::ops::Not for Square {
    type Output = BitBoard;
    fn not(self) -> Self::Output {
        !self.lift()
    }
}

impl std::ops::BitOr<Square> for Square {
    type Output = BitBoard;
    fn bitor(self, other: Square) -> Self::Output {
        self.lift() | other.lift()
    }
}

impl std::ops::BitOr<BitBoard> for Square {
    type Output = BitBoard;
    fn bitor(self, other: BitBoard) -> Self::Output {
        self.lift() | other
    }
}

impl std::ops::BitAnd<BitBoard> for Square {
    type Output = BitBoard;
    fn bitand(self, other: BitBoard) -> Self::Output {
        self.lift() & other
    }
}

impl std::ops::Sub<BitBoard> for Square {
    type Output = BitBoard;
    fn sub(self, other: BitBoard) -> Self::Output {
        self.lift() - other
    }
}

#[rustfmt::skip]
const ALL: [Square; 64] = [
    Square::H1, Square::G1, Square::F1, Square::E1, Square::D1, Square::C1, Square::B1, Square::A1,
    Square::H2, Square::G2, Square::F2, Square::E2, Square::D2, Square::C2, Square::B2, Square::A2,
    Square::H3, Square::G3, Square::F3, Square::E3, Square::D3, Square::C3, Square::B3, Square::A3,
    Square::H4, Square::G4, Square::F4, Square::E4, Square::D4, Square::C4, Square::B4, Square::A4,
    Square::H5, Square::G5, Square::F5, Square::E5, Square::D5, Square::C5, Square::B5, Square::A5,
    Square::H6, Square::G6, Square::F6, Square::E6, Square::D6, Square::C6, Square::B6, Square::A6,
    Square::H7, Square::G7, Square::F7, Square::E7, Square::D7, Square::C7, Square::B7, Square::A7,
    Square::H8, Square::G8, Square::F8, Square::E8, Square::D8, Square::C8, Square::B8, Square::A8,
];

#[cfg(test)]
mod test {
    use crate::square::Square;
    use crate::square::Square::*;
    use crate::Dir::*;

    #[test]
    fn test_rank() {
        assert_eq!(A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1, F1.rank());
        assert_eq!(A4 | B4 | C4 | D4 | E4 | F4 | G4 | H4, D4.rank());
        assert_eq!(A8 | B8 | C8 | D8 | E8 | F8 | G8 | H8, A8.rank());
    }

    #[test]
    fn test_file() {
        assert_eq!(B1 | B2 | B3 | B4 | B5 | B6 | B7 | B8, B4.file())
    }

    #[test]
    fn test_partial_ord() {
        for i in 0..64 {
            let prev: Vec<_> = Square::iter().take(i).collect();
            let next: Vec<_> = Square::iter().skip(i + 1).collect();
            let pivot = Square::from_index(i);

            for smaller in prev {
                assert_eq!(true, smaller < pivot);
            }

            for larger in next {
                assert_eq!(true, pivot < larger);
            }
        }
    }

    #[test]
    fn test_search() {
        assert_eq!(D3.search(S), D2 | D1);
    }

    #[test]
    fn test_search_vec() {
        assert_eq!(D3.search_vec(S), vec![D2, D1])
    }

    #[test]
    fn test_search_one() {
        assert_eq!(D3.search_one(&vec!(S, E)), D2 | E3);
        assert_eq!(A8.search_one(&vec!(N, NWW, SE)), B7.lift());
    }

    #[test]
    fn test_search_all() {
        assert_eq!(C3.search_all(&vec!(SSW, SWW, S)), B1 | A2 | C2 | C1);
    }

    #[test]
    fn test_next() {
        assert_eq!(C3.next(N), Some(C4));
        assert_eq!(C3.next(E), Some(D3));
        assert_eq!(C3.next(S), Some(C2));
        assert_eq!(C3.next(W), Some(B3));
        assert_eq!(C3.next(NE), Some(D4));
        assert_eq!(C3.next(SE), Some(D2));
        assert_eq!(C3.next(SW), Some(B2));
        assert_eq!(C3.next(NW), Some(B4));
        assert_eq!(C3.next(NNE), Some(D5));
        assert_eq!(C3.next(NEE), Some(E4));
        assert_eq!(C3.next(SEE), Some(E2));
        assert_eq!(C3.next(SSE), Some(D1));
        assert_eq!(C3.next(SSW), Some(B1));
        assert_eq!(C3.next(SWW), Some(A2));
        assert_eq!(C3.next(NWW), Some(A4));
        assert_eq!(C3.next(NNW), Some(B5));

        assert_eq!(G8.next(N), None);
        assert_eq!(H6.next(E), None);
        assert_eq!(B1.next(S), None);
        assert_eq!(A4.next(W), None);
    }
}
