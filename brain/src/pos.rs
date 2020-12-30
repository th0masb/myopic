//use crate::{Board, EvalBoard, MaterialParameters, PieceValues, PositionTables};
//use anyhow::Result;
//
///// Construct an instance of the default EvalBoard implementation using the
///// position encoded as a fen string and the given parameters.
//pub fn from_fen_and_params(fen: &str, params: MaterialParameters) -> Result<EvalBoard<Board>> {
//    fen.parse::<Board>()
//        .map(|inner| EvalBoard::new(inner, params.piece_values, params.position_tables))
//}
//
///// Construct an instance of the default EvalBoard implementation using the
///// default Board implementation from a fen string.
//pub fn from_fen(fen: &str) -> Result<EvalBoard<Board>> {
//    from_fen_and_params(fen, MaterialParameters::default())
//}
//
//pub fn start() -> EvalBoard<Board> {
//    EvalBoard::new(
//        myopic_board::start(),
//        PieceValues::default(),
//        PositionTables::default(),
//    )
//}
//
