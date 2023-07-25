use crate::board::{control, iter};
use crate::constants::{class, create_piece, lift, piece, reflect_side, side};
use crate::moves::Move;
use crate::node::{EvalFacet, Evaluation};
use crate::position::Position;
use crate::{union_boards, Board, Side, SquareMap};
use std::cmp::{max, min};

pub struct SafetyFacet {
    control_bonus: u32,
    piece_count_multipliers: [f64; 3],
    endgame_multiplier: f64,
}

impl Default for SafetyFacet {
    fn default() -> Self {
        SafetyFacet {
            control_bonus: 10,
            endgame_multiplier: 0.2,
            piece_count_multipliers: [1.0, 1.5, 3.0],
        }
    }
}

impl SafetyFacet {
    fn compute_king_safety(&self, pos: &Position, side: Side) -> i32 {
        let king = create_piece(side, class::K);
        let king_loc = pos.piece_boards[king].trailing_zeros() as usize;
        // If the king is off the board just skip the computation
        if king_loc == 64 {
            return 0;
        }
        let occupied = union_boards(&pos.side_boards) & !lift(king_loc);
        let safety_ring = control(king, king_loc, 0) & !occupied;
        let other_side = reflect_side(side);
        let mut total_control_count = 0;
        let mut attacker_count = 1usize;
        for class in [class::N, class::B, class::R, class::Q] {
            let p = create_piece(other_side, class);
            iter(pos.piece_boards[p]).for_each(|sq| {
                let control_count = (control(p, sq, occupied) & safety_ring).count_ones();
                total_control_count += control_count;
                attacker_count += min(1, control_count as usize);
            });
        }
        let mul = self.piece_count_multipliers;
        let mul_index = min(mul.len() - 1, attacker_count - 1);
        ((total_control_count * self.control_bonus) as f64 * mul[mul_index]).round() as i32
    }
}

impl EvalFacet for SafetyFacet {
    fn static_eval(&self, board: &Position) -> Evaluation {
        let mid_eval =
            self.compute_king_safety(board, side::W) - self.compute_king_safety(board, side::B);
        Evaluation::Phased {
            mid: mid_eval,
            end: (mid_eval as f64 * self.endgame_multiplier).round() as i32,
        }
    }

    fn make(&mut self, mv: &Move, board: &Position) {}

    fn unmake(&mut self, mv: &Move) {}
}
