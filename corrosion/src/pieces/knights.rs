use super::{BlackKnight, Piece, WhiteKnight};
use crate::bitboard::BitBoard;
use crate::dir::*;
use crate::side::Side;
use crate::square::{constants::SQUARES, Square};

fn compute_empty_board_control() -> Vec<BitBoard> {
    let search_dirs = vec![NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW];
    SQUARES
        .iter()
        .map(|&sq| sq.search_one(&search_dirs))
        .collect()
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

#[cfg(test)]
mod white_test {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_control() {
        let (wn, zero) = (WhiteKnight, BitBoard::EMPTY);
        assert_eq!(D1 | C2 | C4 | D5 | F5 | G4 | G2 | F1, wn.controlset(E3, zero, zero));
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
