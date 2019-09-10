use std::cmp;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::EvalBoard;

#[cfg(test)]
mod mate_benchmark;
pub mod quiescent;

pub struct SearchTermination {
    max_depth: usize,
    max_time: Duration,
    stop_signal: Option<Receiver<bool>>,
}

pub struct Search<B: EvalBoard> {
    root: B,
    termination: SearchTermination,
}

impl<B: EvalBoard> Search<B> {
    pub fn new(root: B, termination: SearchTermination) -> Search<B> {
        Search { root, termination }
    }
    pub fn time_capped(root: B, max_time: Duration) -> Search<B> {
        Search {
            root,
            termination: SearchTermination { max_time, max_depth: 1000, stop_signal: None },
        }
    }
    pub fn depth_capped(root: B, max_depth: usize) -> Search<B> {
        Search {
            root,
            termination: SearchTermination {
                max_depth,
                max_time: Duration::from_secs(100_000_000_000),
                stop_signal: None,
            },
        }
    }

    pub fn execute(&self) -> Result<(Move, i32), ()> {
        let tracker = SearchTerminationImpl {
            search_start: Instant::now(),
            max_time: self.termination.max_time,
            stop_signal: self.termination.stop_signal.as_ref(),
        };
        let searcher = TerminatingSearchImpl {
            root: self.root.clone(),
            termination_tracker: tracker,
            max_depth: self.termination.max_depth,
        };
        searcher.search()
    }
}

struct SearchTerminationImpl<'a> {
    search_start: Instant,
    max_time: Duration,
    stop_signal: Option<&'a Receiver<bool>>,
}

struct TerminatingSearchImpl<'a, B: EvalBoard> {
    root: B,
    termination_tracker: SearchTerminationImpl<'a>,
    max_depth: usize,
}

impl SearchTerminationImpl<'_> {
    pub fn should_stop_search(&self) -> bool {
        self.search_start.elapsed() > self.max_time
            || self.stop_signal.map_or(false, |rec| rec.try_recv().is_ok())
    }
}

impl<B: EvalBoard> TerminatingSearchImpl<'_, B> {
    pub fn search(&self) -> Result<(Move, i32), ()> {
        let mut best_move = Err(());
        for i in 1..self.max_depth + 1 {
            match self.best_move(i) {
                Err(_) => break,
                Ok(next_depth) => best_move = Ok(next_depth),
            }
        }
        best_move
    }

    pub fn best_move(&self, depth: usize) -> Result<(Move, i32), ()> {
        assert!(depth > 0);
        let mut state = self.root.clone();
        let mut best_move = None;
        let (mut alpha, beta) = (-eval::INFTY, eval::INFTY);
        for evolve in state.compute_moves(MoveComputeType::All) {
            let discards = state.evolve(&evolve);
            match self.negamax(&mut state, -beta, -alpha, depth - 1) {
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

    pub fn negamax(&self, root: &mut B, mut a: i32, b: i32, depth: usize) -> Result<i32, ()> {
        if self.termination_tracker.should_stop_search() {
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
            let next_result = -self.negamax(root, -b, -a, depth - 1)?;
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
    use crate::base::square::Square;
    use crate::base::square::Square::*;
    use crate::base::Reflectable;
    use crate::board::Move;
    use crate::board::Move::*;
    use crate::eval::EvalBoard;
    use crate::pieces::Piece;
    use crate::search::Search;

    const DEPTH: usize = 3;

    fn test(fen_string: &'static str, expected_move_pool: Vec<Move>, is_won: bool) {
        let board = crate::eval::new_board(fen_string).unwrap();
        let (ref_board, ref_move_pool) = (board.reflect(), expected_move_pool.reflect());
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn test_impl<B: EvalBoard>(board: B, expected_move_pool: Vec<Move>, is_won: bool) {
        let search_result = Search::depth_capped(board, DEPTH).execute();
        if expected_move_pool.is_empty() {
            assert_eq!(Err(()), search_result);
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
