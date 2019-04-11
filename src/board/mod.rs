use crate::base::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::base::square::Square;

pub mod hash;
pub mod tables;

#[derive(PartialEq)]
struct PieceTracker {
    boards: Vec<BitBoard>,
    white: BitBoard,
    black: BitBoard,
    hash: u64,
    mid_eval: i32,
    end_eval: i32,
}

impl PieceTracker {
    fn contains(&self, piece: &dyn Piece, location: Square) -> bool {
        self.boards[piece.index()].contains(location)
    }

    fn add(&mut self, piece: &dyn Piece, location: Square) {
        debug_assert!(!self.boards[piece.index()].contains(location));
        self.boards[piece.index()] ^= location;
        // Need hasher and positional eval tables.
        panic!()
    }
}


