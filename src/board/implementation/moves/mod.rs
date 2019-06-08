use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::implementation::BoardImpl;
use crate::board::Move;
use crate::pieces::Piece;
use crate::board::MoveComputationType;

#[cfg(test)]
mod test;

mod control;
mod enpassant_source;
mod pinning;
mod constraints;

const WHITE_SLIDERS: [Piece; 3] = [Piece::WB, Piece::WR, Piece::WQ];
const BLACK_SLIDERS: [Piece; 3] = [Piece::BB, Piece::BR, Piece::BQ];
const FILES: [BitBoard; 8] = BitBoard::FILES;

fn nbrq<'a>(side: Side) -> &'a [Piece; 4] {
    match side {
        Side::White => &[Piece::WN, Piece::WB, Piece::WR, Piece::WQ],
        Side::Black => &[Piece::BN, Piece::BB, Piece::BR, Piece::BQ],
    }
}

fn pnbrq<'a>(side: Side) -> &'a [Piece; 5] {
    match side {
        Side::White => &[Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ],
        Side::Black => &[Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ],
    }
}

impl BoardImpl {
    /// Computes all legal moves for the active side at this position.
    pub fn compute_moves(&self) -> Vec<Move> {
        unimplemented!()
        //self.compute_moves_impl(false)
    }

    /// Used in quiescent search to find quiet positions, if the king is
    /// in check this method calculates any legal move otherwise we just
    /// compute legal moves which result in the capture of an enemy piece.
    ///
    /// TODO Add method which additionally computes checking moves. In quiescent
    ///      search we would allow attacks + checks + escapes until certain depth
    ///      then just attacks + escapes.
    pub fn compute_attacks_or_escapes(&self) -> Vec<Move> {
        unimplemented!()
        //self.compute_moves_impl(true)
    }


    fn compute_moves_impl(&self, computation_type: MoveComputationType) -> Vec<Move> {
        let passive_control = self.compute_control(self.active.reflect());
        if passive_control.intersects(self.pieces.locations(Piece::king(self.active))) {
            self.compute_moves_assuming_in_check(passive_control)
        } else {
            match computation_type {
                MoveComputationType::All => unimplemented!(),
                MoveComputationType::Attacks => unimplemented!(),
                MoveComputationType::AttacksChecks => unimplemented!(),
            }
            unimplemented!()
            //self.compute_moves_assuming_no_check(passive_control, force_attacks)
        }
    }

    /// Assuming the active side is in check, compute the legal escape moves.
    fn compute_moves_assuming_in_check(&self, passive_control: BitBoard) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(10);
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        let active_king = Piece::king(self.active);
        let king_loc = self.pieces.king_location(self.active);
        dest.extend(Move::standards(
            active_king,
            king_loc,
            unchecked_moves(active_king, king_loc) - passive_control,
        ));

        let attackers = self.compute_king_attackers(whites, blacks);
        if attackers.len() == 1 {
            let (attacker, loc) = attackers[0];
            let block_constraint = if attacker.is_knight() {
                loc.lift()
            } else {
                BitBoard::cord(loc, king_loc)
            };
            dest.extend(self.compute_pnbrq_moves(
                unchecked_moves,
                block_constraint,
                block_constraint,
            ));
        }
        dest
    }

    fn compute_king_attackers(&self, whites: BitBoard, blacks: BitBoard) -> Vec<(Piece, Square)> {
        let king_loc = self.pieces.king_location(self.active);
        pnbrq(self.active.reflect())
            .iter()
            .flat_map(|&p| self.pieces.locations(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites, blacks).contains(king_loc))
            .collect()
    }

    fn compute_pnbrq_moves<F>(
        &self,
        unchecked_moves: F,
        nbrq_constraint: BitBoard,
        pawn_constraint: BitBoard,
    ) -> Vec<Move>
    where
        F: Fn(Piece, Square) -> BitBoard,
    {
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        let pinned = self.compute_pinned();
        if pinned.pinned_locations.is_empty() {
            dest.extend(self.legal_moves(nbrq(self.active), &unchecked_moves, |_| nbrq_constraint));
            dest.extend(self.legal_pawn_moves(&unchecked_moves, |_| pawn_constraint));
        } else {
            let nbrq = nbrq(self.active);
            let compute_nbrq_constraint =
                |loc: Square| pinned.compute_constraint_area(loc, nbrq_constraint);
            dest.extend(self.legal_moves(nbrq, &unchecked_moves, &compute_nbrq_constraint));
            let compute_pawn_constraint =
                |loc: Square| pinned.compute_constraint_area(loc, pawn_constraint);
            dest.extend(self.legal_pawn_moves(&unchecked_moves, &compute_pawn_constraint));
        }
        dest
    }

    /// Assuming the active side is not in check, compute the legal moves
    /// with the option of forcing moves which result in an enemy capture.
    fn compute_moves_assuming_no_check(
        &self,
        passive_control: BitBoard,
        force_attacks: bool,
    ) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(50);
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        let (nbrq_cons, pawn_cons, king_cons) =
            self.compute_area_constraints(force_attacks, (whites, blacks), passive_control);

        if !force_attacks {
            dest.extend(self.compute_castle_moves(passive_control, whites | blacks));
        }
        dest.extend(self.compute_pnbrq_moves(&unchecked_moves, nbrq_cons, pawn_cons));
        let king = &[Piece::king(self.active)];
        dest.extend(self.legal_moves(king, &unchecked_moves, |_| king_cons));
        dest
    }

    /// Computes the constraint areas for nbrq, pawn and king pieces returning
    /// the three results as a triple in that order.
    fn compute_area_constraints(
        &self,
        force_attacks: bool,
        locations: (BitBoard, BitBoard),
        passive_control: BitBoard,
    ) -> (BitBoard, BitBoard, BitBoard) {
        let (whites, blacks) = locations;
        // Constraint for nbrq
        let nbrq = if force_attacks {
            match self.active {
                Side::White => blacks,
                Side::Black => whites,
            }
        } else {
            BitBoard::ALL
        };
        let enpassant = self.enpassant.map_or(BitBoard::EMPTY, |x| x.lift());
        (nbrq, nbrq | enpassant, nbrq - passive_control)
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
        let active_pawn = Piece::pawn(self.active);

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
            enpassant_source::squares(self.active, sq)
        });
        let promotion_rank = self.active.pawn_last_rank();
        let pawn_locs = self.pieces.locations(Piece::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

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
}
