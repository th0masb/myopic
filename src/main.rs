//#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod base;
mod board;
mod eval;
mod pgn;
mod pieces;
mod search;

use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
enum Test {
    X,
    Y,
    Z,
}

fn main() {
        //println!("{:?}", 1u64.trailing_zeros())
    let fen = "r1b1k1nr/p2p1ppp/n2B4/1p1NPN1P/6P1/3P1Q2/P1P1K3/q5b1 w - - 0 30";
    //let fen = "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1";
    //let fen = "r1bk2nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 w - - 1 2";
    //let fen = "r1bk2nr/p2p1pNp/n2B1Q2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 2 2";
    //let fen = "r1bk3r/p2pnpNp/n2B1Q2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w - - 3 3";
    let mut board = eval::new_board(fen).unwrap();
        let best_move = search::best_move(&mut board, 4).unwrap();
        println!("{:?}", best_move);
}
