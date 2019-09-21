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
//    let fen = "Q2qkb1r/p2npp1p/6p1/1B2p3/4PB2/2N5/PPP2PPP/R2bK2R b KQk - 0 11";
//
//    let board = eval::new_board(fen).unwrap();
//    let (input_tx, output_rx) = search::init::<SimpleEvalBoard<BoardImpl>>();
//    input_tx.send(SearchCommand::Time(3_000)).unwrap();
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

