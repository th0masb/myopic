use std::time::{Duration, Instant};

use crate::eval;
use crate::eval::EvalChessBoard;
use crate::search::negascout::{Scout, SearchContext, SearchResponse};
use crate::search::ordering::EstimatorImpl;
use crate::search::transpositions::TranspositionTable;
use anyhow::{anyhow, Result};
use myopic_board::Move;
use ordering_hints::OrderingHints;
use serde::export::PhantomData;
use serde::ser::SerializeStruct;
use serde::Serializer;
use terminator::SearchTerminator;

pub mod interactive;
pub mod negascout;
mod ordering;
mod ordering_hints;
pub mod terminator;
mod transpositions;

const DEPTH_UPPER_BOUND: usize = 10;
const SHALLOW_EVAL_TRIGGER_DEPTH: usize = 2;
const SHALLOW_EVAL_DEPTH: usize = 1;

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<B, T>(root: B, parameters: SearchParameters<T>) -> Result<SearchOutcome>
where
    B: EvalChessBoard,
    T: SearchTerminator,
{
    Search {
        root,
        terminator: parameters.terminator,
    }
    .search(parameters.table_size)
}

pub struct SearchParameters<T: SearchTerminator> {
    pub terminator: T,
    pub table_size: usize,
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
            best_move: Move::Castle {
                source: 0,
                zone: CastleZone::WK,
            },
            eval: -125,
            depth: 2,
            time: Duration::from_millis(3000),
            optimal_path: vec![
                Move::Castle {
                    source: 0,
                    zone: CastleZone::WK,
                },
                Move::Standard {
                    source: 1,
                    moving: Piece::BP,
                    from: Square::D7,
                    dest: Square::D5,
                    capture: None,
                },
            ],
        };
        assert_eq!(
            r#"{"bestMove":"e1g1","positionEval":-125,"depthSearched":2,"searchDurationMillis":3000,"optimalPath":["e1g1","d7d5"]}"#,
            serde_json::to_string(&search_outcome).expect("Serialization failed")
        );
    }
}

struct Search<B: EvalChessBoard, T: SearchTerminator> {
    root: B,
    terminator: T,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: usize,
}

