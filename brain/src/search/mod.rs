use std::time::{Duration, Instant};

use crate::eval;
use crate::eval::EvalBoard;
use crate::search::negamax::{SearchContext, SearchResponse, SearchTerminator, Searcher};
use crate::search::ordering::EstimatorImpl;
use myopic_board::Move;
use serde::export::PhantomData;
use serde::ser::SerializeStruct;
use serde::Serializer;

pub mod interactive;
pub mod negamax;
mod ordering;

const DEPTH_UPPER_BOUND: usize = 10;

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<B, T>(root: B, terminator: T) -> Result<SearchOutcome, String>
where
    B: EvalBoard,
    T: SearchTerminator,
{
    Search { root, terminator }.search()
}

/// Data class composing information/result about/of a best move search.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOutcome {
    pub best_move: Move,
    pub eval: i32,
    pub depth: usize,
    pub time: Duration,
    pub optimal_path: Vec<Move>,
}

impl serde::Serialize for SearchOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SearchOutcome", 4)?;
        state.serialize_field("bestMove", &self.best_move.uci_format())?;
        state.serialize_field("positionEval", &self.eval)?;
        state.serialize_field("depthSearched", &self.depth)?;
        state.serialize_field("searchDurationMillis", &self.time.as_millis())?;
        state.serialize_field(
            "optimalPath",
            &self
                .optimal_path
                .iter()
                .map(|m| m.uci_format())
                .collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod searchoutcome_serialize_test {
    use super::SearchOutcome;
    use myopic_board::{CastleZone, Move, Piece, Square};
    use serde_json;
    use std::time::Duration;

    #[test]
    fn test_json_serialize() {
        let search_outcome = SearchOutcome {
            best_move: Move::Castle(CastleZone::WK),
            eval: -125,
            depth: 2,
            time: Duration::from_millis(3000),
            optimal_path: vec![
                Move::Castle(CastleZone::WK),
                Move::Standard(Piece::BP, Square::D7, Square::D5),
            ],
        };
        assert_eq!(
            r#"{"bestMove":"e1g1","positionEval":-125,"depthSearched":2,"searchDurationMillis":3000,"optimalPath":["e1g1","d7d5"]}"#,
            serde_json::to_string(&search_outcome).expect("Serialization failed")
        );
    }
}

struct Search<B: EvalBoard, T: SearchTerminator> {
    root: B,
    terminator: T,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: usize,
}

impl<B: EvalBoard, T: SearchTerminator> Search<B, T> {
    pub fn search(&self) -> Result<SearchOutcome, String> {
        let search_start = Instant::now();
        let mut break_message = format!("Terminated before search began");
        let mut best_move_response = None;
        let mut principle_variation = vec![];

        for i in 1..DEPTH_UPPER_BOUND {
            match self.best_move(i, search_start, &principle_variation) {
                Err(message) => {
                    break_message = message;
                    break;
                }
                Ok(response) => {
                    principle_variation = response.path.clone();
                    best_move_response = Some(response);
                }
            }
        }

        best_move_response
            .ok_or(break_message)
            .map(|response| SearchOutcome {
                best_move: response.best_move,
                eval: response.eval,
                depth: response.depth,
                time: search_start.elapsed(),
                optimal_path: response.path,
            })
    }

    fn best_move(
        &self,
        depth: usize,
        search_start: Instant,
        principle_variation: &Vec<Move>,
    ) -> Result<BestMoveResponse, String> {
        if depth < 1 {
            return Err(format!("Illegal depth: {}", depth));
        }

        let SearchResponse { eval, mut path } = Searcher {
            terminator: &self.terminator,
            principle_variation,
            move_quality_estimator: EstimatorImpl,
            board_type: PhantomData,
        }
        .search(
            &mut self.root.clone(),
            SearchContext {
                depth_remaining: depth,
                start_time: search_start,
                alpha: -eval::INFTY,
                beta: eval::INFTY,
                precursors: vec![],
            },
        )?;

        // The path returned from the negamax function is ordered deepest move -> shallowest
        // so we reverse as the shallowest move is the one we make in this position.
        path.reverse();
        // If the path returned is empty then there must be no legal moves in this position
        if path.is_empty() {
            Err(format!(
                "No moves found for position {}",
                self.root.to_fen()
            ))
        } else {
            Ok(BestMoveResponse {
                best_move: path.get(0).unwrap().clone(),
                eval,
                path,
                depth,
            })
        }
    }
}

/// Tests for 'obvious' positions like taking a hanging piece,
/// checkmating or escaping checkmate etc.
#[cfg(test)]
mod test {
    use crate::eval;
    use crate::eval::EvalBoard;
    use myopic_board::{Move, Move::Standard, Piece, Reflectable, Square, Square::*};

    const DEPTH: usize = 3;

    fn test(fen_string: &'static str, expected_move_pool: Vec<Move>, is_won: bool) {
        let board = crate::eval::position(fen_string).unwrap();
        let (ref_board, ref_move_pool) = (board.reflect(), expected_move_pool.reflect());
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn test_impl<B: EvalBoard>(board: B, expected_move_pool: Vec<Move>, is_won: bool) {
        match super::search(board, DEPTH) {
            Err(message) => panic!("{}", message),
            Ok(outcome) => {
                assert!(expected_move_pool.contains(&outcome.best_move), serde_json::to_string(&outcome).unwrap());
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

    /// A funny one which currently depends on move ordering, at depth 3 the
    /// best move has the same evaluation as another inferior move.
    #[ignore]
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
