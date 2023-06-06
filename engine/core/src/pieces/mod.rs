use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use enumset::EnumSetType;
use enum_map::Enum;

use crate::{BitBoard, Side, Square};

mod kings;
mod knights;
mod pawns;
mod sliding;

#[derive(Debug, PartialOrd, Ord, Hash, Enum, EnumSetType)]
#[rustfmt::skip]
pub enum Class { P, N, B, R, Q, K }

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Piece(pub Side, pub Class);

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}{:?}", self.0, self.1).to_lowercase())
    }
}

impl FromStr for Class {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "p" | "P" => Ok(Class::P),
            "n" | "N" => Ok(Class::N),
            "b" | "B" => Ok(Class::B),
            "r" | "R" => Ok(Class::R),
            "q" | "Q" => Ok(Class::Q),
            "k" | "K" => Ok(Class::K),
            _ => Err(anyhow!("{} not a piece type", s))
        }
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            Err(anyhow!("{} is not a valid piece", s))
        } else {
            Ok(Piece(s[0..1].parse()?, s[1..2].parse()?))
        }
    }
}

impl Piece {
    /// Computes the control set for this piece given it's location and the
    /// locations of all the occupied squares on the board.
    pub fn control(&self, loc: Square, occupied: BitBoard) -> BitBoard {
        match self.1 {
            Class::N => knights::control(loc),
            Class::K => kings::control(loc),
            Class::B => sliding::bishops::control(loc, occupied),
            Class::R => sliding::rooks::control(loc, occupied),
            Class::Q => sliding::bishops::control(loc, occupied) |
                sliding::rooks::control(loc, occupied),
            Class::P => match self.0 {
                Side::W => pawns::white_control(loc),
                Side::B => pawns::black_control(loc),
            }
        }
    }

    /// Computes the control set for this piece given it's location on an
    /// empty board.
    pub fn empty_control(&self, loc: Square) -> BitBoard {
        self.control(loc, BitBoard::EMPTY)
    }

    /// Computes the set of legal moves for this piece given it's location
    /// and the locations of all the white and black pieces on the board.
    /// Note that this method does not take into account special restrictions
    /// for or due to the king, e.g. can't move in such a way to put the king
    /// into check.
    pub fn moves(&self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        let friendly = if self.0 == Side::W { whites } else { blacks };
        (match self.1 {
            Class::P => match self.0 {
                Side::W => pawns::white_moves(loc, whites, blacks),
                Side::B => pawns::black_moves(loc, whites, blacks),
            }
            _ => self.control(loc, whites | blacks)
        }) - friendly
    }
}

#[cfg(test)]
mod test {
    use crate::{Piece, Side};
    use crate::pieces::Class;

    #[test]
    fn display() {
        assert_eq!("wp", Piece(Side::W, Class::P).to_string().as_str());
        assert_eq!("br", Piece(Side::B, Class::R).to_string().as_str());
    }

    #[test]
    fn from_str() {
        assert_eq!(Piece(Side::W, Class::P), "wp".parse::<Piece>().unwrap());
        assert_eq!(Piece(Side::W, Class::P), "WP".parse::<Piece>().unwrap());
        assert_eq!(Piece(Side::B, Class::Q), "bq".parse::<Piece>().unwrap());
        assert!("ba".parse::<Piece>().is_err());
        assert!("bqs".parse::<Piece>().is_err());
        assert!("wxk".parse::<Piece>().is_err());
    }
}
