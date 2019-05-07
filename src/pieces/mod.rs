use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

mod kings;
mod knights;
mod pawns;
mod sliding;

pub type PieceRef = &'static dyn Piece;
pub trait Piece {
    fn index(&self) -> usize;

    fn id(&self) -> &str;

    fn side(&self) -> Side;

    fn control(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn moves(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn attacks(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn is_pawn(&self) -> bool {
        self.index() % 6 == 0
    }

    fn empty_control(&self, location: Square) -> BitBoard {
        self.control(location, BitBoard::EMPTY, BitBoard::EMPTY)
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.id())
    }
}

impl PartialEq<Piece> for Piece {
    fn eq(&self, other: &Piece) -> bool {
        self.index() == other.index()
    }
}

pub fn king(side: Side) -> PieceRef {
    match side {
        Side::White => WK,
        Side::Black => BK,
    }
}

pub fn pawn(side: Side) -> PieceRef {
    match side {
        Side::White => WP,
        Side::Black => BP,
    }
}

pub fn pieces(side: Side) -> &'static [PieceRef] {
    match side {
        Side::White => &WHITE,
        Side::Black => &BLACK,
    }
}

pub fn army(side: Side) -> [PieceRef; 5] {
    match side {
        Side::White => [WP, WN, WB, WR, WQ],
        Side::Black => [BP, BN, BB, BR, BQ],
    }
}

/// Constant static references to each white piece.
pub const WP: PieceRef = &WhitePawn;
pub const WN: PieceRef = &WhiteKnight;
pub const WB: PieceRef = &WhiteBishop;
pub const WR: PieceRef = &WhiteRook;
pub const WQ: PieceRef = &WhiteQueen;
pub const WK: PieceRef = &WhiteKing;

/// Constant static references to each black piece.
pub const BP: PieceRef = &BlackPawn;
pub const BN: PieceRef = &BlackKnight;
pub const BB: PieceRef = &BlackBishop;
pub const BR: PieceRef = &BlackRook;
pub const BQ: PieceRef = &BlackQueen;
pub const BK: PieceRef = &BlackKing;

/// Constant piece groupings.
pub const ALL: [PieceRef; 12] = [WP, WN, WB, WR, WQ, WK, BP, BN, BB, BR, BQ, BK];

pub const WHITE: [PieceRef; 6] = [WP, WN, WB, WR, WQ, WK];
pub const BLACK: [PieceRef; 6] = [BP, BN, BB, BR, BQ, BK];

pub const PAWNS:   [PieceRef; 2] = [WP, BP];
pub const KNIGHTS: [PieceRef; 2] = [WN, BN];
pub const BISHOPS: [PieceRef; 2] = [WB, BB];
pub const ROOKS:   [PieceRef; 2] = [WR, BR];
pub const QUEENS:  [PieceRef; 2] = [WQ, BQ];
pub const KINGS:   [PieceRef; 2] = [WK, BK];


/// Encapsulated singleton structs for each piece type.
#[derive(Copy, Clone, PartialEq, Eq)]
struct WhitePawn;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackPawn;

#[derive(Copy, Clone, PartialEq, Eq)]
struct WhiteKnight;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackKnight;

#[derive(Copy, Clone, PartialEq, Eq)]
struct WhiteBishop;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackBishop;

#[derive(Copy, Clone, PartialEq, Eq)]
struct WhiteRook;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackRook;

#[derive(Copy, Clone, PartialEq, Eq)]
struct WhiteQueen;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackQueen;

#[derive(Copy, Clone, PartialEq, Eq)]
struct WhiteKing;
#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackKing;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indices() {
        assert_eq!("wp", ALL[WhitePawn.index()].id());
        assert_eq!("wn", ALL[WhiteKnight.index()].id());
        assert_eq!("wb", ALL[WhiteBishop.index()].id());
        assert_eq!("wr", ALL[WhiteRook.index()].id());
        assert_eq!("wq", ALL[WhiteQueen.index()].id());
        assert_eq!("wk", ALL[WhiteKing.index()].id());
        assert_eq!("bp", ALL[BlackPawn.index()].id());
        assert_eq!("bn", ALL[BlackKnight.index()].id());
        assert_eq!("bb", ALL[BlackBishop.index()].id());
        assert_eq!("br", ALL[BlackRook.index()].id());
        assert_eq!("bq", ALL[BlackQueen.index()].id());
        assert_eq!("bk", ALL[BlackKing.index()].id());
    }
}
