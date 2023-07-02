use std::fmt::Debug;

use myopic_core::*;

use crate::cache::rays::RaySet;
use crate::enum_map::EnumMap;
use crate::Board;

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

    pub fn intersect(&mut self, location: Square, constraint: BitBoard) {
        let curr = self.data[location];
        self.data[location] = curr & constraint;
    }

    pub fn intersect_pins(&mut self, pinned: &RaySet) {
        for loc in pinned.points {
            self.intersect(loc, pinned.constraints[loc])
        }
    }

    pub fn set(&mut self, location: Square, constraint: BitBoard) {
        self.data[location] = constraint;
    }
}

impl Board {
    pub fn compute_check_constraints(
        &self,
        passive_control: BitBoard,
        pinned: &RaySet
    ) -> MoveConstraints {
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
        [Class::P, Class::N, Class::B, Class::R, Class::Q]
            .into_iter()
            .map(|class| Piece(self.active.reflect(), class))
            .flat_map(|p| self.pieces.locs(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites | blacks).contains(king_loc))
            .collect()
    }
}
