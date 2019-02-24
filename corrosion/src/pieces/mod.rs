use crate::square::Square;
use crate::bitboard::BitBoard;

pub mod pawns;


pub trait Piece {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
}