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
    let fen = "5r2/pp2R3/1q1p3Q/2pP1b2/2Pkrp2/3B4/PPK2PP1/R7 w - - 1 0";
    let fen = "5r2/pp2R3/1q1p4/2pP1b2/2Pkrp1Q/3B4/PPK2PP1/R7 b - - 2 1";
    let fen = "5r2/pp2R3/1q1p4/2pP1b2/2Pk1p1Q/3B4/PPK2PP1/R3r3 w - - 3 2";
    // Performing enpassant opens discovered attack on queen!! Probably easiest to just make the
    // switch to a psuedo legal move computation mode in the search...
    // TODO encapsulate this bug into a test...

    //let fen = "5r2/pp2R3/1q1p4/2pP1b2/2Pk1pPQ/3B4/PPK2P2/R3r3 b - - 0 2";
    let mut board = eval::new_board(fen).unwrap();
    let neg = search::best_move(&mut board, 2);
    println!("{:?}", neg);
}
