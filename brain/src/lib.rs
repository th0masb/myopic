extern crate itertools;
extern crate myopic_board;
extern crate myopic_core;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
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

pub use eval::*;
pub use eval_impl::EvalBoardImpl;
pub use search::interactive::interactive_search;
pub use search::interactive::InteractiveSearchCommand;
pub use search::interactive::InteractiveSearchCommandTx;
pub use search::interactive::InteractiveSearchResultRx;
pub use search::negamax::NegamaxTerminator;
pub use search::search;
pub use search::SearchOutcome;
