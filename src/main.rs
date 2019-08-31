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
    let fen = "rn4nr/pppq2bk/7p/5b1P/4NBQ1/3B4/PPP3P1/R3K2R w - - 1 0";
    let mut board = eval::new_board(fen).unwrap();
    let neg = search::best_move(&mut board, 4);
    println!("{:?}", neg);
}
