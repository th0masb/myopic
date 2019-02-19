#[macro_use]
extern crate itertools;
//extern crate bitwise;

mod square;
mod bitboard;
mod dir;

use crate::square::*;
use crate::bitboard::BitBoard;

fn main() {
    let dirs = vec!(&dir::N);
    let board = BitBoard::new(&[D2, H3]);
    let board2 = BitBoard::new(&[A3, G7]);
    println!("{}", board | board2);
    println!("{}", board | F3);
    println!("{}", F3 | board);
    println!("{}", G1 | A8);
//    let squares = square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", square::H1);
}

fn some_func(mut input_ref: &Square) {
    input_ref = &square::H1;
}

fn first2(square: Square) {
    println!("{:?}", square);
}

fn first(mut square: Square) {
    square.i = 5;
    println!("{:?}", square);
}

fn second(square: &Square) {
    println!("{:?}", square);
}

fn third(square: &mut Square) {
    square.i = 5;
    println!("{:?}", square);
}
