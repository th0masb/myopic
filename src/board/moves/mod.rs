use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
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
#[cfg(test)]
mod test;

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

#[cfg(test)]
mod test_enpassant_source_squares {
    use crate::base::bitboard::constants::*;
    use crate::base::square::constants;
    use crate::base::Side;

    use super::enpassant_source_squares;

    #[test]
    fn test() {
        assert_eq!(
            H4 | F4,
            enpassant_source_squares(Side::Black, constants::G3)
        );
        assert_eq!(G4, enpassant_source_squares(Side::Black, constants::H3));
        assert_eq!(B4, enpassant_source_squares(Side::Black, constants::A3));
        assert_eq!(
            H5 | F5,
            enpassant_source_squares(Side::White, constants::G6)
        );
        assert_eq!(G5, enpassant_source_squares(Side::White, constants::H6));
        assert_eq!(B5, enpassant_source_squares(Side::White, constants::A6));
    }
}

fn nbrq<'a>(side: Side) -> &'a [Piece; 4] {
    match side {
        Side::White => &[pieces::WN, pieces::WB, pieces::WR, pieces::WQ],
        Side::Black => &[pieces::BN, pieces::BB, pieces::BR, pieces::BQ],
    }
}

impl Board {
    /// Computes all legal moves for the active side at this position.
    pub fn compute_moves(&self) -> Vec<Move> {
        let passive_control = self.compute_control(self.active.other());
        self.compute_moves_no_check(passive_control, false)
    }

    /// Used in quiescent search to find quiet positions, if the king is
    /// in check this method calculates any legal move otherwise we just
    /// compute legal moves which result in the capture of an enemy piece.
    pub fn compute_attacks_or_escapes(&self) -> Vec<Move> {
        let passive_control = self.compute_control(self.active.other());
        self.compute_moves_no_check(passive_control, true)
    }

    //    pub fn has_legal_move(&self) -> bool {
    //        unimplemented!()
    //    }

    fn compute_moves_in_check(&self, passive_control: BitBoard) -> Vec<Move> {
        unimplemented!()
    }

    fn compute_moves_no_check(&self, passive_control: BitBoard, force_attacks: bool) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(50);
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        let (nbrq_cons, pawn_cons, king_cons) =
            self.compute_area_constraints(force_attacks, (whites, blacks), passive_control);

        if !force_attacks {
            dest.extend(self.compute_castle_moves(passive_control, whites | blacks));
        }

        let pinned = self.compute_pinned();
        if pinned.0.is_empty() {
            dest.extend(self.legal_moves(nbrq(self.active), &unchecked_moves, |_| nbrq_cons));
            dest.extend(self.legal_pawn_moves(&unchecked_moves, |_| pawn_cons));
        } else {
            dest.extend(
                self.legal_moves(nbrq(self.active), &unchecked_moves, |loc| {
                    compute_constraint_area(loc, &pinned, nbrq_cons)
                }),
            );
            dest.extend(self.legal_pawn_moves(&unchecked_moves, |loc| {
                compute_constraint_area(loc, &pinned, pawn_cons)
            }));
        }

        let king = &[pieces::king(self.active)];
        dest.extend(self.legal_moves(king, &unchecked_moves, |_| king_cons));
        dest
    }

    fn compute_area_constraints(
        &self,
        force_attacks: bool,
        locations: (BitBoard, BitBoard),
        passive_control: BitBoard,
    ) -> (BitBoard, BitBoard, BitBoard) {
        let (whites, blacks) = locations;
        let nbrq_cons = if force_attacks {
            match self.active {
                Side::White => blacks,
                Side::Black => whites,
            }
        } else {
            BitBoard::ALL
        };
        (
            nbrq_cons,
            nbrq_cons | self.enpassant.map_or(BitBoard::EMPTY, |x| x.lift()),
            nbrq_cons - passive_control,
        )
    }

    fn compute_castle_moves(&self, passive_control: BitBoard, piece_locs: BitBoard) -> Vec<Move> {
        let p1 = |z: CastleZone| !passive_control.intersects(z.uncontrolled_requirement());
        let p2 = |z: CastleZone| !piece_locs.intersects(z.unoccupied_requirement());
        self.castling
            .rights()
            .iter()
            .filter(|&z| p1(z) && p2(z))
            .map(Move::Castle)
            .collect()
    }

    fn legal_pawn_moves<F, G>(&self, compute_moves: F, compute_constraint: G) -> Vec<Move>
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
