use crate::dir::Dir;
use crate::bitboard::BitBoard;
use itertools::iterate;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square {
    pub i: u8,
    pub rank: u8,
    pub file: u8,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Square {
    pub fn name(self) -> &'static str {
        NAMES[self.i as usize]
    }

    pub fn next(self, dir: Dir) -> Option<Square> {
        let new_rank = (self.rank as i8) + dir.dr;
        let new_file = (self.file as i8) + dir.df;
        if -1 < new_rank && new_rank < 8 && -1 < new_file && new_file < 8 {
            Some(ALL[(8 * new_rank + new_file) as usize])
        } else {
            None
        }
    }

    pub fn search(self, dir: Dir) -> BitBoard {
        iterate(Some(self), |op| op.and_then(|sq| sq.next(dir)))
            .skip(1)
            .take_while(|op| op.is_some())
            .map(|op| op.unwrap())
            .collect()
    }

    pub fn search_all(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter().flat_map(|dir| self.search(*dir)).collect()
    }

    pub fn search_one(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter()
            .flat_map(|dir| self.next(*dir).into_iter())
            .collect()
    }

    const fn new(index: u8) -> Square {
        Square {
            i: index,
            rank: index / 8,
            file: index % 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dir::*;
    use crate::bitboard::*;

    #[test]
    fn test_search() {
        assert_eq!(D3.search(S), BitBoard::new(&[D2, D1]));
    }

    #[test]
    fn test_search_one() {
        assert_eq!(D3.search_one(&vec!(S, E)), BitBoard::new(&[D2, E3]));
        assert_eq!(A8.search_one(&vec!(N, NWW, SE)), BitBoard::new(&[B7]));
    }

    #[test]
    fn test_search_all() {
        assert_eq!(
            C3.search_all(&vec!(SSW, SWW, S)),
            BitBoard::new(&[B1, A2, C2, C1])
        );
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

pub const H1: Square = Square::new(0);
pub const G1: Square = Square::new(1);
pub const F1: Square = Square::new(2);
pub const E1: Square = Square::new(3);
pub const D1: Square = Square::new(4);
pub const C1: Square = Square::new(5);
pub const B1: Square = Square::new(6);
pub const A1: Square = Square::new(7);

pub const H2: Square = Square::new(8);
pub const G2: Square = Square::new(9);
pub const F2: Square = Square::new(10);
pub const E2: Square = Square::new(11);
pub const D2: Square = Square::new(12);
pub const C2: Square = Square::new(13);
pub const B2: Square = Square::new(14);
pub const A2: Square = Square::new(15);

pub const H3: Square = Square::new(16);
pub const G3: Square = Square::new(17);
pub const F3: Square = Square::new(18);
pub const E3: Square = Square::new(19);
pub const D3: Square = Square::new(20);
pub const C3: Square = Square::new(21);
pub const B3: Square = Square::new(22);
pub const A3: Square = Square::new(23);

pub const H4: Square = Square::new(24);
pub const G4: Square = Square::new(25);
pub const F4: Square = Square::new(26);
pub const E4: Square = Square::new(27);
pub const D4: Square = Square::new(28);
pub const C4: Square = Square::new(29);
pub const B4: Square = Square::new(30);
pub const A4: Square = Square::new(31);

pub const H5: Square = Square::new(32);
pub const G5: Square = Square::new(33);
pub const F5: Square = Square::new(34);
pub const E5: Square = Square::new(35);
pub const D5: Square = Square::new(36);
pub const C5: Square = Square::new(37);
pub const B5: Square = Square::new(38);
pub const A5: Square = Square::new(39);

pub const H6: Square = Square::new(40);
pub const G6: Square = Square::new(41);
pub const F6: Square = Square::new(42);
pub const E6: Square = Square::new(43);
pub const D6: Square = Square::new(44);
pub const C6: Square = Square::new(45);
pub const B6: Square = Square::new(46);
pub const A6: Square = Square::new(47);

pub const H7: Square = Square::new(48);
pub const G7: Square = Square::new(49);
pub const F7: Square = Square::new(50);
pub const E7: Square = Square::new(51);
pub const D7: Square = Square::new(52);
pub const C7: Square = Square::new(53);
pub const B7: Square = Square::new(54);
pub const A7: Square = Square::new(55);

pub const H8: Square = Square::new(56);
pub const G8: Square = Square::new(57);
pub const F8: Square = Square::new(58);
pub const E8: Square = Square::new(59);
pub const D8: Square = Square::new(60);
pub const C8: Square = Square::new(61);
pub const B8: Square = Square::new(62);
pub const A8: Square = Square::new(63);

pub const ALL: [Square; 64] = [
    H1, G1, F1, E1, D1, C1, B1, A1, H2, G2, F2, E2, D2, C2, B2, A2, H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4, H5, G5, F5, E5, D5, C5, B5, A5, H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7, H8, G8, F8, E8, D8, C8, B8, A8,
];

const NAMES: [&str; 64] = [
    "H1", "G1", "F1", "E1", "D1", "C1", "B1", "A1", "H2", "G2", "F2", "E2", "D2", "C2", "B2", "A2",
    "H3", "G3", "F3", "E3", "D3", "C3", "B3", "A3", "H4", "G4", "F4", "E4", "D4", "C4", "B4", "A4",
    "H5", "G5", "F5", "E5", "D5", "C5", "B5", "A5", "H6", "G6", "F6", "E6", "D6", "C6", "B6", "A6",
    "H7", "G7", "F7", "E7", "D7", "C7", "B7", "A7", "H8", "G8", "F8", "E8", "D8", "C8", "B8", "A8",
];
