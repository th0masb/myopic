use myopic_brain::{parse, EvalBoardImpl, MutBoard, MutBoardImpl};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

pub struct ThinkingTimeParams {
    pub expected_half_move_count: u32,
    pub half_moves_played: u32,
    pub initial: Duration,
    pub increment: Duration,
}

pub fn compute_thinking_time(params: ThinkingTimeParams) -> Duration {
    // Lets say we predict a chess game will last n moves before the game. Lets say the
    // time limit for the whole game is T seconds. Then for the first n / 2 moves allocate
    // T / n seconds. For the next n / 2 moves allocate T / 2n seconds and so on.
    //
    // So for move m, define i = m // (n / 2) and we allocate t_m = T / n * 2^i seconds
    //
    // This can be modified to make the decrease in thinking time less sharp
    let i = params.half_moves_played / (params.expected_half_move_count / 2);
    let n = params.expected_half_move_count;
    params.increment + (params.initial / (2u32.pow(i) * n))
}

#[cfg(test)]
mod thinking_time_test {
    use super::{compute_thinking_time, ThinkingTimeParams};
    use std::time::Duration;

    #[test]
    fn test_first_move_of_first_period() {
        let params = ThinkingTimeParams {
            expected_half_move_count: 60,
            half_moves_played: 0,
            increment: Duration::from_secs(2),
            initial: Duration::from_secs(600),
        };
        assert_eq!(Duration::from_secs(12), compute_thinking_time(params))
    }

    #[test]
    fn test_first_move_of_second_period() {
        let params = ThinkingTimeParams {
            expected_half_move_count: 60,
            half_moves_played: 30,
            increment: Duration::from_secs(2),
            initial: Duration::from_secs(600),
        };
        assert_eq!(Duration::from_secs(7), compute_thinking_time(params))
    }

    #[test]
    fn test_first_move_of_third_period() {
        let params = ThinkingTimeParams {
            expected_half_move_count: 60,
            half_moves_played: 60,
            increment: Duration::from_secs(2),
            initial: Duration::from_secs(600),
        };
        assert_eq!(Duration::from_millis(4500), compute_thinking_time(params))
    }

    #[test]
    fn test_first_move_of_fourth_period() {
        let params = ThinkingTimeParams {
            expected_half_move_count: 60,
            half_moves_played: 90,
            increment: Duration::from_secs(2),
            initial: Duration::from_secs(600),
        };
        assert_eq!(Duration::from_millis(3250), compute_thinking_time(params))
    }
}

pub fn get_game_state(moves: &String) -> Result<(EvalBoardImpl<MutBoardImpl>, u32), String> {
    let moves = parse::uci(moves)?;
    let mut board = myopic_brain::start();
    moves.iter().for_each(|mv| {
        board.evolve(mv);
    });
    Ok((board, moves.len() as u32))
}
