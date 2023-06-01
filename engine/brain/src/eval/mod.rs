use serde_derive::{Deserialize, Serialize};

use myopic_board::{ChessBoard, Move, Piece, Square};

use crate::eval::tables::PositionTables;
use crate::eval::values::PieceValues;

pub mod imp;
mod material;
pub mod tables;
pub mod values;
mod castling;
mod development;
mod antipattern;

/// The evaluation upper/lower bound definition
pub const INFTY: i32 = 500_000i32;

/// The evaluation assigned to a won position.
pub const WIN_VALUE: i32 = INFTY - 1;

/// The evaluation assigned to a lost position.
pub const LOSS_VALUE: i32 = -WIN_VALUE;

/// The evaluation assigned to a drawn position.
pub const DRAW_VALUE: i32 = 0;

/// Extension of the Board trait which adds a static evaluation function.
pub trait EvalChessBoard: ChessBoard {
    /// The relative evaluation function assigns a score to this exact
    /// position at the point of time it is called. It does not take
    /// into account potential captures/recaptures etc. It must follow
    /// the rule that 'A LARGER +VE SCORE BETTER FOR ACTIVE, LARGER -VE
    /// SCORE BETTER FOR PASSIVE'. That is if it is white to move next
    /// then a high positive score indicates a favorable position for
    /// white and if it is black to move a high positive score indicates
    /// a favorable position for black. If the state it terminal it must
    /// return the LOSS_VALUE or DRAW_VALUE depending on the type of
    /// termination.
    fn relative_eval(&self) -> i32;

    /// The value each piece is considered to have in the current
    /// state of the game.
    fn piece_values(&self) -> &[i32; 6];

    /// The positional (table) value of the given piece situated at the
    /// given square in the context of this position.
    fn positional_eval(&self, piece: Piece, location: Square) -> i32;
}

/// Represents some (possibly stateful) feature of a position which can be
/// evaluated.
pub trait EvalFacet<B : ChessBoard> {
    /// Return the static evaluation of the given position. Implementors are guaranteed
    /// that exactly the same move sequence will have been passed to this component
    /// and the given board position. I.e the internal states are aligned. It must
    /// follow the rule 'A LARGER +VE SCORE BETTER FOR WHITE, LARGER -VE SCORE BETTER
    /// FOR BLACK'.
    fn static_eval(&self, board: &B) -> i32;

    /// Update internal state by making the given move FROM the given position
    fn make(&mut self, mv: &Move, board: &B);

    /// Update internal state by unmaking the given move which is guaranteed to have
    /// previously been passed to the "make" method.
    fn unmake(&mut self, mv: &Move);
}

/// Allows one to configure the parameters of the evaluation board.
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Default)]
pub struct MaterialParameters {
    pub piece_values: PieceValues,
    pub position_tables: PositionTables,
}
