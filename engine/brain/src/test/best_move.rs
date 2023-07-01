use myopic_board::{Board, Reflectable};

use crate::search::SearchParameters;
use crate::{eval, Evaluator, UciMove};

const DEPTH: usize = 4;
const TABLE_SIZE: usize = 10_000;

enum Setup {
    Fen(&'static str),
    Pgn(&'static str),
}

fn test(setup: Setup, expected_move_pool: Vec<UciMove>, is_won: bool) {
    match setup {
        Setup::Fen(fen_string) => {
            let base_board = fen_string.parse::<Board>().unwrap();
            let ref_board = Evaluator::from(base_board.reflect());
            let board = Evaluator::from(base_board);
            let ref_move_pool = expected_move_pool.reflect();
            test_impl(board, expected_move_pool, is_won);
            test_impl(ref_board, ref_move_pool, is_won);
        }
        Setup::Pgn(pgn_string) => {
            let mut board = Evaluator::default();
            board.play_pgn(pgn_string).unwrap();
            test_impl(board, expected_move_pool, is_won)
        }
    }
}

fn test_impl(board: Evaluator, expected_move_pool: Vec<UciMove>, is_won: bool) {
    match crate::search(board, SearchParameters { terminator: DEPTH, table_size: TABLE_SIZE }) {
        Err(message) => panic!("{}", message),
        Ok(outcome) => {
            assert!(
                expected_move_pool
                    .contains(&UciMove::new(outcome.best_move.uci_format().as_str()).unwrap()),
                "{}",
                serde_json::to_string(&outcome).unwrap()
            );
            if is_won {
                assert_eq!(eval::WIN_VALUE, outcome.relative_eval);
            }
        }
    }
}

#[test]
fn queen_escape_attack() {
    test(
        Setup::Fen("r4rk1/5ppp/8/1Bn1p3/Q7/8/5PPP/1R3RK1 w Qq - 5 27"),
        vec![
            UciMove::new("a4b4").unwrap(),
            UciMove::new("a4c4").unwrap(),
            UciMove::new("a4g4").unwrap(),
            UciMove::new("a4h4").unwrap(),
            UciMove::new("a4c2").unwrap(),
            UciMove::new("a4d1").unwrap(),
        ],
        false,
    )
}

#[test]
fn mate_0() {
    test(
        Setup::Fen("r2r2k1/5ppp/1N2p3/1n6/3Q4/2B5/5PPP/1R3RK1 w Qq - 4 21"),
        vec![UciMove::new("d4g7").unwrap()],
        true,
    )
}

#[test]
fn mate_1() {
    test(
        Setup::Fen("8/8/8/4Q3/8/6R1/2n1pkBK/8 w - - 0 1"),
        vec![UciMove::new("g3d3").unwrap()],
        true,
    )
}

#[test]
fn mate_2() {
    test(
        Setup::Fen("8/7B/5Q2/6p1/6k1/8/5K2/8 w - - 0 1"),
        vec![UciMove::new("f6h8").unwrap(), UciMove::new("f6f3").unwrap()],
        true,
    )
}

#[test]
fn mate_3() {
    test(
        Setup::Fen("3qr2k/1b1p2pp/7N/3Q2b1/4P3/8/5PP1/6K1 w - - 0 1"),
        vec![UciMove::new("d5g8").unwrap()],
        true,
    )
}

// Mate in 4 moves TODO probably better in benchmark.
#[ignore]
#[test]
fn mate_4() {
    test(
        Setup::Fen("r1k2b1r/pp4pp/2p1n3/3NQ1B1/6q1/8/PPP2P1P/2KR4 w - - 4 20"),
        vec![UciMove::new("e5c7").unwrap()],
        true,
    )
}

#[test]
fn mate_5() {
    test(
        Setup::Fen("r1b1k1nr/p2p1ppp/n2B4/1p1NPN1P/6P1/3P1Q2/P1P1K3/q5b1 w - - 0 30"),
        vec![UciMove::new("f5g7").unwrap()],
        true,
    )
}

/// A funny one which currently depends on move ordering, at depth 3 the
/// best move has the same evaluation as another inferior move.
#[ignore]
#[test]
fn tactic_1() {
    test(
        Setup::Fen("1r3k2/2R5/1p2p2p/1Q1pPp1q/1P1P2p1/2P1P1P1/6KP/8 b - - 2 31"),
        vec![UciMove::new("b8a8").unwrap()],
        false,
    )
}

/// This fails at depth 3 but passes at depth 4, should be moved to a
/// benchmark maybe
#[ignore]
#[test]
fn tactic_2() {
    test(
        Setup::Fen("r5k1/pb4pp/1pn1pq2/5B2/2Pr4/B7/PP3RPP/R4QK1 b - - 0 23"),
        vec![UciMove::new("e6f5").unwrap()],
        false,
    )
}

#[test]
fn prefer_castling() {
    test(
        Setup::Pgn("1. e4 e5 2. f4 exf4 3. Nf3 g5 4. Nc3 Nc6 5. g3 g4 6. Nh4 Nd4 7. Bc4 Be7"),
        vec![UciMove::new("e1g1").unwrap()],
        false,
    )
}

#[test]
fn win_material() {
    test(
        Setup::Pgn("1. d4 d5 2. e3 Nf6 3. c4 c6 4. Nc3 e6 5. Bd3 dxc4 6. Bxc4 b5 7. Be2 Bd6 8. e4 b4 9. e5 bxc3 10. exf6 O-O 11. fxg7 cxb2"),
        vec![UciMove::new("g7f8q").unwrap(), UciMove::new("g7f8r").unwrap()],
        false,
    )
}
