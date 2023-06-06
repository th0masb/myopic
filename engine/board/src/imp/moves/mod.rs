use myopic_core::*;

use crate::imp::cache::MoveConstraints;
use crate::imp::Board;
use crate::mv::Move;
use crate::ChessBoard;
use crate::MoveComputeType;
use crate::Square::*;

#[cfg(test)]
mod test;

mod enpassantsrc;

impl Board {
    pub(crate) fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        let constraints = self.move_constraints(computation_type);
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
        for piece in Piece::of(self.active).skip(1) {
            for location in self.pieces.locs(piece) {
                let moves = unchecked_moves(piece, location) & constraints.get(location);
                dest.extend(self.standards(piece, location, moves));
            }
        }
        dest
    }

    fn standards(&self, moving: Piece, from: Square, dests: BitBoard) -> Vec<Move> {
        let source = self.hash();
        dests
            .iter()
            .map(|dest| Move::Standard {
                source,
                moving,
                from,
                dest,
                capture: self.piece(dest),
            })
            .collect()
    }

    fn promotions(&self, side: Side, from: Square, dests: BitBoard) -> Vec<Move> {
        let source = self.hash();
        dests
            .iter()
            .flat_map(|dest| {
                Board::promotion_targets(side)
                    .iter()
                    .map(move |&promoted| Move::Promotion {
                        source,
                        from,
                        dest,
                        promoted,
                        capture: self.piece(dest),
                    })
            })
            .collect()
    }

    fn promotion_targets<'a>(side: Side) -> &'a [Piece; 4] {
        match side {
            Side::W => &[Piece::WQ, Piece::WR, Piece::WB, Piece::WN],
            Side::B => &[Piece::BQ, Piece::BR, Piece::BB, Piece::BN],
        }
    }

    fn compute_pawn_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let (active_pawn, (whites, blacks)) = (Piece::pawn(self.active), self.sides());
        let compute_moves = |loc: Square| active_pawn.moves(loc, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(location) & constraints.get(location);
            moves.extend(self.standards(active_pawn, location, targets));
        }
        let (source, active) = (self.hash(), self.active);
        for from in enpassant {
            let dest = self.enpassant.unwrap();
            let capture = dest.next(active.reflect().pawn_dir()).unwrap();
            if constraints.get(from).contains(capture)
                && self.enpassant_doesnt_discover_attack(from)
            {
                moves.push(Move::Enpassant {
                    source,
                    side: active,
                    from,
                    dest,
                    capture,
                });
            }
        }
        for location in promotion {
            let targets = compute_moves(location) & constraints.get(location);
            moves.extend(self.promotions(self.active, location, targets));
        }

        moves
    }

    fn enpassant_doesnt_discover_attack(&self, enpassant_source: Square) -> bool {
        let (active, passive) = (self.active, self.active.reflect());
        let active_king = self.king(active);
        let third_rank = passive.pawn_third_rank();
        if !third_rank.contains(active_king) {
            return true;
        }
        let (r, q) = (Piece::rook(passive), Piece::queen(passive));
        let potential_attackers = self.locs(&[r, q]) & third_rank;
        let all_pieces = self.all_pieces();
        for loc in potential_attackers {
            let cord = BitBoard::cord(loc, active_king) & all_pieces;
            if cord.size() == 4
                && cord.contains(enpassant_source)
                && cord.intersects(self.locs(&[Piece::pawn(passive)]))
            {
                return false;
            }
        }
        return true;
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source = self
            .enpassant
            .map_or(BitBoard::EMPTY, |sq| enpassantsrc::squares(self.active, sq));
        let promotion_rank = self.active.pawn_promoting_from_rank();
        let pawn_locs = self.locs(&[Piece::pawn(self.active)]);
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_castle_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let king_constraint = constraints.get(self.king(self.active));
        let (whites, blacks) = self.sides();
        let p1 = |c: Corner| king_constraint.subsumes(uncontrolled_req(c));
        let p2 = |c: Corner| !(whites | blacks).intersects(unoccupied_req(c));
        let source = self.hash();
        self.rights
            .corners()
            .filter(|&c| p1(c) && p2(c))
            .filter(|&c| {
                let Line(k_source, _) = Line::king_castling(c);
                let Line(r_source, _) = Line::rook_castling(c);
                self.piece(k_source) == Some(Piece::king(c.0)) &&
                    self.piece(r_source) == Some(Piece::rook(c.0))
            })
            .map(|corner| Move::Castle { source, corner })
            .collect()
    }
}

fn uncontrolled_req(Corner(side, flank): Corner) -> BitBoard {
    match (side, flank) {
        (Side::W, Flank::K) => E1 | F1 | G1,
        (Side::W, Flank::Q) => E1 | D1 | C1,
        (Side::B, Flank::K) => E8 | F8 | G8,
        (Side::B, Flank::Q) => E8 | D8 | C8,
    }
}

fn unoccupied_req(Corner(side, flank): Corner) -> BitBoard {
    match (side, flank) {
        (Side::W, Flank::K) => F1 | G1,
        (Side::W, Flank::Q) => D1 | C1 | B1,
        (Side::B, Flank::K) => F8 | G8,
        (Side::B, Flank::Q) => D8 | C8 | B8,
    }
}
