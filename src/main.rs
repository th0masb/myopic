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
    let fen = "1r2r2k/5p1p/ppbp1P1B/5PR1/2P3Rp/3n4/PP1N3P/6K1 w - - 1 0";
    //let fen = "1r2r2k/5p1p/ppbp1P1B/5PR1/2P3Rp/3n4/PP5P/5NK1 b - - 2 1";
    //let fen = "1r2r2k/5p1p/ppbp1P1B/5PR1/2P3R1/3n3p/PP5P/5NK1 w - - 0 2";
    //let fen = "1r2r1Rk/5p1p/ppbp1P1B/5P2/2P3R1/3n3p/PP5P/5NK1 b - - 1 2";
    //let fen = "1r4rk/5p1p/ppbp1P1B/5P2/2P3R1/3n3p/PP5P/5NK1 w - - 0 3";
    // No moves computed here!!
    let mut board = eval::new_board(fen).unwrap();
    let neg = search::best_move(&mut board, 4);
    //let neg = search::negamax(&mut board, -eval::INFTY, eval::INFTY, 4);
    println!("{:?}", neg);
}
