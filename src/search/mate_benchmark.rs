use crate::board::{BoardImpl, Move};
use crate::eval::{SimpleEvalBoard, WIN_VALUE};
use crate::search::SearchCommand;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

const DATA_PATH: &'static str = r"/home/t/git/myopic/data/formatted-three-puzzles";
const MAX_CASES: usize = 500;
const DEPTH: usize = 4;

#[rustfmt::skip]
///
/// cargo test --release mate_benchmark -- --ignored --nocapture
///
/// Errors at case 330:
/// -- quiescent search on "8/7k/1p6/5p1p/PP2bb2/6QP/6PK/5q2 b - - 0 3" doesn't recognize the mate
///    because see thinks it's a bad exchange. I think I can live with it for now.
///
/// RESULTS:
/// ------------------------------------------------------------------------------------------------
/// Date     | Depth   | Cases | Errors | Time (ms)          | Notes
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4(8)(2) | 3     |        | 24,537             |
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4(8)(2) | 100   | 10     | 1,282,849          |
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4(8)(2) | 100   | 5      | 1,272,875          | Fixed bug with static exchange eval
/// ------------------------------------------------------------------------------------------------
/// 28/08/19 | 4(8)(2) | 100   | 4      | 1,375,979          | Another bug with see
/// ------------------------------------------------------------------------------------------------
/// 30/08/19 | 4(8)(2) | 100   | 3      | 1,455,897          | Fixed issue with check by discovery
/// ------------------------------------------------------------------------------------------------
/// 30/08/19 | 4(8)(1) | 100   | 3      | 1,315,718          |
/// ------------------------------------------------------------------------------------------------
/// 01/09/19 | 4(8)(2) | 100   | 1      | 1,521,827          | Fixed bug with termination status
///          |         |       |        |                    | computation, unsure why performance -
/// ------------------------------------------------------------------------------------------------
/// 02/09/19 | 4(8)(2) | 458   | 6      | 5,642,934          | First full run
/// ------------------------------------------------------------------------------------------------
/// 03/09/19 | 4(8)(2) | 458   | 1      | 5,891,925          | Second full run, fixed bugs
/// ------------------------------------------------------------------------------------------------
/// 10/09/19 | 4(8)(2) | 457   | 1      | 6,155,301          | Tested new interruptable search,
///          |         |       |        |                    | pleasingly fast considering it uses
///          |         |       |        |                    | naive iterative deepening. Adjusted
///          |         |       |        |                    | the timing to be more precise though
///          |         |       |        |                    | So I think that played a part.
/// ------------------------------------------------------------------------------------------------
/// 11/09/19 | 4(8)(2) | 457   | 1      | 6,066,524          | Refactored the search again, runs on
///          |         |       |        |                    | separate thread. Further tightened
///          |         |       |        |                    | the timing which will explain the
///          |         |       |        |                    | performance increase.
/// ------------------------------------------------------------------------------------------------
///
///
#[test]
#[ignore]
fn mate_benchmark() {
    let cases = load_cases();
    let mut search_duration = Duration::from_secs(0);
    let (mut err_count, mut case_count) = (0, 0);
    let (search_input_tx, search_output_rx) = crate::search::init();
    search_input_tx.send(SearchCommand::Time {max_depth: DEPTH, max_time: Duration::from_secs(10000)});
    for (i, mut test_case) in cases.into_iter().enumerate() {
        if i % 5 == 0 {
            println!("{}", i);
        }
        search_input_tx.send(SearchCommand::Root(test_case.board));
        search_input_tx.send(SearchCommand::Go);
        match search_output_rx.recv() {
            Err(_) => panic!(),
            Ok(result) => match result {
                Err(_) => panic!(),
                Ok(details) => {
                    search_duration += details.time;
                    if test_case.expected_move != details.best_move || WIN_VALUE != details.eval {
                        err_count += 1;
                        println!("Error at index {}", i);
                    }
                }
            }
        }
        case_count += 1;
    }
    search_input_tx.send(SearchCommand::Close);
    println!(
        "Depth: {}, Cases: {}, Errors: {}, Time: {}",
        DEPTH, case_count, err_count, search_duration.as_millis()
    );
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
        match crate::board::from_fen(fen) {
            Err(_) => {
                println!("Error with position parsing: {}", line_clone);
                continue;
            },
            Ok(board) => match crate::pgn::parse_pgn(&board, pgn) {
                Err(_) => {
                    println!("Error with move parsing: {}", line_clone);
                    continue;
                },
                Ok(moves) => {
                    let expected_move = moves.first().unwrap().to_owned();
                    dest.push(TestCase { board: SimpleEvalBoard::new(board), expected_move });
                    if dest.len() == MAX_CASES {
                        break;
                    }
                }
            }
        }
    }
    dest
}

struct TestCase {
    board: SimpleEvalBoard<BoardImpl>,
    expected_move: Move,
}
