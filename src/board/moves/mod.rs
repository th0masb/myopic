use crate::board::Board;
use crate::board::Move;
use crate::board::PieceRef;
use crate::base::square::Square;
use crate::base::bitboard::BitBoard;

type PinnedPiece = (PieceRef, Square, BitBoard);

impl Board {
    fn compute_pinned(&self) -> (BitBoard, Vec<PinnedPiece>) {
        unimplemented!()
    }

    pub fn compute_moves(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn compute_attacks(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn has_legal_move(&self) -> bool {
        unimplemented!()
    }


}