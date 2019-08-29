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
    let fen = "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P1p1P1/1P4PK/5R2 w - - 1 0";
    let fen = "2b3rk/1q3p1p/p1p1pPpQ/4N3/2pP4/2P3P1/1P2p1PK/7R w - - 0 2";
    let fen = "2b3rk/1q3p1Q/p1p1pPp1/4N3/2pP4/2P3P1/1P2p1PK/7R b - - 0 2";
    let mut board = eval::new_board(fen).unwrap();
    let neg = search::best_move(&mut board, 1);
    println!("{:?}", neg);
}
