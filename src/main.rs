#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod base;
mod board;
mod eval;
mod parse;
mod pieces;
mod search;
mod uci;

fn main() -> () {
    uci::uci_main()
//    // Start depth 5
//    let fen = board::START_FEN;
//    let fen = "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1";
//    let fen = "rnbqkb1r/pppppppp/5n2/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 2 2";
//    let fen = "rnbqkb1r/pppppppp/5n2/8/8/2N4N/PPPPPPPP/R1BQKB1R b KQkq - 3 2";
//    let fen = "r1bqkb1r/pppppppp/2n2n2/8/8/2N4N/PPPPPPPP/R1BQKB1R w KQkq - 4 3";
//    // It thinks this is the best position, evals as 37 because it gets
//    // the knights in a better position
//    let fen = "r1bqkb1r/pppppppp/2n2n2/8/5N2/2N5/PPPPPPPP/R1BQKB1R b KQkq - 5 3";
//
//    // Lets try a different line
//    let fen = board::START_FEN;
//    let fen = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1";
//    let fen = "rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 1 2";
//    let board = eval::new_board(fen).unwrap();
//    let (input_tx, output_rx) = search::init::<SimpleEvalBoard<BoardImpl>>();
//    input_tx.send(SearchCommand::Depth(3)).unwrap();
//    input_tx.send(SearchCommand::Root(board)).unwrap();
//    input_tx.send(SearchCommand::GoOnce).unwrap();
//    match output_rx.recv() {
//        Err(_) => panic!(),
//        Ok(result) => match result {
//            Err(_) => panic!(),
//            Ok(details) => println!("{:?}", details),
//        }
//    }
}

