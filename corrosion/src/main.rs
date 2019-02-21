#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
//extern crate bitwise;

use crate::bitboard::{BitBoard, simple::*};

use crate::square::constants::*;
use crate::pieces::{Piece, pawns::WhitePawn};

mod square;
mod bitboard;
mod dir;
mod side;
mod pieces;

fn main() {
    let _dirs = vec!(&dir::N);
    let board = BitBoard::new(&[D2, H3]);
    let board2 = BitBoard::new(&[A3, G7]);
    println!("{}", board | board2);
    println!("{}", board | F3);
    println!("{}", F3 | board);
    println!("{}", G1 | A8);
    println!("{}", G1 > H1);
    let bitboard: BitBoard = vec!(A1, G5).into_iter().collect();
    println!("{}", bitboard);
//    let squares = square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", square::H1);
    println!("{}", RANKS[1]);
    println!("{}", WhitePawn.control_set(H3, BitBoard::EMPTY, BitBoard::EMPTY));
}

