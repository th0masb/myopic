use std::time::{Duration, Instant};

use serde::ser::SerializeStruct;
use serde::Serializer;

use anyhow::{anyhow, Result};
use end::SearchEnd;

use crate::moves::Move;
use crate::node;
use crate::node::TreeNode;
use crate::search::moves::MoveGenerator;
use crate::search::pv::PrincipleVariation;
use crate::search::search::{Context, SearchResponse, TreeSearcher};
pub use crate::search::table::{NodeType, TableEntry, Transpositions, TranspositionsImpl};

pub mod end;
mod moves;
mod pv;
pub mod quiescent;
pub mod search;
mod table;

const DEPTH_UPPER_BOUND: usize = 20;

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<E: SearchEnd, T: Transpositions>(
    node: TreeNode,
    parameters: SearchParameters<E, T>,
) -> Result<SearchOutcome> {
    Search { node, end: parameters.end, transpositions: parameters.table }.search()
}

pub struct SearchParameters<'a, E: SearchEnd, T: Transpositions> {
    pub end: E,
    pub table: &'a mut T,
}

/// Data class composing information/result about/of a best move search.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOutcome {
    pub best_move: Move,
    /// Larger +ve score better for side to move
    pub relative_eval: i32,
    pub depth: u8,
    pub time: Duration,
    pub optimal_path: Vec<Move>,
}

impl serde::Serialize for SearchOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SearchOutcome", 4)?;
        state.serialize_field("bestMove", &self.best_move.to_string())?;
        state.serialize_field("positionEval", &self.relative_eval)?;
        state.serialize_field("depthSearched", &self.depth)?;
        state.serialize_field("searchDurationMillis", &self.time.as_millis())?;
        state.serialize_field(
            "optimalPath",
            &self.optimal_path.iter().map(|m| m.to_string()).collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod searchoutcome_serialize_test {
    use std::time::Duration;

    use serde_json;

    use crate::constants::create_piece;
    use crate::constants::{class, corner, side, square};
    use crate::moves::Move;

    use super::SearchOutcome;

    #[test]
    fn test_json_serialize() {
        let search_outcome = SearchOutcome {
            best_move: Move::Castle { corner: corner::WK },
            relative_eval: -125,
            depth: 2,
            time: Duration::from_millis(3000),
            optimal_path: vec![
                Move::Castle { corner: corner::WK },
                Move::Normal {
                    moving: create_piece(side::B, class::P),
                    from: square::D7,
                    dest: square::D5,
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

struct Search<'a, E: SearchEnd, T: Transpositions> {
    node: TreeNode,
    end: E,
    transpositions: &'a mut T,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: u8,
}

impl<E: SearchEnd, T: Transpositions> Search<'_, E, T> {
    pub fn search(&mut self) -> Result<SearchOutcome> {
        let search_start = Instant::now();
        let mut break_err = anyhow!("Terminated before search began");
        let mut pv = PrincipleVariation::default();
        let mut best_response = None;
        for i in 1..DEPTH_UPPER_BOUND {
            match self.best_move(i as u8, search_start, &pv) {
                Err(message) => {
                    break_err = anyhow!("{}", message);
                    break;
                }
                Ok(response) => {
                    pv.set(response.path.as_slice());
                    let eval = response.eval;
                    best_response = Some(response);
                    // Inevitable checkmate detected, don't search any deeper
                    if eval.abs() == node::WIN_VALUE {
                        break;
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
        depth: u8,
        search_start: Instant,
        pv: &PrincipleVariation,
    ) -> Result<BestMoveResponse> {
        if depth < 1 {
            return Err(anyhow!("Cannot iteratively deepen with depth 0"));
        }

        let root_index = self.node.position().history.len() as u16;
        let SearchResponse { eval, path } = TreeSearcher {
            end: &self.end,
            table: self.transpositions,
            moves: MoveGenerator::default(),
            pv,
        }
        .search(
            &mut self.node,
            Context {
                depth,
                start: search_start,
                alpha: -node::INFTY,
                beta: node::INFTY,
                precursors: vec![],
                known_raise_alpha: None,
                root_index,
            },
        )?;

        // If the path returned is empty then there must be no legal moves in this position
        if path.is_empty() {
            Err(anyhow!("No moves for position {} at depth {}", self.node.position(), depth))
        } else {
            Ok(BestMoveResponse { best_move: path.get(0).unwrap().clone(), eval, path, depth })
        }
    }
}
