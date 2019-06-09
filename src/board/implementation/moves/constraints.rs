use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::board::implementation::BoardImpl;
use crate::board::MoveComputationType;
use crate::base::Reflectable;
use crate::pieces::Piece;
use crate::board::implementation::moves::pnbrq;
use crate::board::Board;
use crate::board::implementation::moves::pinning::PinnedSet;
use crate::board::Move;

pub struct MoveConstraints {
    data: [BitBoard; 64],
}

impl MoveConstraints {
    pub fn constraint(&self, location: Square) -> BitBoard {
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
            data: [bitboard; 64]
        }
    }

    fn intersect(&mut self, location: Square, constraint: BitBoard) {
        let curr = self.data[location as usize];
        self.data[location as usize] = curr & constraint;
    }

    fn set(&mut self, location: Square, constraint: BitBoard) {
        self.data[location as usize] = constraint;
    }
}



impl BoardImpl {
    pub fn constraints(&self, computation_type: MoveComputationType) -> MoveConstraints {
        let passive_control = self.compute_control(self.active.reflect());
        let pinned = self.compute_pinned();

        if passive_control.contains(self.king_location(self.active)) {
            self.in_check(passive_control, &pinned)
        } else {
            match computation_type {
                MoveComputationType::All => self.any(passive_control, &pinned),
                MoveComputationType::Attacks => self.attacks(passive_control, &pinned, false),
                MoveComputationType::AttacksChecks => self.attacks(passive_control, &pinned, true),
            }
        }
    }

    /// Assuming the king is not in check
    fn any(&self, passive_control: BitBoard, pinned: &PinnedSet) -> MoveConstraints {
        let mut constraints = MoveConstraints::all_universal();
        constraints.set(self.king_location(self.active), !passive_control);
        for (loc, constraint) in pinned.constraint_areas.iter() {
            constraints.set(*loc, *constraint);
        }
        constraints
    }

    /// Assuming the king is not in check
    fn attacks(&self, passive_control: BitBoard, pinned: &PinnedSet, checks: bool) -> MoveConstraints {
        let (whites, blacks) = self.whites_blacks();
        let (active, passive) = (self.active, self.active.reflect());
        let mut constraints = MoveConstraints::all_universal();
        let active_king_loc = self.king_location(active);
        let passive_locs = self.pieces.side_locations(passive);
        let passive_king_loc = self.king_location(passive);
        // King constraint
        constraints.set(self.king_location(active), passive_locs - passive_control);
        // Add pinned constraints
        for (loc, constraint) in pinned.constraint_areas.iter() {
            constraints.intersect(*loc, *constraint);
        }
        // Add attack constraints
        for piece in Piece::on_side(active) {
            // We reflect the piece here to correctly account for pawns.
            let check_squares = piece.reflect().control(passive_king_loc, whites, blacks);
            for loc in self.piece_locations(piece) {
                if checks {
                    constraints.intersect(loc, passive_locs | check_squares);
                } else {
                    constraints.intersect(loc, passive_locs);
                }
            }
        }
        constraints
    }

    fn in_check(&self, passive_control: BitBoard, pinned: &PinnedSet) -> MoveConstraints {
        let (active, passive) = (self.active, self.active.reflect());
        let (whites, blacks) = self.whites_blacks();
        let active_king_loc = self.pieces.king_location(active);
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
            constraint.set(active_king_loc, !passive_control);
            constraint
        } else {
            let mut constraint = MoveConstraints::all_empty();
            constraint.set(active_king_loc, !passive_control);
            constraint
        }
    }
}