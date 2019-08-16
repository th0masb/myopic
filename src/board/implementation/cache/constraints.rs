use crate::base::bitboard::BitBoard;
use crate::base::Reflectable;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::implementation::BoardImpl;
use crate::board::implementation::cache::pinning::PinnedSet;
use crate::board::MoveComputeType;
use crate::pieces::Piece;
use crate::base::Side;

pub struct MoveConstraints {
    data: [BitBoard; 64],
}

impl MoveConstraints {
    pub fn get(&self, location: Square) -> BitBoard {
        self.data[location as usize]
    }

    pub fn all_universal() -> MoveConstraints {
        MoveConstraints {
            data: [BitBoard::ALL; 64],
        }
    }

    pub fn all_empty() -> MoveConstraints {
        MoveConstraints {
            data: [BitBoard::EMPTY; 64],
        }
    }

    pub fn all(bitboard: BitBoard) -> MoveConstraints {
        MoveConstraints {
            data: [bitboard; 64],
        }
    }

    fn intersect(&mut self, location: Square, constraint: BitBoard) {
        let curr = self.data[location as usize];
        self.data[location as usize] = curr & constraint;
    }

    fn intersect_pins(&mut self, pinned: &PinnedSet) {
        for loc in pinned.pinned_locations {
            self.intersect(loc, pinned.constraint(loc))
        }
    }

    fn set(&mut self, location: Square, constraint: BitBoard) {
        self.data[location as usize] = constraint;
    }
}

impl BoardImpl {
    pub fn constraints(&mut self, computation_type: MoveComputeType) -> MoveConstraints {
        let passive_control = self.passive_control_impl();
        let pinned = self.pinned_set_impl();
        if passive_control.contains(self.king(self.active)) {
            self.in_check(passive_control, &pinned)
        } else {
            match computation_type {
                MoveComputeType::All => self.any(passive_control, &pinned),
                MoveComputeType::Attacks => self.attacks(passive_control, &pinned, false),
                MoveComputeType::AttacksChecks => self.attacks(passive_control, &pinned, true),
            }
        }
    }

    /// Assuming the king is not in check
    fn any(&self, passive_control: BitBoard, pinned: &PinnedSet) -> MoveConstraints {
        let mut constraints = MoveConstraints::all_universal();
        constraints.set(self.king(self.active), !passive_control);
        constraints.intersect_pins(pinned);
        constraints
    }

    /// Assuming the king is not in check
    fn attacks(
        &self,
        passive_control: BitBoard,
        pinned: &PinnedSet,
        checks: bool,
    ) -> MoveConstraints {
        let (whites, blacks) = self.sides();
        let (active, passive) = (self.active, self.active.reflect());
        let mut constraints = MoveConstraints::all_universal();
        let passive_locs = self.pieces.side_locations(passive);
        let passive_king_loc = self.king(passive);
        // King constraint
        constraints.set(self.king(active), passive_locs - passive_control);
        // Add pinned constraints
        constraints.intersect_pins(pinned);
        // Add attack constraints
        let enpassant_set = self.enpassant.map_or(BitBoard::EMPTY, |sq| sq.lift());
        for piece in Piece::on_side(active) {
            // We reflect the piece here to correctly account for pawns.
            let enpassant = if piece.is_pawn() {
                enpassant_set
            } else {
                BitBoard::EMPTY
            };
            let check_squares = piece.reflect().control(passive_king_loc, whites, blacks);
            for loc in self.locs(piece) {
                if checks {
                    constraints.intersect(loc, passive_locs | check_squares | enpassant);
                } else {
                    constraints.intersect(loc, passive_locs | enpassant);
                }
            }
        }
        constraints
    }

    fn in_check(&self, passive_control: BitBoard, pinned: &PinnedSet) -> MoveConstraints {
        let active_king_loc = self.king(self.active);
        let attackers = self.compute_king_attackers();
        if attackers.len() == 1 {
            // If one attacker then all pieces can only move to block the attack
            // except the king who can move anywhere out of the passive control
            // zone.
            let (piece, attack_location) = attackers[0];
            let blocking_squares = if piece.is_knight() {
                attack_location.lift()
            } else {
                BitBoard::cord(attack_location, active_king_loc)
            };
            let mut constraint = MoveConstraints::all(blocking_squares);
            constraint.intersect_pins(pinned);
            constraint.set(active_king_loc, !passive_control);
            constraint
        } else {
            let mut constraint = MoveConstraints::all_empty();
            constraint.set(active_king_loc, !passive_control);
            constraint
        }
    }

    fn compute_king_attackers(&self) -> Vec<(Piece, Square)> {
        let (whites, blacks) = self.sides();
        let king_loc = self.king(self.active);
        pnbrq(self.active.reflect())
            .iter()
            .flat_map(|&p| self.pieces.locs_impl(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites, blacks).contains(king_loc))
            .collect()
    }

}

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
