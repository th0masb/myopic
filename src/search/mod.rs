use std::cmp;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use crate::board::{Move, BoardImpl, Board};
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::{EvalBoard, SimpleEvalBoard};

#[cfg(test)]
mod mate_benchmark;
pub mod quiescent;

// TODO need to restructure this, there should be a search thread with
// a state and receivers for setup/stopping and a sender for the best
// move computation.
type SearchResult = Result<SearchDetails, ()>;
type DefaultBoard = SimpleEvalBoard<BoardImpl>;

pub fn init<B: EvalBoard>(root: B) -> (Sender<SearchInput<B>>, Receiver<SearchResult>) {
    let (input_tx, input_rx) = mpsc::channel::<SearchInput<B>>();
    let (output_tx, output_rx) = mpsc::channel::<SearchResult>();
    std::thread::spawn(move || {
        let mut searcher = Search2::new(root, input_rx, output_tx);
        loop {
            match &searcher.input_rx.recv() {
                Err(_) => continue,
                Ok(input) => match input.to_owned() {
                    SearchInput::Close => break,
                    SearchInput::Stop => (),
                    SearchInput::Go => {
                        match &searcher.output_tx.send(searcher.execute()) {
                            _ => (),
                        }
                    },
                    SearchInput::Setup {root, max_depth, max_time} => {
                        searcher.root = root;
                        searcher.max_depth = max_depth;
                        searcher.max_time = max_time;
                    },
                }
            }
        }
    });
    (input_tx, output_rx)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchInput<B: EvalBoard> {
    Go,
    Stop,
    Close,
    Setup {
        root: B,
        max_depth: usize,
        max_time: Duration,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchDetails {
    best_move: Move,
    eval: i32,
    depth: usize,
    time: Duration,
    ponder: Option<Move>,
}

type InChannel<B: EvalBoard> = Receiver<SearchInput<B>>;
type OutChannel = Sender<SearchResult>;

pub struct Search2<B: EvalBoard> {
    input_rx: InChannel<B>,
    output_tx: OutChannel,
    root: B,
    max_depth: usize,
    max_time: Duration,
}

impl<B: EvalBoard> Search2<B> {
    pub fn new(root: B, input_rx: InChannel<B>, output_tx: OutChannel) -> Search2<B> {
        Search2 {
            input_rx,
            output_tx,
            root,
            max_depth: 10,
            max_time: Duration::from_secs(10000),
        }
    }

    pub fn execute(&self) -> Result<SearchDetails, ()> {
        let search_start = Instant::now();
        let tracker = SearchTerminationImpl {
            search_start,
            max_time: self.max_time,
            stop_signal: &self.input_rx,
        };
        let searcher = SearchImpl {
            root: self.root.clone(),
            termination_tracker: tracker,
            max_depth: self.max_depth,
        };
        searcher.search().map(|(best_move, eval, depth)| {
            SearchDetails {
                best_move,
                eval,
                depth,
                ponder: None,
                time: search_start.elapsed(),
            }
        })
    }
}

struct SearchTerminationImpl<'a, B: EvalBoard> {
    search_start: Instant,
    max_time: Duration,
    stop_signal: &'a InChannel<B>,
}

struct SearchImpl<'a, B: EvalBoard> {
    root: B,
    termination_tracker: SearchTerminationImpl<'a, B>,
    max_depth: usize,
}

impl<B: EvalBoard> SearchTerminationImpl<'_, B> {
    pub fn should_stop_search(&self) -> bool {
        self.search_start.elapsed() > self.max_time
            || match self.stop_signal.try_recv() {
            Ok(SearchInput::Stop) => true,
            _ => false,
        }
    }
}

impl<B: EvalBoard> SearchImpl<'_, B> {
    pub fn search(&self) -> Result<(Move, i32, usize), ()> {
        let mut best_move = Err(());
        for i in 1..self.max_depth + 1 {
            match self.best_move(i) {
                Err(_) => break,
                Ok((mv, eval)) => best_move = Ok((mv, eval, i)),
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
    use crate::search::SearchInput;
    use std::time::Duration;

    const DEPTH: usize = 3;

    fn test(fen_string: &'static str, expected_move_pool: Vec<Move>, is_won: bool) {
        let board = crate::eval::new_board(fen_string).unwrap();
        let (ref_board, ref_move_pool) = (board.reflect(), expected_move_pool.reflect());
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn test_impl<B: EvalBoard>(board: B, expected_move_pool: Vec<Move>, is_won: bool) {
        let (input, output) = super::init();
        input.send(SearchInput::Setup {
            root: board,
            max_depth: DEPTH,
            max_time: Duration::from_secs(120),
        });
        unimplemented!()
//        let search_result = Search::depth_capped(board, DEPTH).execute();
//        if expected_move_pool.is_empty() {
//            assert_eq!(Err(()), search_result);
//        } else {
//            let (best_move, evaluation) = search_result.unwrap().clone();
//            if is_won {
//                assert_eq!(crate::eval::WIN_VALUE, evaluation);
//            }
//            assert!(expected_move_pool.contains(&best_move), "{:?}", best_move);
//        }
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
