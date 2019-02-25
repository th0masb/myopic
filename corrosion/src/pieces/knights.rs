use crate::side::Side;
use crate::bitboard::BitBoard;
use crate::square::{Square, constants::SQUARES};
use crate::dir::*;
use super::{Piece, WhiteKnight, BlackKnight};

fn compute_empty_board_control() -> Vec<BitBoard> {
    let search_dirs = vec![NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW];
    SQUARES.iter().map(|&sq| sq.search_one(&search_dirs)).collect()
}

lazy_static! {
    static ref CONTROL: Vec<BitBoard> = compute_empty_board_control();
}

impl Piece for WhiteKnight {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
       CONTROL[location.i as usize]
    }

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] - white
    }

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] & black
    }
}

impl Piece for BlackKnight {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize]
    }

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] - black
    }

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] & white
    }
}
