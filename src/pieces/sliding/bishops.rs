use super::{
    bishop_dirs, compute_control, compute_magic_index, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_SHIFTS,
};
use crate::bitboard::BitBoard;

fn compute_move_database() -> Vec<Vec<BitBoard>> {
    let dest = Vec::with_capacity(64);
    dest
}

struct WhiteBishop;
