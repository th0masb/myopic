use crate::bitboard::{BitBoard, simple::*};
use crate::dir::{Dir, N, S};
use crate::pieces::Piece;
use crate::side::{Side, Side::Black, Side::White};
use crate::square::constants::{A1, A6, H3, H8};
use crate::square::Square;

fn compute_empty_board_moves(side: Side) -> Vec<BitBoard> {
    let dir = match side {
        White => N,
        _ => S,
    };
    let on_first_rank = |sq: Square| {
        let (lb, ub) = match side {
            White => (A1, H3),
            _ => (A6, H8),
        };
        lb < sq && sq < ub
    };
    unimplemented!()
}

pub struct WhitePawn;
impl Piece for WhitePawn {
    fn control_set(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let (x, left, right) = (loc.as_set() - RANKS[0], FILES[7], FILES[0]);


        unimplemented!()
    }
}

pub struct BlackPawn;
