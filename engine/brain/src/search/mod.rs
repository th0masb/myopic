use std::time::{Duration, Instant};

use serde::ser::SerializeStruct;
use serde::Serializer;

use myopic_board::anyhow::{anyhow, Result};
use myopic_board::Move;
use terminator::SearchTerminator;

use crate::search::negascout::{Context, Scout, SearchResponse};
use crate::search::pv::PrincipleVariation;
pub use crate::search::transpositions::{Transpositions, TranspositionsImpl, TreeNode};
use crate::{eval, Evaluator};

mod moves;
pub mod negascout;
mod pv;
pub mod quiescent;
pub mod terminator;
mod transpositions;

const DEPTH_UPPER_BOUND: usize = 20;

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<T: SearchTerminator, TT: Transpositions>(
    root: Evaluator,
    parameters: SearchParameters<T, TT>,
) -> Result<SearchOutcome> {
    Search { root, terminator: parameters.terminator, transpositions: parameters.table }.search()
}

pub struct SearchParameters<'a, T: SearchTerminator, TT: Transpositions> {
    pub terminator: T,
    pub table: &'a mut TT,
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

    use myopic_board::{Corner, Move, Piece, Square};

    use crate::{Class, Flank, Side};

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

struct Search<'a, T: SearchTerminator, TT: Transpositions> {
    root: Evaluator,
    terminator: T,
    transpositions: &'a mut TT,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: u8,
}

impl<T: SearchTerminator, TT: Transpositions> Search<'_, T, TT> {
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
                    best_response = Some(response);
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

        // TODO If any move in the current position leads to a draw by repetition then disable the
        //  transposition table early break?
        let SearchResponse { eval, path } = Scout {
            terminator: &self.terminator,
            transpositions: self.transpositions,
            moves: pv.into(),
        }
        .search(
            &mut self.root,
            Context {
                depth,
                start: search_start,
                alpha: -eval::INFTY,
                beta: eval::INFTY,
                precursors: vec![],
            },
        )?;

        // If the path returned is empty then there must be no legal moves in this position
        if path.is_empty() {
            Err(anyhow!("No moves for position {} at depth {}", self.root.board().to_fen(), depth))
        } else {
            Ok(BestMoveResponse { best_move: path.get(0).unwrap().clone(), eval, path, depth })
        }
    }
}
