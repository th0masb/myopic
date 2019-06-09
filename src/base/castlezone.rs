use std::iter::FromIterator;
use std::ops;

use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::square::Square::A1;
use crate::base::square::Square::A8;
use crate::base::square::Square::C8;
use crate::base::square::Square::D1;
use crate::base::square::Square::D8;
use crate::base::square::Square::E1;
use crate::base::square::Square::E8;
use crate::base::square::Square::F1;
use crate::base::square::Square::F8;
use crate::base::square::Square::G1;
use crate::base::square::Square::G8;
use crate::base::square::Square::H1;
use crate::base::square::Square::H8;
use crate::base::Reflectable;
use crate::pieces::Piece;
use crate::base::square::Square::C1;

/// Represents one of the four different areas on a chessboard where
/// the special castling move can take place (two for each side).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum CastleZone {
    WK,
    WQ,
    BK,
    BQ,
}

impl CastleZone {
    /// Create an iterator traversing all zones in order.
    pub fn iter() -> impl Iterator<Item = CastleZone> {
        CastleZone::ALL.iter().cloned()
    }

    /// Returns a set of exactly two squares which denote the required
    /// locations of the king and rook in order for the corresponding
    /// castle move to take place.
    pub fn source_squares(self) -> BitBoard {
        CastleZone::KING_SOURCES[self as usize] | CastleZone::ROOK_SOURCES[self as usize]
    }

    /// Returns a triple containing the rook which moves in the corresponding
    /// castle move along with it's required start square followed by the
    /// square it will finish on.
    pub fn rook_data(self) -> (Piece, Square, Square) {
        let i = self as usize;
        let rook = match self {
            CastleZone::WK | CastleZone::WQ => Piece::WR,
            CastleZone::BK | CastleZone::BQ => Piece::BR,
        };
        (
            rook,
            CastleZone::ROOK_SOURCES[i],
            CastleZone::ROOK_TARGETS[i],
        )
    }

    /// Returns a triple containing the king which moves in the corresponding
    /// castle move along with it's required start square followed by the
    /// square it will finish on.
    pub fn king_data(self) -> (Piece, Square, Square) {
        let i = self as usize;
        let king = match self {
            CastleZone::WK | CastleZone::WQ => Piece::WK,
            CastleZone::BK | CastleZone::BQ => Piece::BK,
        };
        (
            king,
            CastleZone::KING_SOURCES[i],
            CastleZone::KING_TARGETS[i],
        )
    }

    /// Returns a set containing the squares which are required to be
    /// free of any other pieces in order for the corresponding castle
    /// move to be legal.
    pub fn unoccupied_requirement(self) -> BitBoard {
        CastleZone::REQ_UNOCCUPIED[self as usize]
    }

    /// Returns a set containing the squares which are required to be
    /// free of enemy control in order for the corresponding castle move
    /// to be legal.
    pub fn uncontrolled_requirement(self) -> BitBoard {
        CastleZone::REQ_UNCONTROLLED[self as usize]
    }

    /// Lifts this zone to a set of one element.
    pub fn lift(self) -> CastleZoneSet {
        CastleZoneSet {
            data: 1usize << self as usize,
        }
    }

    /// All the four different castle zones ordered by their id.
    const ALL: [CastleZone; 4] = [
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

/// A castle is reflected by it's side, i.e.
///  - WK <==> BK
///  - WQ <==> BQ
impl Reflectable for CastleZone {
    fn reflect(&self) -> Self {
        CastleZone::ALL[(*self as usize + 2) % 4]
    }
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
        (1usize << zone as usize) & self.data != 0
    }

    pub fn iter(self) -> impl Iterator<Item = CastleZone> {
        CastleZone::iter()
            .filter(move |&z| self.contains(z))
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
                .map(|cz| 1usize << cz as usize)
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

impl Reflectable for CastleZoneSet {
    fn reflect(&self) -> Self {
        self.iter().map(|z| z.reflect()).collect()
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
