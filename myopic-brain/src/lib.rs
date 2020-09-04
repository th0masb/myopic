extern crate itertools;
extern crate myopic_board;
extern crate myopic_core;
#[macro_use]
#[cfg(test)]
extern crate lazy_static;

pub mod eval;
mod eval_impl;
mod quiescent;
pub mod search;
mod see;
mod tables;
mod values;

#[cfg(test)]
mod mate_benchmark;

pub use eval::EvalBoard;
pub use eval_impl::EvalBoardImpl;
pub use search::SearchCommand;
pub use search::SearchCommandTx;
pub use search::SearchDetails;
pub use search::SearchResult;
pub use search::SearchResultRx;
