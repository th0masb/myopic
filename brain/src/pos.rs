use crate::{Board, EvalBoardImpl, EvalParameters, PieceValues, PositionTables};
use anyhow::Result;

/// Construct an instance of the default EvalBoard implementation using the
/// position encoded as a fen string and the given parameters.
pub fn from_fen_and_params(fen: &str, params: EvalParameters) -> Result<EvalBoardImpl<Board>> {
    fen.parse::<Board>()
        .map(|inner| EvalBoardImpl::new(inner, params.piece_values, params.position_tables))
}

/// Construct an instance of the default EvalBoard implementation using the
/// default Board implementation from a fen string.
pub fn from_fen(fen: &str) -> Result<EvalBoardImpl<Board>> {
    from_fen_and_params(fen, EvalParameters::default())
}

pub fn start() -> EvalBoardImpl<Board> {
    EvalBoardImpl::new(
        myopic_board::start(),
        PieceValues::default(),
        PositionTables::default(),
    )
}
