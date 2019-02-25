use crate::square::Square;
use crate::bitboard::BitBoard;

pub mod pawns;


pub trait Piece {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
}


pub struct WhitePawn;
pub struct BlackPawn;

pub struct WhiteKnight;
pub struct BlackKnight;

pub struct WhiteBishop;
pub struct BlackBishop;

pub struct WhiteRook;
pub struct BlackRook;

pub struct WhiteQueen;
pub struct BlackQueen;

pub struct WhiteKing;
pub struct BlackKing;
