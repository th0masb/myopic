use crate::square::Square;
use crate::bitboard::BitBoard;

mod pawns;


trait Piece {
    fn control_set(location: Square, white_locations: BitBoard, black_locations: BitBoard);

    fn move_set(location: Square, white_locations: BitBoard, black_locations: BitBoard);

    fn attack_set(location: Square, white_locations: BitBoard, black_locations: BitBoard);
}