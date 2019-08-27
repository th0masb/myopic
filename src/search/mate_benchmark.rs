use crate::board::{BoardImpl, Move};
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};

const DATA_PATH: &'static str = r"/home/t/git/myopic/data/formatted-three-puzzles";
const N_CASES: usize = 459;
const DEPTH: usize = 4;

#[test]
#[ignore]
fn mate_benchmark() {
    let cases = load_cases();
    assert_eq!(cases.len(), N_CASES)
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
        dest.push(TestCase{board, expected_move});
        if dest.len() == N_CASES {
            break;
        }
    }
    dest
}

struct TestCase {
    board: BoardImpl,
    expected_move: Move,
}
