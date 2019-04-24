use std::iter::FromIterator;
use std::ops;

use crate::base::bitboard::BitBoard;
use crate::base::square::constants::A1;
use crate::base::square::constants::A8;
use crate::base::square::constants::B1;
use crate::base::square::constants::B8;
use crate::base::square::constants::C1;
use crate::base::square::constants::C8;
use crate::base::square::constants::D1;
use crate::base::square::constants::D8;
use crate::base::square::constants::E1;
use crate::base::square::constants::E8;
use crate::base::square::constants::F1;
use crate::base::square::constants::F8;
use crate::base::square::constants::G1;
use crate::base::square::constants::G8;
use crate::base::square::constants::H1;
use crate::base::square::constants::H8;
use crate::base::square::Square;
use crate::board::hash;
use crate::pieces::{KINGS, Piece, ROOKS};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct CastleZone {
    i: usize,
}

impl CastleZone {
    pub fn i(self) -> usize {
        self.i
    }

    pub fn source_squares(self) -> BitBoard {
        CastleZone::KING_SOURCES[self.i] | CastleZone::ROOK_SOURCES[self.i]
    }

    pub fn rook_data(self) -> (&'static dyn Piece, Square, Square) {
        let i = self.i;
        (ROOKS[i / 2], CastleZone::ROOK_SOURCES[i], CastleZone::ROOK_TARGETS[i])
    }

    pub fn king_data(self) -> (&'static dyn Piece, Square, Square) {
        let i = self.i;
        (KINGS[i / 2], CastleZone::KING_SOURCES[i], CastleZone::KING_TARGETS[i])
    }

//    pub fn king_source(self) -> Square {
//        CastleZone::KING_SOURCES[self.i]
//    }
//
//    pub fn king_target(self) -> Square {
//        CastleZone::KING_TARGETS[self.i]
//    }
//
//    pub fn rook_source(self) -> Square {
//        CastleZone::ROOK_SOURCES[self.i]
//    }
//
//    pub fn rook_target(self) -> Square {
//        CastleZone::ROOK_TARGETS[self.i]
//    }
//
//    pub fn king(self) -> &'static dyn Piece {
//        KINGS[self. i / 2]
//    }
//
//    pub fn rook(self) -> &'static dyn Piece {
//        ROOKS[self. i / 2]
//    }

    pub fn lift(&self) -> CastleZoneSet {
        CastleZoneSet {data: 1usize << self.i}
    }

    pub const WK: CastleZone = CastleZone {i: 0};
    pub const WQ: CastleZone = CastleZone {i: 1};
    pub const BK: CastleZone = CastleZone {i: 2};
    pub const BQ: CastleZone = CastleZone {i: 3};

    pub const ALL: [CastleZone; 4] = [
        CastleZone::WK,
        CastleZone::WQ,
        CastleZone::BK,
        CastleZone::BQ
    ];

    const KING_SOURCES: [Square; 4] = [E1, E1, E8, E8];
    const KING_TARGETS: [Square; 4] = [G1, C1, G8, C8];
    const ROOK_SOURCES: [Square; 4] = [H1, A1, H8, A8];
    const ROOK_TARGETS: [Square; 4] = [F1, D1, F8, D8];
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleZoneSet {
    data: usize,
}

impl CastleZoneSet {
    pub fn all() -> CastleZoneSet {
        CastleZoneSet {data: 0b1111}
    }

    pub fn none() -> CastleZoneSet {
        CastleZoneSet {data: 0}
    }

    pub fn white() -> CastleZoneSet {
        CastleZoneSet {data: 0b11}
    }

    pub fn black() -> CastleZoneSet {
        CastleZoneSet {data: 0b1100}
    }

    pub fn contains(self, zone: CastleZone) -> bool {
        (1usize << zone.i) & self.data != 0
    }

    pub fn hash(self) -> u64 {
        (0..4).filter(|i| (1usize << i) & self.data != 0)
            .map(|i| hash::castle_feature(CastleZone::ALL[i]))
            .fold(0u64, |a, b| a ^ b)
    }
}

impl FromIterator<CastleZone> for CastleZoneSet {
    fn from_iter<T: IntoIterator<Item=CastleZone>>(iter: T) -> Self {
        CastleZoneSet{data: iter.into_iter().map(|cz| 1usize << cz.i).fold(0, |a, b| a | b)}
    }
}

impl<'a> FromIterator<&'a CastleZone> for CastleZoneSet {
    fn from_iter<T: IntoIterator<Item=&'a CastleZone>>(iter: T) -> Self {
        CastleZoneSet{data: iter.into_iter().map(|cz| 1usize << cz.i).fold(0, |a, b| a | b)}
    }
}

impl ops::Sub<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn sub(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet {data: self.data & !rhs.data}
    }
}

impl ops::SubAssign<CastleZoneSet> for CastleZoneSet {
    fn sub_assign(&mut self, rhs: CastleZoneSet) {
        self.data &= !rhs.data
    }
}

impl ops::BitOr<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn bitor(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet{data: self.data | rhs.data}
    }
}

impl ops::BitAnd<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn bitand(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet{data: self.data & rhs.data}
    }
}


#[cfg(test)]
mod set_test {
    use super::*;

    #[test]
    fn test_all() {
        let all = CastleZoneSet::all();
        for &zone in &CastleZone::ALL {
            assert!(all.contains(zone));
        }
    }

    #[test]
    fn test_none() {
        let none = CastleZoneSet::none();
        for &zone in &CastleZone::ALL {
            assert!(!none.contains(zone));
        }
    }

    #[test]
    fn test_collect() {
        let source = vec![CastleZone::BK, CastleZone::WK, CastleZone::WQ, CastleZone::BQ];
        let collected: CastleZoneSet = source.into_iter().collect();
        assert_eq!(CastleZoneSet::all(), collected);
    }
}
