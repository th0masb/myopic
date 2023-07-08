#[macro_use]
#[cfg(test)]
extern crate lazy_static;

use std::time::{Duration, Instant};

pub use crate::search::{Transpositions, TranspositionsImpl};
use anyhow::Result;
pub use eval::tables::PositionTables;
pub use eval::Evaluator;
pub use myopic_board::*;
pub use search::negascout;
pub use search::quiescent;
pub use search::search;
pub use search::terminator::SearchTerminator;
pub use search::SearchOutcome;
pub use search::SearchParameters;
pub use timing::TimeAllocator;

mod eval;

mod search;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod test;
mod timing;

pub trait LookupMoveService: Send + Sync {
    fn lookup(&mut self, position: Board) -> Result<Option<Move>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputeMoveInput {
    pub position: Board,
    pub remaining: Duration,
    pub increment: Duration,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComputeMoveOutput {
    pub best_move: Move,
    pub search_details: Option<SearchOutcome>,
}

pub struct Engine {
    transpositions: TranspositionsImpl,
    lookups: Vec<Box<dyn LookupMoveService>>,
    timing: TimeAllocator,
}

impl Engine {
    pub fn new(table_size: usize, lookups: Vec<Box<dyn LookupMoveService>>) -> Engine {
        Engine {
            transpositions: TranspositionsImpl::new(table_size),
            lookups,
            timing: TimeAllocator::default(),
        }
    }

    pub fn compute_move(&mut self, input: ComputeMoveInput) -> Result<ComputeMoveOutput> {
        let start = Instant::now();
        let eval: Evaluator = input.position.into();
        match self.perform_lookups(eval.board().clone()) {
            Some(mv) => Ok(ComputeMoveOutput { best_move: mv, search_details: None }),
            None => {
                let position_count = eval.board().position_count();
                crate::search(
                    eval,
                    SearchParameters {
                        table: &mut self.transpositions,
                        terminator: self.timing.allocate(
                            position_count,
                            input.remaining - start.elapsed(),
                            input.increment,
                        ),
                    },
                )
                .map(|outcome| ComputeMoveOutput {
                    best_move: outcome.best_move.clone(),
                    search_details: Some(outcome),
                })
            }
        }
    }

    fn perform_lookups(&mut self, position: Board) -> Option<Move> {
        for service in self.lookups.iter_mut() {
            if let Ok(Some(m)) = service.lookup(position.clone()) {
                return Some(m);
            }
        }
        None
    }
}
