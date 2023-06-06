use std::iter::repeat;

use itertools::izip;
use lazy_static::lazy_static;

use crate::bitboard::BitBoard;
use crate::square::Square;

use super::{bishop_dirs, BISHOP_MASKS, compute_bishop_index, compute_control, compute_powerset};

pub fn control(loc: Square, occupied: BitBoard) -> BitBoard {
    MOVES[loc as usize][compute_bishop_index(loc, occupied)]
}

/// Implementation and tests for the static magic move database.
type Moves = Vec<Vec<BitBoard>>;

lazy_static! {
    static ref MOVES: Moves = compute_move_database();
}

fn compute_move_database() -> Moves {
    let mut dest = Vec::with_capacity(64);
    let dirs = bishop_dirs();
    for (sq, bb) in izip!(Square::iter(), BISHOP_MASKS.iter().map(|&m| BitBoard(m))) {
        let dest_size = 1 << bb.size();
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(dest_size).collect();
        for occ_var in compute_powerset(&bb.into_iter().collect()) {
            let index = compute_bishop_index(sq, occ_var);
            if sq_dest[index] == BitBoard::ALL {
                sq_dest[index] = compute_control(sq, occ_var, &dirs);
            }
        }
        dest.push(sq_dest);
    }
    dest
}

#[cfg(test)]
mod test {
    use crate::square::Square::*;

    use super::{compute_bishop_index, compute_move_database, Moves};

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(&moves);
        test_case_two(&moves);
    }

    fn test_case_one(moves: &Moves) {
        let (sq, occ) = (D3, E2 | B1 | F5 | H7 | D4);
        let expected = E2 | C2 | B1 | C4 | B5 | A6 | E4 | F5;
        assert_eq!(expected, moves[sq as usize][compute_bishop_index(sq, occ)])
    }

    fn test_case_two(moves: &Moves) {
        let (sq, occ) = (H5, D1 | E2 | G6 | C5 | F6 | A1 | A4 | D2);
        let expected = G4 | F3 | E2 | G6;
        assert_eq!(expected, moves[sq as usize][compute_bishop_index(sq, occ)])
    }
}
