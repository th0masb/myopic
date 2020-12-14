#[macro_use]
#[cfg(test)]
extern crate lazy_static;

mod eval;
mod eval_impl;
mod quiescent;
mod search;
mod see;
pub mod tables;
pub mod values;

#[cfg(test)]
mod mate_benchmark;

pub use eval::{EvalBoard, EvalParameters, position, position_and_params, start};
pub use eval_impl::EvalBoardImpl;
pub use myopic_board::*;

pub use search::interactive;
pub use search::negamax;
pub use search::negamax::SearchTerminator;
pub use search::search;
pub use search::SearchOutcome;
