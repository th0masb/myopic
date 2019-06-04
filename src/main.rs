#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::dir;
use crate::base::square::Square::*;

mod base;
mod pieces;
mod board;
mod utils;
mod search;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jenjinn3tmp!");
}

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
    println!("{}", Test::Y as usize)

//    let squares = base.square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", base.square::H1);

//    let x = pieces::pawns::BLACK_CONTROL.clone().into_iter().map(|x| x.0).collect::<Vec<_>>();
//    println!("{:?}", x);
}

