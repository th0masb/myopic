use crate::base::bitboard::BitBoard;
use crate::base::square::Square;

pub mod kings;
pub mod knights;
pub mod pawns;
pub mod sliding;

pub trait Piece {
    fn index(&self) -> usize;

    fn id(&self) -> &'static str;

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

fn all() -> Vec<Box<Piece>> {
    let mut dest: Vec<Box<Piece>> = Vec::with_capacity(12);
    dest.push(Box::new(WhitePawn));
    dest.push(Box::new(WhiteKnight));
    dest.push(Box::new(WhiteBishop));
    dest.push(Box::new(WhiteRook));
    dest.push(Box::new(WhiteQueen));
    dest.push(Box::new(WhiteKing));
    dest.push(Box::new(BlackPawn));
    dest.push(Box::new(BlackKnight));
    dest.push(Box::new(BlackBishop));
    dest.push(Box::new(BlackRook));
    dest.push(Box::new(BlackQueen));
    dest.push(Box::new(BlackKing));
    dest
}

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
        let id = ALL[0].id();
        //assert_eq!(WhitePawn.id(), *(&ALL[WhitePawn.index()].id()));
    }
}
