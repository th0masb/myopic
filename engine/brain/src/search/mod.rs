use std::time::{Duration, Instant};

use serde::ser::SerializeStruct;
use serde::Serializer;

use movehints::MoveOrderingHints;
use myopic_board::anyhow::{anyhow, Result};
use myopic_board::Move;
use terminator::SearchTerminator;

use crate::search::movequality::MaterialAndPositioningHeuristic;
use crate::search::negascout::{Scout, SearchContext, SearchResponse};
use crate::search::transpositions::TranspositionTable;
use crate::{eval, Evaluator};

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
pub fn search<T>(root: Evaluator, parameters: SearchParameters<T>) -> Result<SearchOutcome>
where
    T: SearchTerminator,
{
    Search { root, terminator: parameters.terminator }.search(parameters.table_size)
}

pub struct SearchParameters<T: SearchTerminator> {
    pub terminator: T,
    pub table_size: usize,
}

/// Data class composing information/result about/of a best move search.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOutcome {
    pub best_move: Move,
    /// Larger +ve score better for side to move
    pub relative_eval: i32,
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
        state.serialize_field("positionEval", &self.relative_eval)?;
        state.serialize_field("depthSearched", &self.depth)?;
        state.serialize_field("searchDurationMillis", &self.time.as_millis())?;
        state.serialize_field(
            "optimalPath",
            &self.optimal_path.iter().map(|m| m.uci_format()).collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod searchoutcome_serialize_test {
    use std::time::Duration;

    use serde_json;

    use crate::{Class, Flank, Side};
    use myopic_board::{Corner, Move, Piece, Square};

    use super::SearchOutcome;

    #[test]
    fn test_json_serialize() {
        let search_outcome = SearchOutcome {
            best_move: Move::Castle { corner: Corner(Side::W, Flank::K) },
            relative_eval: -125,
            depth: 2,
            time: Duration::from_millis(3000),
            optimal_path: vec![
                Move::Castle { corner: Corner(Side::W, Flank::K) },
                Move::Standard {
                    moving: Piece(Side::B, Class::P),
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

struct Search<T: SearchTerminator> {
    root: Evaluator,
    terminator: T,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: usize,
}

impl<T: SearchTerminator> Search<T> {
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

        best_response.ok_or(break_err).map(|response| SearchOutcome {
            best_move: response.best_move,
            relative_eval: response.eval,
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
            Err(anyhow!("No moves for position {} at depth {}", self.root.board().to_fen(), depth))
        } else {
            Ok(BestMoveResponse { best_move: path.get(0).unwrap().clone(), eval, path, depth })
        }
    }
}
