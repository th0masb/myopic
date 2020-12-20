use myopic_brain::{parse, EvalBoardImpl, MutBoard, MutBoardImpl};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

pub fn get_game_state(moves: &String) -> Result<(EvalBoardImpl<MutBoardImpl>, u32), String> {
    let moves = parse::uci(moves)?;
    let mut board = myopic_brain::pos::start();
    moves.iter().for_each(|mv| {
        board.evolve(mv);
    });
    Ok((board, moves.len() as u32))
}
