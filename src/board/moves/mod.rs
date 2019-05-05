use crate::board::Board;
use crate::board::Move;
use crate::board::PieceRef;
use crate::base::square::Square;
use crate::base::bitboard::BitBoard;
use crate::base::Side;
use crate::pieces;

type PinnedPiece = (PieceRef, Square, BitBoard);
type PinnedSet = (BitBoard, Vec<PinnedPiece>);

impl Board {
    fn compute_pinned(&self) -> PinnedSet {
        let mut pinned: Vec<PinnedPiece> = Vec::with_capacity(2);
        let mut pinned_locs = BitBoard::EMPTY;

        unimplemented!()
    }

    fn compute_rook_pinned(&self, active_army: BitBoard, active_king: BitBoard) -> PinnedSet {
        let mut pinned: Vec<PinnedPiece> = Vec::with_capacity(2);
        let mut pinned_locs = BitBoard::EMPTY;
        let passive_rook = match self.active {Side::White => pieces::BR, _ => pieces::WR};
        for rook_loc in self.pieces.locations(passive_rook) {

        }

        unimplemented!()
    }

    fn compute_bishop_pinned(&self, active_army: BitBoard, active_king: BitBoard) -> PinnedSet {
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