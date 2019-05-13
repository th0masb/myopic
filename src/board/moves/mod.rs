use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Board;
use crate::board::Move;
use crate::pieces;
use crate::pieces::Piece;

#[cfg(test)]
mod control_test;
#[cfg(test)]
mod pin_test;

type PinnedPiece = (Square, BitBoard);
type PinnedSet = (BitBoard, Vec<PinnedPiece>);

const WHITE_SLIDERS: [Piece; 3] = [pieces::WB, pieces::WR, pieces::WQ];
const BLACK_SLIDERS: [Piece; 3] = [pieces::BB, pieces::BR, pieces::BQ];
const FILES: [BitBoard; 8] = BitBoard::FILES;

fn compute_constraint_area(piece_loc: Square, pinned: &PinnedSet, existing: BitBoard) -> BitBoard {
    if pinned.0.contains(piece_loc) {
        (&pinned.1)
            .into_iter()
            .find(|(sq, _)| *sq == piece_loc)
            .unwrap()
            .1
            & existing
    } else {
        BitBoard::ALL & existing
    }
}

/// TODO Could have adjacent files in a constant array
fn enpassant_source_squares(active: Side, enpassant_target: Square) -> BitBoard {
    let fi = enpassant_target.file_index() as usize;
    let adjacent_files = match fi % 7 {
        0 => {
            if fi == 0 {
                FILES[1]
            } else {
                FILES[6]
            }
        }
        _ => FILES[fi + 1] | FILES[fi - 1],
    };
    adjacent_files & active.other().pawn_third_rank()
}

fn nbrq<'a>(side: Side) -> &'a [Piece; 4] {
    match side {
        Side::White => &[pieces::WN, pieces::WB, pieces::WR, pieces::WQ],
        Side::Black => &[pieces::BN, pieces::BB, pieces::BR, pieces::BQ],
    }
}

impl Board {
    pub fn compute_moves(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn compute_attacks(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn has_legal_move(&self) -> bool {
        unimplemented!()
    }

    // TODO need to do castle moves
    fn compute_moves_impl(
        &self,
        constraints: (BitBoard, BitBoard, BitBoard),
        passive_control: BitBoard,
    ) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(50);
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        let (nbrq_constraint, pawn_constraint, king_constraint) = constraints;
        let pinned = self.compute_pinned();
        let active = self.active;



        if pinned.0.is_empty() {
            dest.extend(self.legal_moves(nbrq(active), &unchecked_moves, |_| nbrq_constraint));
            dest.extend(self.pawn_moves(&unchecked_moves, |_| pawn_constraint));
        } else {
            dest.extend(self.legal_moves(nbrq(active), &unchecked_moves, |loc| {
                compute_constraint_area(loc, &pinned, nbrq_constraint)
            }));
            dest.extend(self.pawn_moves(&unchecked_moves, |loc| {
                compute_constraint_area(loc, &pinned, pawn_constraint)
            }));
        }

        let king = &[pieces::king(active)];
        dest.extend(self.legal_moves(king, &unchecked_moves, |_| king_constraint));
        dest
    }

    fn compute_castle_moves(&self, passive_control: BitBoard, piece_locs: BitBoard) -> Vec<Move> {
        unimplemented!()
    }

    //    fn enpassant_bitboard(&self) -> BitBoard {
    //        self.enpassant.map_or(BitBoard::EMPTY, |sq| sq.lift())
    //    }

    fn pawn_moves<F, G>(&self, compute_moves: F, compute_constraint: G) -> Vec<Move>
    where
        F: Fn(Piece, Square) -> BitBoard,
        G: Fn(Square) -> BitBoard,
    {
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let active_pawn = pieces::pawn(self.active);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(active_pawn, location) & compute_constraint(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }
        for location in enpassant {
            if compute_constraint(location).contains(self.enpassant.unwrap()) {
                dest.push(Move::Enpassant(location));
            }
        }
        for location in promotion {
            let targets = compute_moves(active_pawn, location) & compute_constraint(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source = self.enpassant.map_or(BitBoard::EMPTY, |sq| {
            enpassant_source_squares(self.active, sq)
        });
        let promotion_rank = self.active.pawn_last_rank();
        let pawn_locs = self.pieces.locations(pieces::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    /// TODO Could reduce the arguments of this further to compute_targets
    fn legal_moves<F, G>(&self, pieces: &[Piece], unchecked_moves: F, constraint: G) -> Vec<Move>
    where
        F: Fn(Piece, Square) -> BitBoard,
        G: Fn(Square) -> BitBoard,
    {
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        // Add standard moves for pieces which aren't pawns or king
        for &piece in pieces.iter() {
            for location in self.pieces.locations(piece) {
                let moves = unchecked_moves(piece, location) & constraint(location);
                dest.extend(Move::standards(piece, location, moves));
            }
        }
        dest
    }

    /// Computes the total area of control on the board for a given side.
    /// TODO Improve efficiency by treated all pawns as a block
    fn compute_control(&self, side: Side) -> BitBoard {
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let locs = |piece: Piece| self.pieces.locations(piece);
        let control = |piece: Piece, square: Square| piece.control(square, whites, blacks);
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
            Side::Black => WHITE_SLIDERS,
        };
        let locs = |p: Piece| self.pieces.locations(p);
        passive_sliders
            .iter()
            .flat_map(|&p| locs(p) & p.control(king_loc, BitBoard::EMPTY, BitBoard::EMPTY))
            .collect()
    }
}
