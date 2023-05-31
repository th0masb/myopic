use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub use anyhow;
pub use enum_map;
use enum_map::Enum;
pub use enumset;
use enumset::EnumSetType;

pub use bitboard::constants;
pub use bitboard::BitBoard;
pub use castlezone::CastleZone;
pub use pieces::Piece;
pub use reflectable::Reflectable;
pub use square::Square;

mod bitboard;
mod castlezone;
pub mod hash;
mod pieces;
mod reflectable;
mod square;

/// Represents the two different teams in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Side {
    White,
    Black,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::White => write!(f, "w"),
            Side::Black => write!(f, "b"),
        }
    }
}

impl FromStr for Side {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" | "W" => Ok(Side::White),
            "b" | "B" => Ok(Side::Black),
            _ => Err(anyhow::anyhow!("Cannot parse Side from {}", s)),
        }
    }
}

impl Side {
    /// Get the vertical direction in which a pawn on this side moves
    /// (north or south).
    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::White => Dir::N,
            Side::Black => Dir::S,
        }
    }

    /// Get the rank on which a pawn on this side starts the game.
    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[1],
            Side::Black => BitBoard::RANKS[6],
        }
    }

    /// Get the rank to which a pawn on this side moves to following
    /// it's special two rank first move.
    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[3],
            Side::Black => BitBoard::RANKS[4],
        }
    }

    /// Get the rank a pawn on this side must be on for it to be able
    /// to promote on it's next move.
    pub fn pawn_promoting_from_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[6],
            Side::Black => BitBoard::RANKS[1],
        }
    }

    /// The rank a pawn on this side will end up on after promoting to
    /// another piece.
    pub fn pawn_promoting_dest_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[7],
            Side::Black => BitBoard::RANKS[0],
        }
    }

    pub fn parity(self) -> i32 {
        match self {
            Side::White => 1,
            Side::Black => -1,
        }
    }
}

/// Type representing a square on a chessboard.
#[derive(Debug, EnumSetType, Hash, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Dir {
    N, E, S, W,
    NE, SE, SW, NW,
    NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW,
}

#[cfg(test)]
mod test {
    use crate::Side;

    #[test]
    fn display() {
        assert_eq!("w", Side::White.to_string().as_str());
        assert_eq!("b", Side::Black.to_string().as_str());
    }

    #[test]
    fn from_str() {
        assert_eq!(Side::White, "w".parse::<Side>().unwrap());
        assert_eq!(Side::White, "W".parse::<Side>().unwrap());
        assert_eq!(Side::Black, "b".parse::<Side>().unwrap());
        assert_eq!(Side::Black, "b".parse::<Side>().unwrap());
    }
}
