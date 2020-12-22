use crate::eval::tables::PositionTables;
use crate::eval::values::PieceValues;
use myopic_board::{Discards, Move, MutBoard, Piece, Square};
use serde_derive::{Deserialize, Serialize};

pub mod eval_impl;
mod material;
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
pub trait EvalBoard: MutBoard {
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

    /// The value each piece is considered to have in the current
    /// state of the game.
    fn piece_values(&self) -> &[i32; 6];

    /// The positional (table) value of the given piece situated at the
    /// given square in the context of this position.
    fn positional_eval(&self, piece: Piece, location: Square) -> i32;
}

pub trait EvalComponent {
    fn static_eval(&mut self) -> i32;

    fn evolve(&mut self, mv: &Move);

    fn devolve(&mut self, mv: &Move, discards: &Discards);

    fn replicate(&self) -> Box<dyn EvalComponent>;
}

/// Allows one to configure the parameters of the evaluation board.
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Default)]
pub struct EvalParameters {
    pub piece_values: PieceValues,
    pub position_tables: PositionTables,
}
