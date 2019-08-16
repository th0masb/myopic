use crate::eval::EvalBoard;
use crate::board::MoveComputeType;
use std::cmp;

pub fn negamax<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: usize) -> i32 {
    // Termination state should be precomputed at move time? What if depth is zero and the state is terminal?
    if depth == 0 || state.termination_status().is_some() {
        state.static_eval()
    } else {
        // Not quite right
        for evolve in state.compute_moves(MoveComputeType::All) {
            let discards = state.evolve(&evolve);
            let next_res = negamax(state, -beta, -alpha, depth - 1);
            state.devolve(&evolve, discards);
            alpha = cmp::max(alpha, next_res);
            if alpha > beta {
                return beta
            }
        }
        return alpha;
    }
}