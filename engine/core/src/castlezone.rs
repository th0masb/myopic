use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::anyhow;
use enum_map::Enum;
use enumset::{enum_set, EnumSet, EnumSetType};

use crate::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::square::Square;
use crate::Side;

/// Represents one of the four different areas on a chessboard where
/// the special castling move can take place (two for each side).
#[derive(Debug, EnumSetType, Enum, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum CastleZone { WK, WQ, BK, BQ }

impl Display for CastleZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
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
            CastleZone::WK | CastleZone::WQ => Side::W,
            CastleZone::BK | CastleZone::BQ => Side::B,
        }
    }

    /// Return the kingside zone for the given side.
    pub fn kingside(side: Side) -> CastleZone {
        match side {
            Side::W => CastleZone::WK,
            Side::B => CastleZone::BK,
        }
    }

    /// Return the queenside zone for the given side.
    pub fn queenside(side: Side) -> CastleZone {
        match side {
            Side::W => CastleZone::WQ,
            Side::B => CastleZone::BQ,
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
    pub fn lift(self) -> EnumSet<CastleZone> {
        enum_set!(self)
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
        assert_eq!(CastleZone::WK, "wk".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::WK, "WK".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::WQ, "wq".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::WQ, "WQ".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::BK, "bk".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::BK, "BK".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::BQ, "bq".parse::<CastleZone>().unwrap());
        assert_eq!(CastleZone::BQ, "BQ".parse::<CastleZone>().unwrap());
    }
}
