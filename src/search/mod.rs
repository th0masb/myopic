use std::cmp;

use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::EvalBoard;

mod quiescent;

pub fn best_move<B: EvalBoard>(state: &mut B, depth: usize) -> Option<Move> {
    assert!(depth > 0);
    let mut best_move = None;
    let (mut alpha, beta) = (-eval::INFTY, eval::INFTY);
    for evolve in state.compute_moves(MoveComputeType::All) {
        let discards = state.evolve(&evolve);
        let result = -negamax(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        if result > alpha {
            alpha = result;
            best_move = Some(evolve.clone());
        }
    }
    best_move
}

fn negamax<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 || state.termination_status().is_some() {
        return match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            None => quiescent::search(state, -eval::INFTY, eval::INFTY, -1),
        };
    }
    let mut result = -eval::INFTY;
    for evolve in state.compute_moves(MoveComputeType::All) {
        let discards = state.evolve(&evolve);
        let next_result = -negamax(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return beta;
        }
    }
    return result;
}
