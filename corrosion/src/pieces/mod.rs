use crate::square::Square;
use crate::bitboard::BitBoard;

pub mod pawns;


pub trait Piece {
    fn control_set(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;

//    fn move_set(location: Square, white_locations: BitBoard, black_locations: BitBoard);
//
//    fn attack_set(location: Square, white_locations: BitBoard, black_locations: BitBoard);
}