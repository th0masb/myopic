use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Board;
use crate::board::Move;
use crate::pieces;
use crate::pieces::PieceRef;

#[cfg(test)]
mod control_test;
#[cfg(test)]
mod pin_test;

type PinnedPiece = (Square, BitBoard);
type PinnedSet = (BitBoard, Vec<PinnedPiece>);

const WHITE_SLIDERS: [PieceRef; 3] = [pieces::WB, pieces::WR, pieces::WQ];
const BLACK_SLIDERS: [PieceRef; 3] = [pieces::BB, pieces::BR, pieces::BQ];

impl Board {
    pub fn compute_moves(&self) -> Vec<Move> {
        let pinned = self.compute_pinned();
        let passive_control = self.compute_control(self.active.other());
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        // Add standard moves for pieces which aren't pawns or king
        for &piece in Board::knight_and_sliders(self.active).iter() {
            for location in self.pieces.locations(piece) {
                let constraint = Board::compute_constraint_area(location, &pinned);
                let moves = piece.moves(location, whites, blacks) & constraint;
                dest.extend(Move::standards(piece, location, moves));
            }
        }

        unimplemented!()
    }

//    fn compute_knight_and_slider_moves(&self) -> impl Iterator<Item = Move> {
//
//    }

    fn knight_and_sliders(side: Side) -> [PieceRef; 4] {
        match side {
            Side::White => [pieces::WN, pieces::WB, pieces::WR, pieces::WQ],
            _ => [pieces::BN, pieces::BB, pieces::BR, pieces::BQ],
        }
    }

    pub fn compute_attacks(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn has_legal_move(&self) -> bool {
        unimplemented!()
    }

    fn compute_constraint_area(piece_loc: Square, pinned: &PinnedSet) -> BitBoard {
        let (all_pinned_locs, pinned_pieces) = pinned;
        if all_pinned_locs.contains(piece_loc) {
            pinned_pieces
                .into_iter()
                .find(|(sq, _)| *sq == piece_loc)
                .unwrap()
                .1
        } else {
            BitBoard::ALL
        }
    }

    /// Computes the total area of control on the board for a given side.
    /// TODO Improve efficiency by treated all pawns as a block
    fn compute_control(&self, side: Side) -> BitBoard {
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let locs = |piece: PieceRef| self.pieces.locations(piece);
        let control = |piece: PieceRef, square: Square| piece.control(square, whites, blacks);
        pieces::on_side(side)
            .iter()
            .flat_map(|&p| locs(p).into_iter().map(move |sq| control(p, sq)))
            .collect()
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
                pinned.push((pinned_loc, cord));
                pinned_locs |= pinned_loc;
            }
        }
        (pinned_locs, pinned)
    }

    fn compute_potential_pinners(&self, king_loc: Square) -> BitBoard {
        let passive_sliders = match self.active {
            Side::White => BLACK_SLIDERS,
            _ => WHITE_SLIDERS,
        };
        let locs = |p: PieceRef| self.pieces.locations(p);
        passive_sliders
            .iter()
            .flat_map(|&p| locs(p) & p.empty_control(king_loc))
            .collect()
    }
}
