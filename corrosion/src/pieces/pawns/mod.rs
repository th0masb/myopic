use crate::bitboard::{simple::*, BitBoard};
use crate::dir::{Dir, N, S};
use crate::pieces::{BlackPawn, Piece, WhitePawn};
use crate::side::{Side, Side::Black, Side::White};
use crate::square::constants::{A1, A6, H3, H8, SQUARES};
use crate::square::Square;

pub mod white;
pub mod black;

fn compute_all_empty_board_control(side: Side) -> Vec<BitBoard> {
    SQUARES
        .iter()
        .map(|&sq| compute_empty_board_control(side, sq))
        .collect()
}

fn compute_empty_board_control(side: Side, loc: Square) -> BitBoard {
    let (x, left, right) = (loc.as_set() - RANKS[0], FILES[7], FILES[0]);
    match side {
        White => ((x - left) << 9u8) | ((x - right) << 7u8),
        Black => ((x - left) >> 7u8) | ((x - right) >> 9u8),
    }
}

lazy_static! {
    static ref WHITE_CONTROL: Vec<BitBoard> = compute_all_empty_board_control(White);
    static ref BLACK_CONTROL: Vec<BitBoard> = compute_all_empty_board_control(Black);
}
