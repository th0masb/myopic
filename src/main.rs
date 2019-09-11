#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use regex::{Match, Regex};

use crate::board::{BoardImpl, Move, Board, MoveComputeType};
use crate::board::Move::Standard;
use crate::uci::uci_main;

mod base;
mod board;
mod eval;
mod pgn;
mod pieces;
mod search;
mod uci;

fn main() -> () {
//    //uci_main()
//    let fen = "8/6bk/1p6/5pBp/1P2b3/6QP/P5PK/5q2 b - - 1 0";
//    let fen = "8/7k/1p6/4bpBp/1P2b3/6QP/P5PK/5q2 w - - 1 2";
////    let fen = "8/7k/1p6/4bp1p/1P2bB2/6QP/P5PK/5q2 b - - 2 2";
////    let fen = "8/7k/1p6/5p1p/1P2bb2/6QP/P5PK/5q2 w - - 0 3";
////    let fen = "8/7k/1p6/5p1p/PP2bb2/6QP/6PK/5q2 b - - 0 3";
//    let mut root = eval::new_board(fen).unwrap();
//    //let res = search::quiescent::search(&mut root, -eval::INFTY, eval::INFTY, -1);
//    let search_res = Search::depth_capped(root, 4).execute();
//    println!("{:?}", search_res);
}

