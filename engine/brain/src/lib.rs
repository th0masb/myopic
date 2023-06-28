#[macro_use]
#[cfg(test)]
extern crate lazy_static;

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
