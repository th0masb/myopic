use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::moves::constraints::MoveConstraints;
use crate::board::implementation::BoardImpl;
use crate::board::Board;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::pieces::Piece;

#[cfg(test)]
mod test;

mod constraints;
mod control;
mod enpassant_source;
mod pinning;

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
    pub(in crate::board::implementation) fn compute_moves_impl(
        &self,
        computation_type: MoveComputeType,
    ) -> Vec<Move> {
        let constraints = self.constraints(computation_type);
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
            for location in self.pieces.locations(piece) {
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
        let pawn_locs = self.locs(Piece::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_king_attackers(&self) -> Vec<(Piece, Square)> {
        let (whites, blacks) = self.sides();
        let king_loc = self.king(self.active);
        pnbrq(self.active.reflect())
            .iter()
            .flat_map(|&p| self.pieces.locations(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites, blacks).contains(king_loc))
            .collect()
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
