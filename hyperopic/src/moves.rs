use crate::{Corner, Piece, Side, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    Standard { moving: Piece, from: Square, dest: Square, capture: Option<Piece> },
    Enpassant { side: Side, from: Square, dest: Square, capture: Square },
    Promotion { from: Square, dest: Square, promoted: Piece, capture: Option<Piece> },
    Castle { corner: Corner },
    Null,
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
