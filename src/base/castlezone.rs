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
use crate::pieces::{Piece, KINGS, ROOKS};

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

    pub fn rook_data(self) -> (Piece, Square, Square) {
        let i = self.i;
        (
            ROOKS[i / 2],
            CastleZone::ROOK_SOURCES[i],
            CastleZone::ROOK_TARGETS[i],
        )
    }

    pub fn king_data(self) -> (Piece, Square, Square) {
        let i = self.i;
        (
            KINGS[i / 2],
            CastleZone::KING_SOURCES[i],
            CastleZone::KING_TARGETS[i],
        )
    }

    pub fn unoccupied_requirement(self) -> BitBoard {
        CastleZone::REQ_UNOCCUPIED[self.i]
    }

    pub fn uncontrolled_requirement(self) -> BitBoard {
        CastleZone::REQ_UNCONTROLLED[self.i]
    }

    pub fn lift(self) -> CastleZoneSet {
        CastleZoneSet {
            data: 1usize << self.i,
        }
    }

    pub const WK: CastleZone = CastleZone { i: 0 };
    pub const WQ: CastleZone = CastleZone { i: 1 };
    pub const BK: CastleZone = CastleZone { i: 2 };
    pub const BQ: CastleZone = CastleZone { i: 3 };

    pub const ALL: [CastleZone; 4] = [
        CastleZone::WK,
        CastleZone::WQ,
        CastleZone::BK,
        CastleZone::BQ,
    ];

    const KING_SOURCES: [Square; 4] = [E1, E1, E8, E8];
    const KING_TARGETS: [Square; 4] = [G1, C1, G8, C8];
    const ROOK_SOURCES: [Square; 4] = [H1, A1, H8, A8];
    const ROOK_TARGETS: [Square; 4] = [F1, D1, F8, D8];

    const REQ_UNCONTROLLED: [BitBoard; 4] = [
        BitBoard(sq(1) | sq(2) | sq(3)),
        BitBoard(sq(3) | sq(4) | sq(5) | sq(6)),
        BitBoard(sq(57) | sq(58) | sq(59)),
        BitBoard(sq(59) | sq(60) | sq(61) | sq(62)),
    ];

    const REQ_UNOCCUPIED: [BitBoard; 4] = [
        BitBoard(sq(1) | sq(2)),
        BitBoard(sq(4) | sq(5) | sq(6)),
        BitBoard(sq(57) | sq(58)),
        BitBoard(sq(60) | sq(61) | sq(62)),
    ];
}

const fn sq(i: usize) -> u64 {
    1u64 << i
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleZoneSet {
    data: usize,
}

impl CastleZoneSet {
    pub fn contains(self, zone: CastleZone) -> bool {
        (1usize << zone.i) & self.data != 0
    }

    pub fn hash(self) -> u64 {
        (0..4)
            .filter(|i| (1usize << i) & self.data != 0)
            .map(|i| hash::castle_feature(CastleZone::ALL[i]))
            .fold(0u64, |a, b| a ^ b)
    }

    pub fn iter(self) -> impl Iterator<Item = CastleZone> {
        CastleZone::ALL.iter().map(|&z| z).filter(move |&z| self.contains(z))
    }

    pub const ALL: CastleZoneSet = CastleZoneSet { data: 0b1111 };
    pub const NONE: CastleZoneSet = CastleZoneSet { data: 0 };
    pub const WHITE: CastleZoneSet = CastleZoneSet { data: 0b11 };
    pub const BLACK: CastleZoneSet = CastleZoneSet { data: 0b1100 };
    pub const WK: CastleZoneSet = CastleZoneSet { data: 0b1 };
    pub const WQ: CastleZoneSet = CastleZoneSet { data: 0b10 };
    pub const BK: CastleZoneSet = CastleZoneSet { data: 0b100 };
    pub const BQ: CastleZoneSet = CastleZoneSet { data: 0b1000 };
}

impl FromIterator<CastleZone> for CastleZoneSet {
    fn from_iter<T: IntoIterator<Item = CastleZone>>(iter: T) -> Self {
        CastleZoneSet {
            data: iter
                .into_iter()
                .map(|cz| 1usize << cz.i)
                .fold(0, |a, b| a | b),
        }
    }
}

impl<'a> FromIterator<&'a CastleZone> for CastleZoneSet {
    fn from_iter<T: IntoIterator<Item = &'a CastleZone>>(iter: T) -> Self {
        CastleZoneSet {
            data: iter
                .into_iter()
                .map(|cz| 1usize << cz.i)
                .fold(0, |a, b| a | b),
        }
    }
}

impl ops::Sub<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn sub(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet {
            data: self.data & !rhs.data,
        }
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
        CastleZoneSet {
            data: self.data | rhs.data,
        }
    }
}

impl ops::BitAnd<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn bitand(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet {
            data: self.data & rhs.data,
        }
    }
}

#[cfg(test)]
mod set_test {
    use super::*;

    #[test]
    fn test_all() {
        let all = CastleZoneSet::ALL;
        for &zone in &CastleZone::ALL {
            assert!(all.contains(zone));
        }
    }

    #[test]
    fn test_none() {
        let none = CastleZoneSet::NONE;
        for &zone in &CastleZone::ALL {
            assert!(!none.contains(zone));
        }
    }

    #[test]
    fn test_collect() {
        let source = vec![
            CastleZone::BK,
            CastleZone::WK,
            CastleZone::WQ,
            CastleZone::BQ,
        ];
        let collected: CastleZoneSet = source.into_iter().collect();
        assert_eq!(CastleZoneSet::ALL, collected);
    }
}
