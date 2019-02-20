use crate::bitboard::BitBoard;
use crate::dir;
use crate::square::constants::*;
use crate::square::Square;

pub fn create_files() -> Vec<BitBoard> {
    (H1.search(dir::E) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::N) | sq)
        .collect()
}

pub fn create_ranks() -> Vec<BitBoard> {
    (H1.search(dir::N) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::E) | sq)
        .collect()
}


#[derive(Copy, Clone, PartialEq, Eq)]
struct WhitePawn;



#[derive(Copy, Clone, PartialEq, Eq)]
struct BlackPawn;

trait Indexable {
   fn index(&self) -> u8;
}

const test: [&dyn Indexable; 0] = [];

impl WhitePawn {
    fn index(&self) -> u8 {
        0
    }
}

fn x() -> WhitePawn {
    let wp = WhitePawn;
    let other_ap = WhitePawn;
    //let b = wp == BlackPawn;
    let x = wp.index();
    wp
}

//pub fn create