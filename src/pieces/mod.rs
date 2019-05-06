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
        unimplemented!()
    }
}

impl PartialEq<Piece> for Piece {
    fn eq(&self, other: &Piece) -> bool {
        self.index() == other.index()
    }
}

/// Constant static references to each white piece.
pub const WP: &'static dyn Piece = &WhitePawn;
pub const WN: &'static dyn Piece = &WhiteKnight;
pub const WB: &'static dyn Piece = &WhiteBishop;
pub const WR: &'static dyn Piece = &WhiteRook;
pub const WQ: &'static dyn Piece = &WhiteQueen;
pub const WK: &'static dyn Piece = &WhiteKing;

/// Constant static references to each black piece.
pub const BP: &'static dyn Piece = &BlackPawn;
pub const BN: &'static dyn Piece = &BlackKnight;
pub const BB: &'static dyn Piece = &BlackBishop;
pub const BR: &'static dyn Piece = &BlackRook;
pub const BQ: &'static dyn Piece = &BlackQueen;
pub const BK: &'static dyn Piece = &BlackKing;

/// Constant piece groupings.
pub const ALL: [&'static dyn Piece; 12] = [WP, WN, WB, WR, WQ, WK, BP, BN, BB, BR, BQ, BK];

pub const WHITE: [&'static dyn Piece; 6] = [WP, WN, WB, WR, WQ, WK];
pub const BLACK: [&'static dyn Piece; 6] = [BP, BN, BB, BR, BQ, BK];

pub const PAWNS: [&'static dyn Piece; 2] = [WP, BP];
pub const KNIGHTS: [&'static dyn Piece; 2] = [WN, BN];
pub const BISHOPS: [&'static dyn Piece; 2] = [WB, BB];
pub const ROOKS: [&'static dyn Piece; 2] = [WR, BR];
pub const QUEENS: [&'static dyn Piece; 2] = [WQ, BQ];
pub const KINGS: [&'static dyn Piece; 2] = [WK, BK];


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
