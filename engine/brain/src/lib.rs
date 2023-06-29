#[macro_use]
#[cfg(test)]
extern crate lazy_static;

use std::time::{Duration, Instant};

pub use eval::Evaluator;
pub use eval::tables::PositionTables;
pub use myopic_board::*;
pub use search::negascout;
pub use search::quiescent;
pub use search::search;
pub use search::SearchOutcome;
pub use search::SearchParameters;
pub use search::terminator::SearchTerminator;
pub use timing::TimeAllocator;
use crate::search::Transpositions;
use anyhow::Result;

mod eval;

mod search;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod test;
mod timing;

const TABLE_SIZE: usize = 100_000;

pub trait LookupMoveService {
    fn lookup(&mut self, position: Board) -> Result<Option<Move>>;
}

pub struct ComputeMoveInput {
    position: Board,
    remaining: Duration,
    increment: Duration,
}

pub struct ComputeMoveOutput {
    best_move: Move,
    search_details: Option<SearchOutcome>,
}

pub struct Engine {
    transpositions: Transpositions,
    lookups: Vec<Box<dyn LookupMoveService>>,
    timing: TimeAllocator,
}

impl Engine {
    pub fn compute_move(&mut self, input: ComputeMoveInput) -> Result<ComputeMoveOutput> {
        let start = Instant::now();
        let mut eval: Evaluator = input.position.into();
        match self.perform_lookups(eval.board().clone()) {
            Some(mv) => Ok(ComputeMoveOutput { best_move: mv, search_details: None }),
            None => {
                let position_count = eval.board().position_count();
                crate::search(
                    eval,
                    SearchParameters {
                        table_size: TABLE_SIZE,
                        terminator: self.timing.allocate(
                            position_count,
                            input.remaining - start.elapsed(),
                            input.increment,
                        ),
                    },
                ).map(|outcome| {
                    ComputeMoveOutput {
                        best_move: outcome.best_move.clone(),
                        search_details: Some(outcome),
                    }
                })
            }
        }
    }

    fn perform_lookups(&mut self, position: Board) -> Option<Move> {
        for service in self.lookups.iter_mut() {
            if let Ok(Some(m)) = service.lookup(position.clone()) {
                return Some(m)
            }
        }
        None
    }
}

