use crate::side::{ Side, Side::White, Side ::Black};
use crate::dir::{Dir, N, S};
use crate::square::Square;
use crate::square::constants::{A1, H3, A6, H8};
use crate::bitboard::BitBoard;

fn compute_empty_board_moves(side: Side) -> Vec<BitBoard> {
    let dir = match side {White => N, _ => S};
    let on_first_rank = |sq: Square| {
        let (lb, ub) = match side {White => (A1, H3), _ => (A6, H8)};
        lb < sq && sq < ub
    };
    unimplemented!()
}

pub struct WhitePawn;
pub struct BlackPawn;


