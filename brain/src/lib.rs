#[macro_use]
#[cfg(test)]
extern crate lazy_static;

mod eval;
mod eval_impl;
pub mod pos;
mod quiescent;
mod search;
mod see;
pub mod tables;
pub mod values;

#[cfg(test)]
mod bench;

pub use eval::{EvalBoard, EvalParameters};
pub use eval_impl::EvalBoardImpl;
pub use myopic_board::*;

pub use search::interactive;
pub use search::negascout;
pub use search::search;
pub use search::terminator::SearchTerminator;
pub use search::SearchOutcome;
