use std::cmp;

use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::EvalBoard;

mod quiescent;

pub fn best_move<B: EvalBoard>(state: &mut B, depth: usize) -> Option<(Move, i32)> {
    assert!(depth > 0);
    let mut best_move = None;
    let (mut alpha, beta) = (-eval::INFTY, eval::INFTY);
    for evolve in state.compute_moves(MoveComputeType::All) {
        let discards = state.evolve(&evolve);
        let result = -negamax(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        if result > alpha {
            alpha = result;
            best_move = Some((evolve.clone(), result));
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

/// Tests for 'obvious' positions like taking a hanging piece,
/// checkmating or escaping checkmate etc.
#[cfg(test)]
mod test {
    use crate::base::square::Square;
    use crate::base::square::Square::*;
    use crate::base::Reflectable;
    use crate::board::Move;
    use crate::board::Move::*;
    use crate::eval::EvalBoard;
    use crate::pieces::Piece;

    const DEPTH: usize = 3;

    fn test(fen_string: &'static str, expected_move_pool: Vec<Move>, is_won: bool) {
        let board = crate::eval::new_board(fen_string).unwrap();
        let (ref_board, ref_move_pool) = (board.reflect(), expected_move_pool.reflect());
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn test_impl<B: EvalBoard>(mut board: B, expected_move_pool: Vec<Move>, is_won: bool) {
        let search_result = super::best_move(&mut board, DEPTH);
        if expected_move_pool.is_empty() {
            assert_eq!(None, search_result);
        } else {
            let (best_move, evaluation) = search_result.unwrap().clone();
            if is_won {
                assert_eq!(crate::eval::WIN_VALUE, evaluation);
            }
            assert!(expected_move_pool.contains(&best_move), "{:?}", best_move);
        }
    }

    #[test]
    fn queen_escape_attack() {
        let mv = |target: Square| Standard(Piece::WQ, A4, target);
        test(
            "r4rk1/5ppp/8/1Bn1p3/Q7/8/5PPP/1R3RK1 w Qq - 5 27",
            vec![mv(B4), mv(C4), mv(G4), mv(H4), mv(C2), mv(D1)],
            false,
        )
    }

    #[test]
    fn mate_0() {
        test(
            "r2r2k1/5ppp/1N2p3/1n6/3Q4/2B5/5PPP/1R3RK1 w Qq - 4 21",
            vec![Standard(Piece::WQ, D4, G7)],
            true,
        )
    }

    #[test]
    fn mate_1() {
        test(
            "8/8/8/4Q3/8/6R1/2n1pkBK/8 w - - 0 1",
            vec![Standard(Piece::WR, G3, D3)],
            true,
        )
    }

    #[test]
    fn mate_2() {
        test(
            "8/7B/5Q2/6p1/6k1/8/5K2/8 w - - 0 1",
            vec![Standard(Piece::WQ, F6, H8), Standard(Piece::WQ, F6, F3)],
            true,
        )
    }

    #[test]
    fn mate_3() {
        test(
            "3qr2k/1b1p2pp/7N/3Q2b1/4P3/8/5PP1/6K1 w - - 0 1",
            vec![Standard(Piece::WQ, D5, G8)],
            true,
        )
    }

    // Mate in 4 moves TODO probably better in benchmark.
    #[test]
    fn mate_4() {
        test(
            "r1k2b1r/pp4pp/2p1n3/3NQ1B1/6q1/8/PPP2P1P/2KR4 w - - 4 20",
            vec![Standard(Piece::WQ, E5, C7)],
            true,
        )
    }

    #[test]
    fn tactic_1() {
        test(
            "1r3k2/2R5/1p2p2p/1Q1pPp1q/1P1P2p1/2P1P1P1/6KP/8 b - - 2 31",
            vec![Standard(Piece::BR, B8, A8)],
            false,
        )
    }

    #[test]
    fn tactic_2() {
        test(
            "r5k1/pb4pp/1pn1pq2/5B2/2Pr4/B7/PP3RPP/R4QK1 b - - 0 23",
            vec![Standard(Piece::BP, E6, F5)],
            false,
        )
    }
}
