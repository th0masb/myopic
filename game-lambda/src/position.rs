use crate::game::InitalPosition;
use myopic_brain::{EvalBoardImpl, MutBoard, MutBoardImpl};

pub fn get(
    initial: &InitalPosition,
    uci_sequence: &str,
) -> Result<EvalBoardImpl<MutBoardImpl>, String> {
    let mut position = match initial {
        InitalPosition::Start => myopic_brain::pos::start(),
        InitalPosition::CustomFen(fen) => myopic_brain::pos::from_fen(fen.as_str())?,
    };
    for mv in myopic_brain::parse::partial_uci(&position, uci_sequence)? {
        position.evolve(&mv);
    }
    Ok(position)
}
