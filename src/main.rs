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
    //let fen = "r1n5/pp2q1kp/2ppr1p1/4p1Q1/8/2N4R/PPP3PP/5RK1 w - - 1 0";
    //let fen = "r1n5/pp2q1kp/2ppr1pQ/4p3/8/2N4R/PPP3PP/5RK1 b - - 2 1";
    //let fen = "r1n3k1/pp2q2p/2ppr1pQ/4p3/8/2N4R/PPP3PP/5RK1 w - - 3 2";
    //let fen = "r1n2Rk1/pp2q2p/2ppr1pQ/4p3/8/2N4R/PPP3PP/6K1 b - - 4 2";
    let fen = "r1n2qk1/pp5p/2ppr1pQ/4p3/8/2N4R/PPP3PP/6K1 w - - 0 3";
    let mut board = eval::new_board(fen).unwrap();
        let neg = search::negamax(&mut board, -eval::INFTY, eval::INFTY, 0);
        println!("{:?}", neg);
}
