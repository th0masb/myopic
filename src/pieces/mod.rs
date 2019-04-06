use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;

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
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhitePawn;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackPawn;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhiteKnight;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackKnight;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhiteBishop;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackBishop;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhiteRook;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackRook;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhiteQueen;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackQueen;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WhiteKing;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BlackKing;

const ALL: [&'static dyn Piece; 12] = [
    &WhitePawn,
    &WhiteKnight,
    &WhiteBishop,
    &WhiteRook,
    &WhiteQueen,
    &WhiteKing,
    &BlackPawn,
    &BlackKnight,
    &BlackBishop,
    &BlackRook,
    &BlackQueen,
    &BlackKing,
];

const WHITE: [&'static dyn Piece; 6] = [
    &WhitePawn,
    &WhiteKnight,
    &WhiteBishop,
    &WhiteRook,
    &WhiteQueen,
    &WhiteKing,
];

const BLACK: [&'static dyn Piece; 6] = [
    &BlackPawn,
    &BlackKnight,
    &BlackBishop,
    &BlackRook,
    &BlackQueen,
    &BlackKing,
];

const PAWNS: [&'static dyn Piece; 2] = [&WhitePawn, &BlackPawn];
const KNIGHTS: [&'static dyn Piece; 2] = [&WhiteKnight, &BlackKnight];
const BISHOPS: [&'static dyn Piece; 2] = [&WhiteBishop, &BlackBishop];
const ROOKS: [&'static dyn Piece; 2] = [&WhiteRook, &BlackRook];
const QUEENS: [&'static dyn Piece; 2] = [&WhiteQueen, &BlackQueen];
const KINGS: [&'static dyn Piece; 2] = [&WhiteKing, &BlackKing];


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
