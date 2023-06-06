use std::fmt::Debug;

use myopic_core::*;

use crate::enum_map::EnumMap;
use crate::private::cache::rays::RaySet;
use crate::private::Board;
use crate::MoveComputeType;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveConstraints {
    data: EnumMap<Square, BitBoard>,
}

impl MoveConstraints {
    pub fn get(&self, location: Square) -> BitBoard {
        self.data[location]
    }

    pub fn all_universal() -> MoveConstraints {
        MoveConstraints::all(BitBoard::ALL)
    }

    pub fn all_empty() -> MoveConstraints {
        MoveConstraints::all(BitBoard::EMPTY)
    }

    pub fn all(bitboard: BitBoard) -> MoveConstraints {
        MoveConstraints { data: EnumMap::from_array([bitboard; 64]) }
    }

    fn intersect(&mut self, location: Square, constraint: BitBoard) {
        let curr = self.data[location];
        self.data[location] = curr & constraint;
    }

    fn intersect_pins(&mut self, pinned: &RaySet) {
        for loc in pinned.points() {
            self.intersect(loc, pinned.ray(loc).unwrap_or(BitBoard::ALL))
        }
    }

    fn set(&mut self, location: Square, constraint: BitBoard) {
        self.data[location] = constraint;
    }
}

impl Board {
    pub fn move_constraints(&self, compute_type: MoveComputeType) -> MoveConstraints {
        let cache = self.cache.borrow();
        let cached_moves = cache.move_constraints[compute_type].clone();
        drop(cache);
        cached_moves.unwrap_or_else(|| {
            let computed = self.compute_move_constraints(compute_type);
            self.cache.borrow_mut().move_constraints[compute_type] = Some(computed.clone());
            computed
        })
    }

    fn compute_move_constraints(&self, compute_type: MoveComputeType) -> MoveConstraints {
        let passive_control = self.passive_control();
        let pinned = self.pinned_set();
        if passive_control.contains(self.king(self.active)) {
            self.check(passive_control, &pinned)
        } else {
            match compute_type {
                MoveComputeType::All => self.any(passive_control, &pinned),
                MoveComputeType::Attacks => self.attacks(passive_control, &pinned, false),
                MoveComputeType::AttacksChecks => self.attacks(passive_control, &pinned, true),
            }
        }
    }

    /// Assuming the king is not in check
    fn any(&self, passive_control: BitBoard, pinned: &RaySet) -> MoveConstraints {
        let mut constraints = MoveConstraints::all_universal();
        constraints.set(self.king(self.active), !passive_control);
        constraints.intersect_pins(pinned);
        constraints
    }

    /// Assuming the king is not in check
    fn attacks(&self, passive_control: BitBoard, pinned: &RaySet, checks: bool) -> MoveConstraints {
        let (whites, blacks) = self.sides();
        let (active, passive) = (self.active, self.active.reflect());
        let mut constraints = MoveConstraints::all_universal();
        // Add pinned constraints
        constraints.intersect_pins(pinned);
        let enpassant_set = self.enpassant.map_or(BitBoard::EMPTY, |sq| sq.into());
        let passive_locs = self.side(passive);
        if !checks {
            for class in Class::all() {
                let piece = Piece(active, class);
                let enpassant = if piece.1 == Class::P { enpassant_set } else { BitBoard::EMPTY };
                for loc in self.locs(&[piece]) {
                    constraints.intersect(loc, passive_locs | enpassant);
                }
            }
        } else {
            let discoveries = self.compute_discoveries();
            let passive_king = self.king(passive);
            let promotion_rays = Piece(Side::W, Class::Q).control(passive_king, whites | blacks);
            let promotion_jumps = Piece(Side::W, Class::N).empty_control(passive_king);
            let promotion_checks =
                (promotion_rays | promotion_jumps) & active.pawn_promoting_dest_rank();
            for class in Class::all() {
                let piece = Piece(active, class);
                let is_pawn = piece.1 == Class::P;
                let enpassant = if is_pawn { enpassant_set } else { BitBoard::EMPTY };
                let check_squares = piece.reflect().control(passive_king, whites | blacks);
                let promotion = if is_pawn { promotion_checks } else { BitBoard::EMPTY };
                for loc in self.locs(&[piece]) {
                    let discov = discoveries.ray(loc).map(|r| !r).unwrap_or(BitBoard::EMPTY);
                    constraints.intersect(
                        loc,
                        passive_locs | check_squares | enpassant | discov | promotion,
                    );
                }
            }
        }
        // King can't move into check
        constraints.intersect(self.king(active), !passive_control);
        constraints
    }

    fn check(&self, passive_control: BitBoard, pinned: &RaySet) -> MoveConstraints {
        let active_king_loc = self.king(self.active);
        let attackers = self.compute_king_attackers();
        if attackers.len() == 1 {
            // If one attacker then all pieces can only move to block the attack
            // except the king who can move anywhere out of the passive control
            // zone.
            let (piece, attack_location) = attackers[0];
            let blocking_squares = if piece.1 == Class::N {
                attack_location.into()
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
        pnbrq()
            .into_iter()
            .map(|class| Piece(self.active.reflect(), class))
            .flat_map(|p| self.pieces.locs(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites | blacks).contains(king_loc))
            .collect()
    }
}

fn pnbrq() -> [Class; 5] {
    [Class::P, Class::N, Class::B, Class::R, Class::Q]
}
