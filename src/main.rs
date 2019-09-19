#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use crate::uci::uci_main;
use regex::Regex;

mod base;
mod board;
mod eval;
mod pgn;
mod pieces;
mod search;
mod uci;

fn main() -> () {
    uci_main()
//    let fen = "rnb1k1nr/p2p1ppp/5q2/1pb2N1P/4PBP1/2NP1Q2/PPP5/R4KR1 w kq - 3 17";
//    let board = eval::new_board(fen).unwrap();
//    let (input_tx, output_rx) = search::init::<SimpleEvalBoard<BoardImpl>>();
//    input_tx.send(SearchCommand::Root(board)).unwrap();
//    input_tx.send(SearchCommand::GoOnce).unwrap();
//    thread::sleep(Duration::from_secs(10));
//    input_tx.send(SearchCommand::Stop).unwrap();
//    match output_rx.recv() {
//        Err(_) => panic!(),
//        Ok(result) => match result {
//            Err(_) => panic!(),
//            Ok(details) => println!("{:?}", details),
//        }
//    }

//    //uci_main()
//    let fen = "8/6bk/1p6/5pBp/1P2b3/6QP/P5PK/5q2 b - - 1 0";
//    let fen = "8/7k/1p6/4bpBp/1P2b3/6QP/P5PK/5q2 w - - 1 2";
////    let fen = "8/7k/1p6/4bp1p/1P2bB2/6QP/P5PK/5q2 b - - 2 2";
////    let fen = "8/7k/1p6/5p1p/1P2bb2/6QP/P5PK/5q2 w - - 0 3";
////    let fen = "8/7k/1p6/5p1p/PP2bb2/6QP/6PK/5q2 b - - 0 3";
//    let mut root = eval::new_board(fen).unwrap();
//    //let res = search::quiescent::search(&mut root, -eval::INFTY, eval::INFTY, -1);
//    let search_res = Search::depth_capped(root, 4).execute();
//    println!("{:?}", search_res);
}

