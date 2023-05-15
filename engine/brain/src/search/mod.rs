use std::marker::PhantomData;
use std::time::{Duration, Instant};

use serde::ser::SerializeStruct;
use serde::Serializer;

use movehints::MoveOrderingHints;
use myopic_board::anyhow::{anyhow, Result};
use myopic_board::Move;
use terminator::SearchTerminator;

use crate::eval;
use crate::eval::EvalChessBoard;
use crate::search::movequality::MaterialAndPositioningHeuristic;
use crate::search::negascout::{Scout, SearchContext, SearchResponse};
use crate::search::transpositions::TranspositionTable;

pub mod interactive;
mod movehints;
mod movequality;
pub mod negascout;
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
    use std::time::Duration;

    use serde_json;

    use myopic_board::{CastleZone, Move, Piece, Square};

    use super::SearchOutcome;

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
    pub fn search(&mut self, transposition_table_size: usize) -> Result<SearchOutcome> {
        let search_start = Instant::now();
        let mut break_err = anyhow!("Terminated before search began");
        let mut ordering_hints = MoveOrderingHints::default();
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
                        ordering_hints.populate(&mut self.root, SHALLOW_EVAL_DEPTH);
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
        &mut self,
        depth: usize,
        search_start: Instant,
        ordering_hints: &MoveOrderingHints,
        transposition_table: &mut TranspositionTable,
    ) -> Result<BestMoveResponse> {
        if depth < 1 {
            return Err(anyhow!("Cannot iteratively deepen with depth 0"));
        }

        let SearchResponse { eval, mut path } = Scout {
            terminator: &self.terminator,
            ordering_hints,
            move_quality_estimator: MaterialAndPositioningHeuristic,
            transposition_table,
            board_type: PhantomData,
        }
        .search(
            &mut self.root,
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
    use myopic_board::{Board, ChessBoard, Reflectable};

    use crate::eval::EvalChessBoard;
    use crate::search::SearchParameters;
    use crate::{eval, EvalBoard, UciMove};

    const DEPTH: usize = 3;
    const TABLE_SIZE: usize = 10_000;

    enum Setup {
        Fen(&'static str),
        Pgn(&'static str),
    }

    fn test(setup: Setup, expected_move_pool: Vec<UciMove>, is_won: bool) {
        match setup {
            Setup::Fen(fen_string) => {
                let base_board = fen_string.parse::<Board>().unwrap();
                let ref_board = EvalBoard::from(base_board.reflect());
                let board = EvalBoard::from(base_board);
                let ref_move_pool = expected_move_pool.reflect();
                test_impl(board, expected_move_pool, is_won);
                test_impl(ref_board, ref_move_pool, is_won);
            }
            Setup::Pgn(pgn_string) => {
                let mut board = EvalBoard::default();
                board.play_pgn(pgn_string).unwrap();
                test_impl(board, expected_move_pool, is_won)
            }
        }
    }

    fn test_impl<B: EvalChessBoard>(board: B, expected_move_pool: Vec<UciMove>, is_won: bool) {
        match super::search(
            board,
            SearchParameters {
                terminator: DEPTH,
                table_size: TABLE_SIZE,
            },
        ) {
            Err(message) => panic!("{}", message),
            Ok(outcome) => {
                assert!(
                    expected_move_pool
                        .contains(&UciMove::new(outcome.best_move.uci_format().as_str()).unwrap()),
                    "{}",
                    serde_json::to_string(&outcome).unwrap()
                );
                if is_won {
                    assert_eq!(eval::WIN_VALUE, outcome.eval);
                }
            }
        }
    }

    #[test]
    fn queen_escape_attack() {
        test(
            Setup::Fen("r4rk1/5ppp/8/1Bn1p3/Q7/8/5PPP/1R3RK1 w Qq - 5 27"),
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
            Setup::Fen("r2r2k1/5ppp/1N2p3/1n6/3Q4/2B5/5PPP/1R3RK1 w Qq - 4 21"),
            vec![UciMove::new("d4g7").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_1() {
        test(
            Setup::Fen("8/8/8/4Q3/8/6R1/2n1pkBK/8 w - - 0 1"),
            vec![UciMove::new("g3d3").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_2() {
        test(
            Setup::Fen("8/7B/5Q2/6p1/6k1/8/5K2/8 w - - 0 1"),
            vec![UciMove::new("f6h8").unwrap(), UciMove::new("f6f3").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_3() {
        test(
            Setup::Fen("3qr2k/1b1p2pp/7N/3Q2b1/4P3/8/5PP1/6K1 w - - 0 1"),
            vec![UciMove::new("d5g8").unwrap()],
            true,
        )
    }

    // Mate in 4 moves TODO probably better in benchmark.
    #[ignore]
    #[test]
    fn mate_4() {
        test(
            Setup::Fen("r1k2b1r/pp4pp/2p1n3/3NQ1B1/6q1/8/PPP2P1P/2KR4 w - - 4 20"),
            vec![UciMove::new("e5c7").unwrap()],
            true,
        )
    }

    #[test]
    fn mate_5() {
        test(
            Setup::Fen("r1b1k1nr/p2p1ppp/n2B4/1p1NPN1P/6P1/3P1Q2/P1P1K3/q5b1 w - - 0 30"),
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
            Setup::Fen("1r3k2/2R5/1p2p2p/1Q1pPp1q/1P1P2p1/2P1P1P1/6KP/8 b - - 2 31"),
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
            Setup::Fen("r5k1/pb4pp/1pn1pq2/5B2/2Pr4/B7/PP3RPP/R4QK1 b - - 0 23"),
            vec![UciMove::new("e6f5").unwrap()],
            false,
        )
    }

    #[test]
    fn prefer_castling() {
        test(
            Setup::Pgn("1. e4 e5 2. f4 exf4 3. Nf3 g5 4. Nc3 Nc6 5. g3 g4 6. Nh4 Nd4 7. Bc4 Be7"),
            vec![UciMove::new("e1g1").unwrap()],
            false,
        )
    }
}
