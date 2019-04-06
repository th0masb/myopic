use super::{compute_control, compute_powerset, compute_rook_index, rook_dirs, ROOK_MASKS};
use crate::bitboard::BitBoard;
use crate::pieces::BlackRook;
use crate::pieces::Piece;
use crate::pieces::WhiteRook;
use crate::square::constants::SQUARES;
use crate::square::Square;

use std::iter::repeat;

/// Piece trait implementation for the white rook singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for WhiteRook {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_rook_index(location, white | black)]
    }

    fn moveset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) - white
    }

    fn attackset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(location, white, black) & black
    }
}

/// Piece trait implementation for the black rook singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for BlackRook {
    fn controlset(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_rook_index(location, white | black)]
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
    let dirs = rook_dirs();
    for (&sq, bb) in izip!(SQUARES.iter(), ROOK_MASKS.iter().map(|&m| BitBoard(m))) {
        let dest_size = 1 << bb.size();
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(dest_size).collect();
        for occ_var in compute_powerset(&bb.into_iter().collect()) {
            let index = compute_rook_index(sq, occ_var);
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
    use super::{compute_move_database, compute_rook_index, Moves};
    use crate::square::constants::*;

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(&moves);
        test_case_two(&moves);
    }

    fn test_case_one(moves: &Moves) {
        let (sq, occ) = (D3, E2 | B1 | F5 | H7 | D4);
        let expected = E2 | C2 | B1 | C4 | B5 | A6 | E4 | F5;
        assert_eq!(expected, moves[sq.i as usize][compute_rook_index(sq, occ)])
    }

    fn test_case_two(moves: &Moves) {
        let (sq, occ) = (H5, D1 | E2 | G6 | C5 | F6 | A1 | A4 | D2);
        let expected = G4 | F3 | E2 | G6;
        assert_eq!(expected, moves[sq.i as usize][compute_rook_index(sq, occ)])
    }
}
