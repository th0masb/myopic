//#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod base;
mod eval;
mod pgn;
mod pieces;
mod search;
mod board;

use std::fs;
use std::path::Path;

fn main() {
    //println!("{:?}", 1u64.trailing_zeros())
    let fen = "4R3/1p4rk/6p1/2pQBpP1/p1P1pP2/Pq6/1P6/K7 w - - 1 0";
    //let fen = "4R3/1p1Q2rk/6p1/2p1BpP1/p1P1pP2/Pq6/1P6/K7 b - - 1 1";
    //let fen = "4R3/1p1Q2rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2q4 w - - 2 2";
    //let fen = "4R3/1p4rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2Q4 b - - 0 2";
    let mut board = eval::new_board(fen).unwrap();
    let neg = search::best_move(&mut board, 4);
    println!("{:?}", neg);
}
