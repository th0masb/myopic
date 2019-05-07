use crate::board::Board;
use crate::board::Move;
use crate::pieces::PieceRef;
use crate::base::square::Square;
use crate::base::bitboard::BitBoard;
use crate::base::Side;
use crate::pieces;

#[cfg(test)]
mod pin_test;

type PinnedPiece = (PieceRef, Square, BitBoard);
type PinnedSet = (BitBoard, Vec<PinnedPiece>);

const WHITE_SLIDERS: [PieceRef; 3] = [pieces::WB, pieces::WR, pieces::WQ];
const BLACK_SLIDERS: [PieceRef; 3] = [pieces::BB, pieces::BR, pieces::BQ];

impl Board {

    pub fn compute_moves(&self) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(40);

        unimplemented!()
    }

    pub fn compute_attacks(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn has_legal_move(&self) -> bool {
        unimplemented!()
    }

    /// Computes the set of all active pieces which are pinned to the king,
    /// i.e have their movement areas constrained so that they do not move
    /// and leave the king in check.
    ///
    fn compute_pinned(&self) -> PinnedSet {
        let locs = |side: Side| self.pieces.side_locations(side);
        let (active, passive) = (locs(self.active), locs(self.active.other()));
        let king_loc = self.pieces.king_location(self.active);
        let mut pinned: Vec<PinnedPiece> = Vec::with_capacity(2);
        let mut pinned_locs = BitBoard::EMPTY;
        for potential_pinner in self.compute_potential_pinners(king_loc) {
            let cord = BitBoard::cord(king_loc, potential_pinner);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 {
                let pinned_loc = ((cord & active) - king_loc).into_iter().next().unwrap();
                pinned.push((self.pieces.piece_at(pinned_loc).unwrap(), pinned_loc, cord));
                pinned_locs |= pinned_loc;
            }
        }
        (pinned_locs, pinned)
    }

    fn compute_potential_pinners(&self, king_loc: Square) -> BitBoard {
        let passive_sliders = match self.active {Side::White => BLACK_SLIDERS, _ => WHITE_SLIDERS};
        let locs = |p: PieceRef| self.pieces.locations(p);
        passive_sliders.iter().flat_map(|&p| locs(p) & p.empty_control(king_loc)).collect()
    }
}