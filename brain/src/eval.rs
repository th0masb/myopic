use crate::eval_impl::EvalBoardImpl;
use crate::tables::PositionTables;
use crate::values::PieceValues;
use myopic_board::{MutBoard, MutBoardImpl};

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
}

/// Allows one to configure the parameters of the evaluation board.
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq)]
pub struct EvalParameters {
    piece_values: PieceValues,
    position_tables: PositionTables,
}

impl EvalParameters {
    /// Initialise a new instance of the default parameters
    pub fn default() -> EvalParameters {
        EvalParameters {
            piece_values: PieceValues::default(),
            position_tables: PositionTables::default(),
        }
    }
}

/// Construct an instance of the default EvalBoard implementation using the
/// position encoded as a fen string and the given parameters.
pub fn init_board(
    position_fen: &str,
    parameters: EvalParameters,
) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    myopic_board::fen_position(position_fen)
        .map(|pos| EvalBoardImpl::new(pos, parameters.position_tables, parameters.piece_values))
}

/// Construct an instance of the default EvalBoard implementation using the
/// default Board implementation from a fen string.
#[deprecated]
pub fn new_board(fen_string: &str) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    init_board(fen_string, EvalParameters::default())
}

#[deprecated]
pub fn start() -> EvalBoardImpl<MutBoardImpl> {
    EvalBoardImpl::new(
        myopic_board::start_position(),
        PositionTables::default(),
        PieceValues::default(),
    )
}
