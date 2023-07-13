use crate::{Corner, Piece, reflect_corner, reflect_piece, reflect_side, reflect_square, Side, Square, Symmetric};
use crate::moves::Move::Standard;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    Standard { moving: Piece, from: Square, dest: Square, capture: Option<Piece> },
    Enpassant { side: Side, from: Square, dest: Square, capture: Square },
    Promotion { from: Square, dest: Square, promoted: Piece, capture: Option<Piece> },
    Castle { corner: Corner },
    Null,
}

impl Symmetric for Move {
    fn reflect(&self) -> Self {
        use Move::*;
        match self {
            Null => Null,
            Castle { corner } => Castle { corner: reflect_corner(*corner) },
            Standard { moving, from, dest, capture } => Standard {
                moving: reflect_piece(*moving),
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                capture: capture.map(|p| reflect_piece(p)),
            },
            Enpassant { side, from, dest, capture } => Enpassant {
                side: reflect_side(*side),
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                capture: reflect_square(*capture),
            },
            Promotion { from, dest, promoted, capture } => Promotion {
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                promoted: reflect_piece(*promoted),
                capture: capture.map(|p| reflect_piece(p)),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Moves<'a> {
    All,
    Are(MoveFacet),
    AreAny(&'a [MoveFacet]),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum MoveFacet {
    Checking,
    Attacking,
    Promoting,
}
