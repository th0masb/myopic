//#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod base;
mod pieces;
mod board;
mod eval;
mod search;
mod pgn;

#[derive(Copy, Clone)]
enum Test {
    X, Y, Z
}

fn main() {
//    //println!("{:?}", 1u64.trailing_zeros())
//    let fen = "r4rk1/5ppp/8/1Bn1p3/Q7/8/5PPP/1R3RK1 w KQkq - 5 24";
//    let mut board = eval::new_board(fen).unwrap();
//    let best_moves = search::best_move(&mut board, 4);
//    for mv in best_moves {
//        println!("{:?}", mv);
//    }
    let re = regex::Regex::new(r"[abc]{2}");
    println!("{}", re.unwrap().as_str());
}

