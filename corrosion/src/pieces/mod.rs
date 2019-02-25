use crate::bitboard::BitBoard;
use crate::square::Square;

pub mod pawns;
pub mod knights;

pub trait Piece {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
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
