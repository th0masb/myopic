use crate::board::{BoardImpl, Move};
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use crate::eval::SimpleEvalBoard;

const DATA_PATH: &'static str = r"/home/t/git/myopic/data/formatted-three-puzzles";
const N_CASES: usize = 100;
const DEPTH: usize = 4;

///
/// cargo test --release mate_benchmark -- --ignored --nocapture
///
/// RESULTS:
/// ------------------------------------------------------------------------------------------------
/// Date     | Depth | Cases | Errors | Time (ms)          | Notes
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4     | 3     |        | 24,537             |
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4     | 100   | 10     | 1,282,849          |
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4     | 100   | 4      | 1,272,875          | Fixed bug with static exchange eval
/// ------------------------------------------------------------------------------------------------
///          |       |       |        |
///          |       |       |        |
///          |       |       |        |
///          |       |       |        |
///          |       |       |        |
/// ------------------------------------------------------------
#[test]
#[ignore]
fn mate_benchmark() {
    let cases = load_cases();
    let timer = Instant::now();
    let mut err_count = 0;
    for (i, mut test_case) in cases.into_iter().enumerate() {
        let actual_move = crate::search::best_move(&mut test_case.board, DEPTH).unwrap().0;
        if test_case.expected_move != actual_move {
            err_count += 1;
            println!("Error at index {}", i);
        }
    }
    let time = timer.elapsed().as_millis();
    println!("Depth: {}, Cases: {}, Errors: {}, Time: {}", DEPTH, N_CASES, err_count, time);
}

fn load_cases() -> Vec<TestCase> {
    lazy_static! {
        static ref SEP: Regex = Regex::new(r"[$]{4}").unwrap();
    }
    let file = fs::File::open(DATA_PATH).unwrap();
    let reader = BufReader::new(file);
    let mut dest = Vec::new();
    for line in reader.lines() {
        let line_clone = String::from(&line.unwrap());
        let split: Vec<String> = SEP.split(&line_clone).map(String::from).collect();
        if split.len() != 2 {
            println!("Error with separation: {}", line_clone);
            continue;
        }
        let (fen, pgn) = (split.first().unwrap(), split.last().unwrap());
        let board_res = crate::board::from_fen(fen);
        if board_res.is_err() {
            println!("Error with position parsing: {}", line_clone);
            continue;
        }
        let board = board_res.unwrap();
        let moves_res = crate::pgn::parse_pgn(&board, pgn);
        if moves_res.is_err() {
            println!("Error with move parsing: {}", line_clone);
            continue;
        }
        let expected_move = moves_res.unwrap().first().unwrap().to_owned();
        dest.push(TestCase{ board: SimpleEvalBoard::new(board), expected_move});
        if dest.len() == N_CASES {
            break;
        }
    }
    dest
}

struct TestCase {
    board: SimpleEvalBoard<BoardImpl>,
    expected_move: Move,
}
