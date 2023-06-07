use itertools::Itertools;
use myopic_core::*;

use crate::Board;
use crate::MoveComputeType;
use crate::TerminalState;

const HALF_MOVE_CLOCK_LIMIT: usize = 100;

impl Board {
    pub(crate) fn terminal_state_impl(&self) -> Option<TerminalState> {
        let cache = self.cache.borrow();
        let terminal_status = cache.termination_status;
        drop(cache);
        match terminal_status {
            Some(x) => x,
            None => {
                let result = self.compute_terminal_state();
                self.cache.borrow_mut().termination_status = Some(result);
                result
            }
        }
    }

    fn compute_terminal_state(&self) -> Option<TerminalState> {
        let active = self.active;
        let active_king = self.king(active);
        let passive_control = self.passive_control();
        let (whites, blacks) = self.sides();
        let king_moves = Piece(active, Class::K).moves(active_king, whites, blacks);
        // If king can move somewhere which is usually the case then not terminal.
        if (king_moves - passive_control).is_populated() {
            None
        } else if passive_control.contains(active_king) {
            self.checked_termination()
        } else {
            self.unchecked_termination()
        }
        .or(self.check_clock_limit())
        .or(self.check_repetitions())
    }

    fn check_clock_limit(&self) -> Option<TerminalState> {
        if self.half_move_clock() >= HALF_MOVE_CLOCK_LIMIT {
            Some(TerminalState::Draw)
        } else {
            None
        }
    }

    fn check_repetitions(&self) -> Option<TerminalState> {
        let mut position_hashes = self.history.historical_positions().collect_vec();
        position_hashes.push(self.hash());
        position_hashes.sort_unstable();
        let (mut last, mut count) = (position_hashes[0], 1usize);
        for hash in position_hashes.into_iter().skip(1) {
            if hash == last {
                count += 1;
                if count == 3 {
                    break;
                }
            } else {
                count = 1;
                last = hash;
            }
        }
        if count == 3 {
            Some(TerminalState::Draw)
        } else {
            None
        }
    }

    // Assumes king is in check and cannot move out of it
    fn checked_termination(&self) -> Option<TerminalState> {
        if self.compute_moves(MoveComputeType::All).len() > 0 {
            None
        } else {
            // Checkmate
            Some(TerminalState::Loss)
        }
    }

    // Assumes king cannot move but not in check
    fn unchecked_termination(&self) -> Option<TerminalState> {
        let king = self.king(self.active);
        let pin_rays = Piece(Side::W, Class::Q).empty_control(king);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // These pieces have no constraints since not in check and not on pin rays
        for &class in qrbnp() {
            let piece = Piece(self.active, class);
            let locations = self.locs(&[piece]) - pin_rays;
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        if self.compute_moves(MoveComputeType::All).len() > 0 {
            None
        } else {
            // Stalemate
            Some(TerminalState::Draw)
        }
    }
}

fn qrbnp<'a>() -> &'a [Class] {
    &[Class::Q, Class::R, Class::B, Class::N, Class::P]
}
