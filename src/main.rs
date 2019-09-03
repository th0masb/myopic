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
use std::io::{self, Read};
use std::thread;

fn main() {
    //println!("{:?}", 1u64.trailing_zeros())
    let stdin = io::stdin();
    let mut dest = String::new();
    let handle = thread::spawn(move || {
        stdin.read_line(&mut dest);
        println!("{}", dest);

    });
    handle.join().unwrap();
   // let fen = "rn3k2/pR2b3/4p1Q1/2q1N2P/3R2P1/3K4/P3Br2/8 w - - 1 0";
   // let fen = "rR3k2/p3b3/4p1Q1/2q1N2P/3R2P1/3K4/P3Br2/8 b - - 0 1";
   // // No moves computed here!!
   // let mut board = eval::new_board(fen).unwrap();
   // let neg = search::best_move(&mut board, 3);
   // //let neg = search::negamax(&mut board, -eval::INFTY, eval::INFTY, 4);
   // println!("{:?}", neg);
}
