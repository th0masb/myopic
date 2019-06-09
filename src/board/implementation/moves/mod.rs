use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::implementation::BoardImpl;
use crate::board::Move;
use crate::pieces::Piece;
use crate::board::MoveComputationType;
use crate::board::Board;
use crate::board::implementation::moves::constraints::MoveConstraints;

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
        let constraints = self.constraints(computation_type);
        let pawn_moves = self.compute_pawn_moves(&constraints);
        let nbrqk_moves = self.compute_nbrqk_moves(&constraints);
        let castle_moves = match computation_type {
            MoveComputationType::All => self.compute_castle_moves(&constraints),
            _ => Vec::with_capacity(0),
        };
        unimplemented!()
    }

    fn compute_nbrqk_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        unimplemented!()
    }

    fn compute_pawn_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let (active_pawn, (whites, blacks)) = (Piece::pawn(self.active), self.whites_blacks());
        let compute_moves = |loc: Square| active_pawn.moves(loc, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }
        for location in enpassant {
            if constraints.get(location).contains(self.enpassant.unwrap()) {
                dest.push(Move::Enpassant(location));
            }
        }
        for location in promotion {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source = self.enpassant.map_or(BitBoard::EMPTY, |sq| {
            enpassant_source::squares(self.active, sq)
        });
        let promotion_rank = self.active.pawn_last_rank();
        let pawn_locs = self.piece_locations(Piece::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_king_attackers(&self) -> Vec<(Piece, Square)> {
        let (whites, blacks) = self.whites_blacks();
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

    fn compute_castle_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let king_constraint = constraints.get(self.king_location(self.active));
        let (whites, blacks) = self.whites_blacks();
        let p1 = |z: CastleZone| king_constraint.subsumes(z.uncontrolled_requirement());
        let p2 = |z: CastleZone| !(whites | blacks).intersects(z.unoccupied_requirement());
        self.castling
            .rights()
            .iter()
            .filter(|&z| p1(z) && p2(z))
            .map(Move::Castle)
            .collect()
    }

    fn legal_pawn_moves<F>(&self, compute_moves: F, constraints: &MoveConstraints) -> Vec<Move>
    where
        F: Fn(Piece, Square) -> BitBoard,
    {
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let active_pawn = Piece::pawn(self.active);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(active_pawn, location) & constraints.get(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }
        for location in enpassant {
            if constraints.get(location).contains(self.enpassant.unwrap()) {
                dest.push(Move::Enpassant(location));
            }
        }
        for location in promotion {
            let targets = compute_moves(active_pawn, location) & constraints.get(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
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
