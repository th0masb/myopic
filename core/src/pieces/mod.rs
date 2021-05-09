use crate::{BitBoard, Side, Square};
use anyhow::{anyhow, Error, Result};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use enumset::EnumSetType;

mod kings;
mod knights;
mod pawns;
mod sliding;

/// Value type wrapping a single integer representing one of the 12
/// different pieces in a game of chess.
#[derive(Debug, EnumSetType, Ord, PartialOrd, Hash)]
#[rustfmt::skip]
pub enum Piece {
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        Piece::all()
            .find(|p| p.to_string() == lower)
            .ok_or(anyhow!("Cannot parse {} as piece", s))
    }
}

impl Piece {
    /// Create an iterator traversing over all pieces in order.
    pub fn all() -> impl Iterator<Item = Piece> {
        ALL.iter().cloned()
    }

    /// Create an iterator traversing over all white pieces in order.
    pub fn whites() -> impl Iterator<Item = Piece> {
        WHITE.iter().cloned()
    }

    /// Create an iterator traversing over all black pieces in order.
    pub fn blacks() -> impl Iterator<Item = Piece> {
        BLACK.iter().cloned()
    }

    /// Returns the king which belongs to the given side.
    pub fn king(side: Side) -> Piece {
        match side {
            Side::White => Piece::WK,
            Side::Black => Piece::BK,
        }
    }

    /// Returns the queen which belongs to the given side.
    pub fn queen(side: Side) -> Piece {
        match side {
            Side::White => Piece::WQ,
            Side::Black => Piece::BQ,
        }
    }

    /// Returns the rook belonging to the given side.
    pub fn rook(side: Side) -> Piece {
        match side {
            Side::White => Piece::WR,
            Side::Black => Piece::BR,
        }
    }

    /// Returns the pawn which belongs to the given side.
    pub fn pawn(side: Side) -> Piece {
        match side {
            Side::White => Piece::WP,
            Side::Black => Piece::BP,
        }
    }

    /// Returns a slice containing all pieces belonging to the given side.
    pub fn of(side: Side) -> impl Iterator<Item = Piece> {
        match side {
            Side::White => (&WHITE).iter().cloned(),
            Side::Black => (&BLACK).iter().cloned(),
        }
    }

    /// Returns the side that this piece belongs to.
    pub fn side(self) -> Side {
        if (self as u8) < 6 {
            Side::White
        } else {
            Side::Black
        }
    }

    /// Checks whether this piece is either a white or black pawn.
    pub fn is_pawn(self) -> bool {
        (self as u8) % 6 == 0
    }

    /// Checks whether this piece is either a white or black knight.
    pub fn is_knight(self) -> bool {
        (self as u8) % 6 == 1
    }

    /// Computes the control set for this piece given it's location and the
    /// locations of all the white and black pieces on the board.
    pub fn control(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::CONTROL_FN[self as usize](loc, whites, blacks)
    }

    /// Computes the control set for this piece given it's location on an
    /// empty board.
    pub fn empty_control(self, loc: Square) -> BitBoard {
        self.control(loc, BitBoard::EMPTY, BitBoard::EMPTY)
    }

    /// Computes the set of legal moves for this piece given it's location
    /// and the locations of all the white and black pieces on the board.
    /// Note that this method does not take into account special restrictions
    /// for or due to the king, e.g. can't move in such a way to put the king
    /// into check.
    pub fn moves(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::MOVE_FN[self as usize](loc, whites, blacks)
    }

    const CONTROL_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_control,
        knights::control,
        sliding::bishops::control,
        sliding::rooks::control,
        sliding::queens::control,
        kings::control,
        pawns::black_control,
        knights::control,
        sliding::bishops::control,
        sliding::rooks::control,
        sliding::queens::control,
        kings::control,
    ];

    const MOVE_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_moves,
        knights::white_moves,
        sliding::bishops::white_moves,
        sliding::rooks::white_moves,
        sliding::queens::white_moves,
        kings::white_moves,
        pawns::black_moves,
        knights::black_moves,
        sliding::bishops::black_moves,
        sliding::rooks::black_moves,
        sliding::queens::black_moves,
        kings::black_moves,
    ];
}

/// Constant piece groupings.
const ALL: [Piece; 12] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

const WHITE: [Piece; 6] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
];

const BLACK: [Piece; 6] = [
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

#[cfg(test)]
mod test {
    use crate::Piece;

    #[test]
    fn display() {
        assert_eq!("wp", Piece::WP.to_string().as_str());
        assert_eq!("br", Piece::BR.to_string().as_str());
    }

    #[test]
    fn from_str() {
        assert_eq!(Piece::WP, "wp".parse::<Piece>().unwrap());
        assert_eq!(Piece::WP, "WP".parse::<Piece>().unwrap());
        assert_eq!(Piece::BQ, "bq".parse::<Piece>().unwrap());
        assert!("ba".parse::<Piece>().is_err());
        assert!("bqs".parse::<Piece>().is_err());
        assert!("wxk".parse::<Piece>().is_err());
    }
}
