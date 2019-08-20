use crate::board;
use crate::board::Board;
use crate::board::BoardImpl;

pub use self::evalboardimpl::SimpleEvalBoard;
pub use self::see::exchange_value;

mod evalboardimpl;
mod see;
pub mod tables;
pub mod values;

/// The evaluation upper/lower bound definition
pub const INFTY: i32 = 500_000i32;

/// The evaluation assigned to a won position.
pub const WIN_VALUE: i32 = INFTY - 1;

/// The evaluation assigned to a lost position.
pub const LOSS_VALUE: i32 = -WIN_VALUE;

/// The evaluation assigned to a drawn position.
pub const DRAW_VALUE: i32 = 0;

/// Extension of the Board trait which adds a static evaluation function.
pub trait EvalBoard: Board {
    /// The static evaluation function assigns a score to this exact
    /// position at the point of time it is called. It does not take
    /// into account potential captures/recaptures etc. It must follow
    /// the rule that 'a higher score is best for the active side'. That
    /// is if it is white to move next then a high positive score indicates
    /// a favorable position for white and if it is black to move a high
    /// positive score indicates a favorable position for black. If the
    /// state it terminal it must return the LOSS_VALUE or DRAW_VALUE
    /// depending on the type of termination.
    fn static_eval(&mut self) -> i32;
}

/// Construct an instance of the default EvalBoard implementation using the
/// default Board implementation from a fen string.
pub fn new_board(fen_string: &'static str) -> Result<SimpleEvalBoard<BoardImpl>, String> {
    board::from_fen(fen_string).map(SimpleEvalBoard::new)
}
