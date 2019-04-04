use super::{bishop_dirs, compute_bishop_index, compute_control, compute_powerset, BISHOP_MASKS};
use crate::bitboard::BitBoard;
use crate::square::constants::SQUARES;
use std::iter::repeat;

fn compute_move_database() -> Vec<Vec<BitBoard>> {
    let mut dest = Vec::with_capacity(64);
    let dirs = bishop_dirs();
    for (&sq, &mask) in izip!(SQUARES.iter(), BISHOP_MASKS.iter()) {
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(1 << mask.size()).collect();
        for occ_var in compute_powerset(&mask.into_iter().collect()) {
            let index = compute_bishop_index(occ_var, sq);
            if sq_dest[index] != BitBoard::ALL {
                sq_dest[index] = compute_control(sq, occ_var, &dirs);
            }
        }
        dest.push(sq_dest);
    }
    dest
}

pub struct WhiteBishop;
