use std::cmp;
use std::time::{Duration, Instant};

use crate::eval::EvalBoard;
use crate::{eval, quiescent};
use myopic_board::{Move, MoveComputeType, Termination};

pub mod interactive;

const DEPTH_UPPER_BOUND: usize = 10;

/// Data class composing information/result about/of a best move search.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOutcome {
    pub best_move: Move,
    pub eval: i32,
    pub depth: usize,
    pub time: Duration,
}

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<B: EvalBoard, T: SearchTerminator>(
    root: B,
    terminator: T,
) -> Result<SearchOutcome, String> {
    Search { root, terminator }.search()
}

/// Represents some object which can determine whether a search should be
/// terminated given certain context about the current state. Implementations
/// are provided for Duration (caps the search based on time elapsed), for
/// usize which represents a maximum search depth and for a pair
/// (Duration, usize) which combines both checks.
pub trait SearchTerminator {
    fn should_terminate(&self, search_start: Instant, curr_depth: usize) -> bool;
}

impl SearchTerminator for Duration {
    fn should_terminate(&self, search_start: Instant, _curr_depth: usize) -> bool {
        search_start.elapsed() > *self
    }
}

impl SearchTerminator for usize {
    fn should_terminate(&self, _search_start: Instant, curr_depth: usize) -> bool {
        curr_depth >= *self
    }
}

impl SearchTerminator for (Duration, usize) {
    fn should_terminate(&self, search_start: Instant, curr_depth: usize) -> bool {
        self.0.should_terminate(search_start, curr_depth)
            || self.1.should_terminate(search_start, curr_depth)
    }
}

struct Search<B: EvalBoard, T: SearchTerminator> {
    root: B,
    terminator: T,
}

impl<B: EvalBoard, T: SearchTerminator> Search<B, T> {
    pub fn search(&self) -> Result<SearchOutcome, String> {
        let search_start = Instant::now();
        let mut best_move = Err(String::from("Terminated before search began"));
        for i in 1..DEPTH_UPPER_BOUND {
            match self.best_move(i, search_start) {
                Err(_) => break,
                Ok((mv, eval)) => {
                    best_move = Ok((mv, eval, i));
                }
            }
        }
        best_move.map(|(best_move, eval, depth)| SearchOutcome {
            best_move,
            eval,
            depth,
            time: search_start.elapsed(),
        })
    }

    fn best_move(&self, depth: usize, search_start: Instant) -> Result<(Move, i32), ()> {
        if depth < 1 {
            return Err(());
        }
        let mut state = self.root.clone();
        let mut best_move = None;
        let (mut alpha, beta) = (-eval::INFTY, eval::INFTY);
        for evolve in state.compute_moves(MoveComputeType::All) {
            let discards = state.evolve(&evolve);
            match self.negamax(&mut state, -beta, -alpha, depth - 1, search_start) {
                Err(_) => return Err(()),
                Ok(result) => {
                    let negated_result = -result;
                    state.devolve(&evolve, discards);
                    if negated_result > alpha {
                        alpha = negated_result;
                        best_move = Some((evolve.clone(), negated_result));
                    }
                }
            }
        }
        best_move.ok_or(())
    }

    fn negamax(
        &self,
        root: &mut B,
        mut a: i32,
        b: i32,
        depth: usize,
        start_time: Instant,
    ) -> Result<i32, ()> {
        if self.terminator.should_terminate(start_time, depth) {
            return Err(());
        } else if depth == 0 || root.termination_status().is_some() {
            return Ok(match root.termination_status() {
                Some(Termination::Loss) => eval::LOSS_VALUE,
                Some(Termination::Draw) => eval::DRAW_VALUE,
                None => quiescent::search(root, -eval::INFTY, eval::INFTY, -1),
            });
        }
        let mut result = -eval::INFTY;
        for evolve in root.compute_moves(MoveComputeType::All) {
            //println!("Second {:?}", evolve);
            let discards = root.evolve(&evolve);
            let next_result = -self.negamax(root, -b, -a, depth - 1, start_time)?;
            root.devolve(&evolve, discards);
            result = cmp::max(result, next_result);
            a = cmp::max(a, result);
            if a > b {
                return Ok(b);
            }
        }
        return Ok(result);
    }
}

/// Tests for 'obvious' positions like taking a hanging piece,
/// checkmating or escaping checkmate etc.
#[cfg(test)]
mod test {
    use crate::eval;
    use crate::eval::EvalBoard;
    use myopic_board::Move;
    use myopic_board::Move::Standard;
    use myopic_core::pieces::Piece;
    use myopic_core::reflectable::Reflectable;
    use myopic_core::Square;
    use myopic_core::Square::*;

    const DEPTH: usize = 3;

    fn test(fen_string: &'static str, expected_move_pool: Vec<Move>, is_won: bool) {
        let board = crate::eval::new_board(fen_string).unwrap();
        let (ref_board, ref_move_pool) = (board.reflect(), expected_move_pool.reflect());
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn test_impl<B: EvalBoard>(board: B, expected_move_pool: Vec<Move>, is_won: bool) {
        match super::search(board, DEPTH) {
            Err(message) => panic!("{}", message),
            Ok(outcome) => {
                assert!(expected_move_pool.contains(&outcome.best_move));
                if is_won {
                    assert_eq!(eval::WIN_VALUE, outcome.eval);
                }
            }
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
        test("8/8/8/4Q3/8/6R1/2n1pkBK/8 w - - 0 1", vec![Standard(Piece::WR, G3, D3)], true)
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
    #[ignore]
    #[test]
    fn mate_4() {
        test(
            "r1k2b1r/pp4pp/2p1n3/3NQ1B1/6q1/8/PPP2P1P/2KR4 w - - 4 20",
            vec![Standard(Piece::WQ, E5, C7)],
            true,
        )
    }

    #[test]
    fn mate_5() {
        test(
            "r1b1k1nr/p2p1ppp/n2B4/1p1NPN1P/6P1/3P1Q2/P1P1K3/q5b1 w - - 0 30",
            vec![Standard(Piece::WN, F5, G7)],
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

    /// This fails at depth 3 but passes at depth 4, should be moved to a
    /// benchmark maybe
    #[ignore]
    #[test]
    fn tactic_2() {
        test(
            "r5k1/pb4pp/1pn1pq2/5B2/2Pr4/B7/PP3RPP/R4QK1 b - - 0 23",
            vec![Standard(Piece::BP, E6, F5)],
            false,
        )
    }
}
