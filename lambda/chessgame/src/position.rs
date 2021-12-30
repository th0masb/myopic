use myopic_brain::{Board, ChessBoard, EvalBoard};
use myopic_brain::anyhow::Result;

use crate::game::InitalPosition;

pub fn get(
    initial: &InitalPosition,
    uci_sequence: &str,
) -> Result<EvalBoard<Board>> {
    let mut position = match initial {
        InitalPosition::Start => myopic_brain::start(),
        InitalPosition::CustomFen(fen) => fen.parse()?,
    };
    position.play_uci(uci_sequence)?;
    Ok(EvalBoard::builder(position).build())
}
