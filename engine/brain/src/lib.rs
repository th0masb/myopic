#[macro_use]
#[cfg(test)]
extern crate lazy_static;

pub use eval::{EvalChessBoard, MaterialParameters};
pub use eval::imp::EvalBoard;
pub use eval::tables::PositionTables;
pub use eval::values::PieceValues;
pub use myopic_board::*;
pub use search::interactive;
pub use search::negascout;
pub use search::search;
pub use search::SearchOutcome;
pub use search::SearchParameters;
pub use search::terminator::SearchTerminator;

mod eval;
pub mod pos;

mod quiescent;
mod search;
mod see;

#[cfg(test)]
mod bench;

