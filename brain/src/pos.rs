use crate::tables::PositionTables;
use crate::values::PieceValues;
use crate::{EvalBoardImpl, EvalParameters};
use myopic_board::MutBoardImpl;

/// Construct an instance of the default EvalBoard implementation using the
/// position encoded as a fen string and the given parameters.
pub fn from_fen_and_params(
    fen: &str,
    params: EvalParameters,
) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    myopic_board::fen_position(fen)
        .map(|inner| EvalBoardImpl::new(inner, params.position_tables, params.piece_values))
}

/// Construct an instance of the default EvalBoard implementation using the
/// default Board implementation from a fen string.
pub fn from_fen(fen: &str) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    from_fen_and_params(fen, EvalParameters::default())
}

pub fn from_pgn_and_params(
    pgn_sequence: &str,
    params: EvalParameters,
) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    myopic_board::parse::position_from_pgn(pgn_sequence)
        .map(|inner| EvalBoardImpl::new(inner, params.position_tables, params.piece_values))
}

pub fn from_pgn(pgn_sequence: &str) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    from_pgn_and_params(pgn_sequence, EvalParameters::default())
}

pub fn from_uci_and_params(
    uci_sequence: &str,
    params: EvalParameters,
) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    myopic_board::parse::position_from_uci(uci_sequence)
        .map(|inner| EvalBoardImpl::new(inner, params.position_tables, params.piece_values))
}

pub fn from_uci(uci_sequence: &str) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    from_uci_and_params(uci_sequence, EvalParameters::default())
}

pub fn start() -> EvalBoardImpl<MutBoardImpl> {
    EvalBoardImpl::new(
        myopic_board::start_position(),
        PositionTables::default(),
        PieceValues::default(),
    )
}
