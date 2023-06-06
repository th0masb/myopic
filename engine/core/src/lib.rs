use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub use anyhow;
pub use enum_map;
use enum_map::Enum;
pub use enumset;
use enumset::EnumSetType;

pub use bitboard::BitBoard;
pub use pieces::{Piece, Class};
pub use reflectable::Reflectable;
pub use square::Square;

mod bitboard;
pub mod hash;
mod pieces;
mod reflectable;
mod square;

#[derive(Debug, PartialOrd, Ord, Hash, Enum, EnumSetType)]
#[rustfmt::skip]
pub enum Side { W, B }

#[derive(Debug, PartialOrd, Ord, Hash, Enum, EnumSetType)]
#[rustfmt::skip]
pub enum Flank { K, Q }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Corner(pub Side, pub Flank);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Line(pub Square, pub Square);

#[derive(Debug, EnumSetType, Hash, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Dir { N, E, S, W, NE, SE, SW, NW, NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW }

impl Line {
    pub fn king_castling(Corner(side, flank): Corner) -> Line {
        match (side, flank) {
            (Side::W, Flank::K) => Line(Square::E1, Square::G1),
            (Side::W, Flank::Q) => Line(Square::E1, Square::C1),
            (Side::B, Flank::K) => Line(Square::E8, Square::G8),
            (Side::B, Flank::Q) => Line(Square::E8, Square::C8),
        }
    }

    pub fn rook_castling(Corner(side, flank): Corner) -> Line {
        match (side, flank) {
            (Side::W, Flank::K) => Line(Square::H1, Square::F1),
            (Side::W, Flank::Q) => Line(Square::A1, Square::D1),
            (Side::B, Flank::K) => Line(Square::H8, Square::F8),
            (Side::B, Flank::Q) => Line(Square::A8, Square::D8),
        }
    }
}

impl Dir {
    fn dr_df(self) -> (i8, i8) {
        match self {
            Dir::N => (1, 0),
            Dir::E => (0, -1),
            Dir::S => (-1, 0),
            Dir::W => (0, 1),
            Dir::NE => (1, -1),
            Dir::SE => (-1, -1),
            Dir::SW => (-1, 1),
            Dir::NW => (1, 1),
            Dir::NNE => (2, -1),
            Dir::NEE => (1, -2),
            Dir::SEE => (-1, -2),
            Dir::SSE => (-2, -1),
            Dir::SSW => (-2, 1),
            Dir::SWW => (-1, 2),
            Dir::NWW => (1, 2),
            Dir::NNW => (2, 1),
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::W => write!(f, "w"),
            Side::B => write!(f, "b"),
        }
    }
}

impl FromStr for Side {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" | "W" => Ok(Side::W),
            "b" | "B" => Ok(Side::B),
            _ => Err(anyhow::anyhow!("Cannot parse Side from {}", s)),
        }
    }
}

impl FromStr for Flank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "k" | "K" => Ok(Flank::K),
            "q" | "Q" => Ok(Flank::Q),
            _ => Err(anyhow::anyhow!("Cannot parse Side from {}", s)),
        }
    }
}

impl Side {
    /// Get the vertical direction in which a pawn on this side moves
    /// (north or south).
    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::W => Dir::N,
            Side::B => Dir::S,
        }
    }

    /// Get the rank on which a pawn on this side starts the game.
    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::W => BitBoard::RANKS[1],
            Side::B => BitBoard::RANKS[6],
        }
    }

    /// Get the rank to which a pawn on this side moves to following
    /// it's special two rank first move.
    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::W => BitBoard::RANKS[3],
            Side::B => BitBoard::RANKS[4],
        }
    }

    /// Get the rank a pawn on this side must be on for it to be able
    /// to promote on it's next move.
    pub fn pawn_promoting_from_rank(self) -> BitBoard {
        match self {
            Side::W => BitBoard::RANKS[6],
            Side::B => BitBoard::RANKS[1],
        }
    }

    /// The rank a pawn on this side will end up on after promoting to
    /// another piece.
    pub fn pawn_promoting_dest_rank(self) -> BitBoard {
        match self {
            Side::W => BitBoard::RANKS[7],
            Side::B => BitBoard::RANKS[0],
        }
    }

    pub fn parity(self) -> i32 {
        match self {
            Side::W => 1,
            Side::B => -1,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::Side;

    #[test]
    fn display() {
        assert_eq!("w", Side::W.to_string().as_str());
        assert_eq!("b", Side::B.to_string().as_str());
    }

    #[test]
    fn from_str() {
        assert_eq!(Side::W, "w".parse::<Side>().unwrap());
        assert_eq!(Side::W, "W".parse::<Side>().unwrap());
        assert_eq!(Side::B, "b".parse::<Side>().unwrap());
        assert_eq!(Side::B, "b".parse::<Side>().unwrap());
    }
}
