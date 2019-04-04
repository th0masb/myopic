use super::{bishop_dirs, compute_bishop_index, compute_control, compute_powerset, BISHOP_MASKS};
use crate::bitboard::BitBoard;
use crate::square::constants::SQUARES;
use crate::pieces::Piece;
use crate::pieces::WhiteBishop;
use crate::square::Square;
use crate::pieces::BlackBishop;

use std::iter::repeat;

/// Piece trait implementation for the white bishop singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for WhiteBishop {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_bishop_index(location, white | black)]
    }

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) - white
    }

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) & black
    }
}

/// Piece trait implementation for the black bishop singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for BlackBishop {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_bishop_index(location, white | black)]
    }

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) - black
    }

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) & white
    }
}

/// Implementation and tests for the static magic move database.
///
type Moves = Vec<Vec<BitBoard>>;

lazy_static! {
    static ref MOVES: Moves = compute_move_database();
}

fn compute_move_database() -> Moves {
    let mut dest = Vec::with_capacity(64);
    let dirs = bishop_dirs();
    for (&sq, &mask) in izip!(SQUARES.iter(), BISHOP_MASKS.iter()) {
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(1 << mask.size()).collect();
        for occ_var in compute_powerset(&mask.into_iter().collect()) {
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
mod move_test {
    use super::{compute_bishop_index, compute_move_database, BitBoard, Moves};
    use crate::square::constants::*;

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(moves);
    }

    fn test_case_one(moves: Moves) {
        let (sq, occ) = (D3, E2 | B1 | F5 | H7 | D4);
        let expected = E2 | C2 | B1 | C4 | B5 | A6 | E4 | F5;
        assert_eq!(expected, moves[sq.i as usize][compute_bishop_index(sq, occ)])
    }
}
