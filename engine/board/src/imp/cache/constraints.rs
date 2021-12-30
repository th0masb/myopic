use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

use myopic_core::*;

use crate::ChessBoard;
use crate::imp::Board;
use crate::imp::cache::rays::RaySet;
use crate::MoveComputeType;

#[derive(Clone)]
pub struct MoveConstraints {
    data: [BitBoard; 64],
}

impl PartialEq<MoveConstraints> for MoveConstraints {
    fn eq(&self, other: &MoveConstraints) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(l, r)| *l == *r)
    }
}

impl Debug for MoveConstraints {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.data.to_vec().fmt(f)
    }
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

    fn intersect_pins(&mut self, pinned: &RaySet) {
        for loc in pinned.ray_points {
            self.intersect(loc, pinned.ray(loc).unwrap_or(BitBoard::ALL))
        }
    }

    fn set(&mut self, location: Square, constraint: BitBoard) {
        self.data[location as usize] = constraint;
    }
}

impl Board {
    pub fn constraints_impl(&mut self, computation_type: MoveComputeType) -> MoveConstraints {
        match computation_type {
            MoveComputeType::Attacks => self.compute_constraints(computation_type),
            MoveComputeType::AttacksChecks => self.compute_constraints(computation_type),
            MoveComputeType::All => match &self.cache.move_constraints {
                Some(x) => x.clone(),
                None => {
                    let result = self.compute_constraints(computation_type);
                    self.cache.move_constraints = Some(result.clone());
                    result
                }
            },
        }
    }

    fn compute_constraints(&mut self, computation_type: MoveComputeType) -> MoveConstraints {
        let passive_control = self.passive_control_impl();
        let pinned = self.pinned_set_impl();
        if passive_control.contains(self.king(self.active)) {
            self.check(passive_control, &pinned)
        } else {
            match computation_type {
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
        let enpassant_set = self.enpassant.map_or(BitBoard::EMPTY, |sq| sq.lift());
        let passive_locs = self.side(passive);
        if !checks {
            for piece in Piece::of(active) {
                let enpassant = if piece.is_pawn() {
                    enpassant_set
                } else {
                    BitBoard::EMPTY
                };
                for loc in self.locs(&[piece]) {
                    constraints.intersect(loc, passive_locs | enpassant);
                }
            }
        } else {
            let discoveries = self.compute_discoveries();
            let passive_king = self.king(passive);
            let promotion_rays = Piece::WQ.control(passive_king, whites, blacks);
            let promotion_jumps = Piece::WN.empty_control(passive_king);
            let promotion_checks =
                (promotion_rays | promotion_jumps) & active.pawn_promoting_dest_rank();
            for piece in Piece::of(active) {
                let is_pawn = piece.is_pawn();
                let enpassant = if is_pawn {
                    enpassant_set
                } else {
                    BitBoard::EMPTY
                };
                let check_squares = piece.reflect().control(passive_king, whites, blacks);
                let promotion = if is_pawn {
                    promotion_checks
                } else {
                    BitBoard::EMPTY
                };
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
            .flat_map(|&p| self.pieces.locs(p).into_iter().map(move |s| (p, s)))
            .filter(|(p, s)| p.control(*s, whites, blacks).contains(king_loc))
            .collect()
    }
}

fn pnbrq<'a>(side: Side) -> &'a [Piece; 5] {
    match side {
        Side::White => &[Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ],
        Side::Black => &[Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ],
    }
}
