use crate::base::bitboard::BitBoard;
use crate::base::Side;
use crate::base::square::Square;

use super::bishops;
use super::rooks;

pub fn control(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    bishops::control(loc, whites, blacks) | rooks::control(loc, whites, blacks)
}

pub fn white_moves(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    bishops::white_moves(loc, whites, blacks) | rooks::white_moves(loc, whites, blacks)
}

pub fn black_moves(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    bishops::black_moves(loc, whites, blacks) | rooks::black_moves(loc, whites, blacks)
}


