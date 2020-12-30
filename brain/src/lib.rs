#[macro_use]
#[cfg(test)]
extern crate lazy_static;

mod eval;
pub mod pos;

mod quiescent;
mod search;
mod see;

#[cfg(test)]
mod bench;

pub use eval::eval_impl::EvalBoard;
pub use eval::tables::PositionTables;
pub use eval::values::PieceValues;
pub use eval::{EvalChessBoard, MaterialParameters};
pub use myopic_board::*;

pub use search::interactive;
pub use search::negascout;
pub use search::search;
pub use search::terminator::SearchTerminator;
pub use search::SearchOutcome;
