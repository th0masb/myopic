#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use crate::base::bitboard::BitBoard;
use crate::base::dir;
use crate::base::square::Square::*;
use crate::base::square::Square;

mod base;
mod pieces;
mod board;
mod eval;
mod search;

#[derive(Copy, Clone)]
enum Test {
    X, Y, Z
}

fn main() {
    let _dirs = vec!(&dir::N);
    let board = D2 | H3;
    let board2 = A3 | G7;
    println!("{}", board | board2);
    println!("{}", board | F3);
    println!("{}", F3 | board);
    println!("{}", G1 | A8);
    println!("{}", G1 > H1);
    let bitboard: BitBoard = vec!(A1, G5).into_iter().collect();
    println!("{}", bitboard);
    println!("{:?}", Square::E4);
    let x = format!("{:?}", Square::E4);
    println!("{}", x);
    let string = String::from("hello");
    println!("{}", string.contains("e"))
//    let squares = base.square:;:H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", base.square::H1);

//    let x = pieces::pawns::BLACK_CONTROL.clone().into_iter().map(|x| x.0).collect::<Vec<_>>();
//    println!("{:?}", x);
}

