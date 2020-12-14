use crate::implementation::cache::MoveConstraints;
use crate::implementation::MutBoardImpl;
use crate::Move;
use crate::MoveComputeType;
use crate::MutBoard;
use myopic_core::*;

#[cfg(test)]
mod test;

mod enpassant_source;

impl MutBoardImpl {
    pub fn compute_moves_impl(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        let constraints = self.constraints_impl(computation_type);
        let pawn_moves = self.compute_pawn_moves(&constraints);
        let nbrqk_moves = self.compute_nbrqk_moves(&constraints);
        let castle_moves = match computation_type {
            MoveComputeType::All => self.compute_castle_moves(&constraints),
            _ => Vec::with_capacity(0),
        };
        pawn_moves
            .into_iter()
            .chain(nbrqk_moves.into_iter())
            .chain(castle_moves.into_iter())
            .collect()
    }

    fn compute_nbrqk_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        let (whites, blacks) = self.sides();
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // Add standard moves for pieces which aren't pawns or king
        for piece in Piece::on_side(self.active).skip(1) {
            for location in self.pieces.locs_impl(piece) {
                let moves = unchecked_moves(piece, location) & constraints.get(location);
                dest.extend(Move::standards(piece, location, moves));
            }
        }
        dest
    }

    fn compute_pawn_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let (active_pawn, (whites, blacks)) = (Piece::pawn(self.active), self.sides());
        let compute_moves = |loc: Square| active_pawn.moves(loc, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }
        for loc in enpassant {
            let ep = self.enpassant.unwrap();
            if constraints.get(loc).contains(ep) && self.enpassant_doesnt_discover_attack(loc) {
                dest.push(Move::Enpassant(loc, ep));
            }
        }
        for location in promotion {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
    }

    fn enpassant_doesnt_discover_attack(&self, enpassant_source: Square) -> bool {
        let (active, passive) = (self.active, self.active.reflect());
        let active_king = self.king(active);
        let third_rank = passive.pawn_third_rank();
        if !third_rank.contains(active_king) {
            return true;
        }
        let (r, q) = (Piece::rook(passive), Piece::queen(passive));
        let potential_attackers = self.multi_locs(&[r, q]) & third_rank;
        let all_pieces = self.all_pieces();
        for loc in potential_attackers {
            let cord = BitBoard::cord(loc, active_king) & all_pieces;
            if cord.size() == 4
                && cord.contains(enpassant_source)
                && cord.intersects(self.locs(Piece::pawn(passive)))
            {
                return false;
            }
        }
        return true;
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source = self.enpassant.map_or(BitBoard::EMPTY, |sq| {
            enpassant_source::squares(self.active, sq)
        });
        let promotion_rank = self.active.pawn_promoting_src_rank();
        let pawn_locs = self.locs(Piece::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_castle_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let king_constraint = constraints.get(self.king(self.active));
        let (whites, blacks) = self.sides();
        let p1 = |z: CastleZone| king_constraint.subsumes(z.uncontrolled_requirement());
        let p2 = |z: CastleZone| !(whites | blacks).intersects(z.unoccupied_requirement());
        self.castling
            .rights()
            .iter()
            .filter(|&z| p1(z) && p2(z))
            .map(Move::Castle)
            .collect()
    }
}