impl<B: EvalChessBoard, T: SearchTerminator> Search<B, T> {
    pub fn search(&self, transposition_table_size: usize) -> Result<SearchOutcome> {
        let search_start = Instant::now();
        let mut break_err = anyhow!("Terminated before search began");
        let mut ordering_hints = OrderingHints::new(self.root.clone());
        // TODO inject desired size
        let mut transposition_table = TranspositionTable::new(transposition_table_size)?;
        let mut best_response = None;

        for i in 1..DEPTH_UPPER_BOUND {
            match self.best_move(i, search_start, &ordering_hints, &mut transposition_table) {
                Err(message) => {
                    break_err = anyhow!("{}", message);
                    break;
                }
                Ok(response) => {
                    ordering_hints.add_pv(i, &response.path);
                    best_response = Some(response);
                    // Only fill in the shallow eval when we get deep
                    // enough to male it worthwhile
                    if i == SHALLOW_EVAL_TRIGGER_DEPTH {
                        ordering_hints.populate_shallow_eval(SHALLOW_EVAL_DEPTH);
                    }
                }
            }
        }

        best_response
            .ok_or(break_err)
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
        ordering_hints: &OrderingHints<B>,
        transposition_table: &mut TranspositionTable,
    ) -> Result<BestMoveResponse> {
        if depth < 1 {
            return Err(anyhow!("Cannot iteratively deepen with depth 0"));
        }

        let SearchResponse { eval, mut path } = Scout {
            terminator: &self.terminator,
            ordering_hints,
            move_quality_estimator: EstimatorImpl,
            transposition_table,
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
            Err(anyhow!(
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
    use crate::eval::EvalChessBoard;
    use crate::search::SearchParameters;
    use crate::{eval, EvalBoard, UciMove, SearchOutcome};
    use myopic_board::{Reflectable, Board, ChessBoard};

    const DEPTH: usize = 3;
    const TABLE_SIZE: usize = 10_000;

    fn test(fen_string: &'static str, expected_move_pool: Vec<UciMove>, is_won: bool) {
        let base_board = fen_string.parse::<Board>().unwrap();
        let ref_board = EvalBoard::builder(base_board.reflect()).build();
        let board = EvalBoard::builder(base_board).build();
        let ref_move_pool = expected_move_pool.reflect();
        test_impl(board, expected_move_pool, is_won);
        test_impl(ref_board, ref_move_pool, is_won);
    }

    fn search<B: EvalChessBoard>(board: B) -> SearchOutcome {
        match super::search(
            board,
            SearchParameters {
                terminator: DEPTH,
                table_size: TABLE_SIZE,
            },
        ) {
            Err(message) => panic!("{}", message),
            Ok(outcome) => outcome,
        }
    }

    fn test_impl<B: EvalChessBoard>(board: B, expected_move_pool: Vec<UciMove>, is_won: bool) {
        let outcome = search(board);
        assert!(
            expected_move_pool
                .contains(&UciMove::new(outcome.best_move.uci_format().as_str()).unwrap()),
            serde_json::to_string(&outcome).unwrap()
        );
        if is_won {
            assert_eq!(eval::WIN_VALUE, outcome.eval);
        }
    }

    #[test]
    fn stalemate_not_chosen() {
        let pgn = "1. e4 c5 2. c3 e6 3. d4 cxd4 4. cxd4 d5 5. e5 Nc6 6. Nf3 Bb4+ 7. Nc3 Nge7 8. Bd3 Qb6 9. Bg5 Bd7 10. Bxe7 Bxe7 11. O-O Nxd4 12. Nxd4 Qxd4 13. Re1 Bc5 14. Qc2 Rc8 15. Rad1 Qf4 16. g3 Qf3 17. Re2 Bd4 18. Bxh7 Bxc3 19. Rd3 Qh5 20. Rxc3 Rxc3 21. bxc3 Rxh7 22. f3 Qxf3 23. c4 dxc4 24. Rf2 Qd3 25. Qb2 Bc6 26. Qc2 Qxc2 27. Rxc2 Bd5 28. a4 Rh5 29. Re2 c3 30. Rc2 Rxe5 31. Kf1 Re3 32. Rc1 Bb3 33. Kf2 Rd3 34. a5 c2 35. a6 b6 36. h3 Rd1 37. Rxc2 Bxc2 38. Ke3 Rh1 39. h4 Rh3 40. Kf3 g5 41. hxg5 Bf5 42. Kf4 Ke7 43. Kf3 Kd6 44. Kf4 Kd5 45. Kf3 Kd4 46. g6 f6 47. g7 Bh7 48. Kg2 Rh5 49. Kf2 Rg5 50. g8=Q Rxg8 51. g4 Rg5 52. Kg3 Bf5 53. Kf3 Bxg4+ 54. Kf4 Bf5 55. Kf3 Bd3 56. Kf2 Bxa6 57. Kf3 b5 58. Kf4";
        let mut board = crate::EvalBoard::start();
        board.play_pgn(pgn).unwrap();
        let outcome = crate::search(board, SearchParameters {
            terminator: 6,
            table_size: 10000,
        }).unwrap();
        assert!(false, serde_json::to_string(&outcome).unwrap())
    }

    #[test]
    fn queen_escape_attack() {
        test(
            "r4rk1/5ppp/8/1Bn1p3/Q7/8/5PPP/1R3RK1 w Qq - 5 27",
            vec![
                UciMove::new("a4b4").unwrap(),
                UciMove::new("a4c4").unwrap(),
                UciMove::new("a4g4").unwrap(),
                UciMove::new("a4h4").unwrap(),
                UciMove::new("a4c2").unwrap(),
                UciMove::new("a4d1").unwrap(),
            ],
            false,
        )
    }

    #[test]
    fn mate_0() {
        test(
            "r2r2k1/5ppp/1N2p3/1n6/3Q4/2B5/5PPP/1R3RK1 w Qq - 4 21",
            vec![UciMove::new("d4g7").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_1() {
        test(
            "8/8/8/4Q3/8/6R1/2n1pkBK/8 w - - 0 1",
            vec![UciMove::new("g3d3").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_2() {
        test(
            "8/7B/5Q2/6p1/6k1/8/5K2/8 w - - 0 1",
            vec![UciMove::new("f6h8").unwrap(), UciMove::new("f6f3").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_3() {
        test(
            "3qr2k/1b1p2pp/7N/3Q2b1/4P3/8/5PP1/6K1 w - - 0 1",
            vec![UciMove::new("d5g8").unwrap()],
            true,
        )
    }

    // Mate in 4 moves TODO probably better in benchmark.
    #[ignore]
    #[test]
    fn mate_4() {
        test(
            "r1k2b1r/pp4pp/2p1n3/3NQ1B1/6q1/8/PPP2P1P/2KR4 w - - 4 20",
            vec![UciMove::new("e5c7").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_5() {
        test(
            "r1b1k1nr/p2p1ppp/n2B4/1p1NPN1P/6P1/3P1Q2/P1P1K3/q5b1 w - - 0 30",
            vec![UciMove::new("f5g7").unwrap()],
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
            vec![UciMove::new("b8a8").unwrap()],
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
            vec![UciMove::new("e6f5").unwrap()],
            false,
        )
    }
}
