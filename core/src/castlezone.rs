use std::iter::FromIterator;
use std::ops;

use crate::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::square::Square;
use crate::Side;
use anyhow::anyhow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents one of the four different areas on a chessboard where
/// the special castling move can take place (two for each side).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum CastleZone {
    WK,
    WQ,
    BK,
    BQ,
}

impl Display for CastleZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CastleZone::WK => "wk",
                CastleZone::WQ => "wq",
                CastleZone::BK => "bk",
                CastleZone::BQ => "bq",
            }
        )
    }
}

impl FromStr for CastleZone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wk" | "WK" => Ok(CastleZone::WK),
            "wq" | "WQ" => Ok(CastleZone::WQ),
            "bk" | "BK" => Ok(CastleZone::BK),
            "bq" | "BQ" => Ok(CastleZone::BQ),
            _ => Err(anyhow!("Cannot parse {} as CastleZone", s)),
        }
    }
}

impl CastleZone {
    /// Return the side which this zone belongs to.
    pub fn side(&self) -> Side {
        match self {
            CastleZone::WK | CastleZone::WQ => Side::White,
            CastleZone::BK | CastleZone::BQ => Side::Black,
        }
    }

    /// Return the kingside zone for the given side.
    pub fn kingside(side: Side) -> CastleZone {
        match side {
            Side::White => CastleZone::WK,
            Side::Black => CastleZone::BK,
        }
    }

    /// Return the queenside zone for the given side.
    pub fn queenside(side: Side) -> CastleZone {
        match side {
            Side::White => CastleZone::WQ,
            Side::Black => CastleZone::BQ,
        }
    }

    /// Create an iterator traversing all zones in order.
    pub fn iter() -> impl Iterator<Item = CastleZone> {
        [
            CastleZone::WK,
            CastleZone::WQ,
            CastleZone::BK,
            CastleZone::BQ,
        ]
        .iter()
        .cloned()
    }

    /// Returns a set of exactly two squares which denote the required
    /// locations of the king and rook in order for the corresponding
    /// castle move to take place.
    pub fn source_squares(self) -> BitBoard {
        match self {
            CastleZone::WK => Square::E1 | Square::H1,
            CastleZone::WQ => Square::E1 | Square::A1,
            CastleZone::BK => Square::E8 | Square::H8,
            CastleZone::BQ => Square::E8 | Square::A8,
        }
    }

    /// Returns a triple containing the rook which moves in the corresponding
    /// castle move along with it's required start square followed by the
    /// square it will finish on.
    pub fn rook_data(self) -> (Piece, Square, Square) {
        match self {
            CastleZone::WK => (Piece::WR, Square::H1, Square::F1),
            CastleZone::BK => (Piece::BR, Square::H8, Square::F8),
            CastleZone::WQ => (Piece::WR, Square::A1, Square::D1),
            CastleZone::BQ => (Piece::BR, Square::A8, Square::D8),
        }
    }

    /// Returns a triple containing the king which moves in the corresponding
    /// castle move along with it's required start square followed by the
    /// square it will finish on.
    pub fn king_data(self) -> (Piece, Square, Square) {
        match self {
            CastleZone::WK => (Piece::WK, Square::E1, Square::G1),
            CastleZone::BK => (Piece::BK, Square::E8, Square::G8),
            CastleZone::WQ => (Piece::WK, Square::E1, Square::C1),
            CastleZone::BQ => (Piece::BK, Square::E8, Square::C8),
        }
    }

    /// Returns a set containing the squares which are required to be
    /// free of any other pieces in order for the corresponding castle
    /// move to be legal.
    pub fn unoccupied_requirement(self) -> BitBoard {
        match self {
            CastleZone::WK => Square::F1 | Square::G1,
            CastleZone::WQ => Square::B1 | Square::C1 | Square::D1,
            CastleZone::BK => Square::F8 | Square::G8,
            CastleZone::BQ => Square::B8 | Square::C8 | Square::D8,
        }
    }

    /// Returns a set containing the squares which are required to be
    /// free of enemy control in order for the corresponding castle move
    /// to be legal.
    pub fn uncontrolled_requirement(self) -> BitBoard {
        match self {
            CastleZone::WK => Square::E1 | Square::F1 | Square::G1,
            CastleZone::WQ => Square::E1 | Square::C1 | Square::D1,
            CastleZone::BK => Square::E8 | Square::F8 | Square::G8,
            CastleZone::BQ => Square::E8 | Square::C8 | Square::D8,
        }
    }

    /// Lifts this zone to a set of one element.
    pub fn lift(self) -> CastleZoneSet {
        CastleZoneSet {
            data: 1usize << self as usize,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CastleZone;

    #[test]
    fn display() {
        assert_eq!("wk", CastleZone::WK.to_string().as_str());
        assert_eq!("wq", CastleZone::WQ.to_string().as_str());
        assert_eq!("bk", CastleZone::BK.to_string().as_str());
        assert_eq!("bq", CastleZone::BQ.to_string().as_str());
    }

    #[test]
    fn from_str() {
        assert_eq!(CastleZone::WK, "wk".parse().unwrap());
        assert_eq!(CastleZone::WK, "WK".parse().unwrap());
        assert_eq!(CastleZone::WQ, "wq".parse().unwrap());
        assert_eq!(CastleZone::WQ, "WQ".parse().unwrap());
        assert_eq!(CastleZone::BK, "bk".parse().unwrap());
        assert_eq!(CastleZone::BK, "BK".parse().unwrap());
        assert_eq!(CastleZone::BQ, "bq".parse().unwrap());
        assert_eq!(CastleZone::BQ, "BQ".parse().unwrap());
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub struct CastleZoneSet {
    data: usize,
}

impl CastleZoneSet {
    pub fn contains(self, zone: CastleZone) -> bool {
        (1usize << zone as usize) & self.data != 0
    }

    pub fn intersects(self, other: CastleZoneSet) -> bool {
        other.iter().any(|z| self.contains(z))
    }

    pub fn iter(self) -> impl Iterator<Item = CastleZone> {
        CastleZone::iter().filter(move |&z| self.contains(z))
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

#[cfg(test)]
mod set_test {
    use super::*;

    #[test]
    fn test_all() {
        let all = CastleZoneSet::ALL;
        for zone in CastleZone::iter() {
            assert!(all.contains(zone));
        }
    }

    #[test]
    fn test_none() {
        let none = CastleZoneSet::NONE;
        for zone in CastleZone::iter() {
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
