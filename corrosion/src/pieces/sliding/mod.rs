use crate::bitboard::simple::{FILES, RANKS};
use crate::bitboard::BitBoard;
use crate::dir::Dir;
use crate::dir::{E, N, S, W};
use crate::dir::{NE, NW, SE, SW};
use crate::square::constants::SQUARES;
use crate::square::Square;
use std::vec::IntoIter;

fn board_border() -> BitBoard {
    RANKS[0] | RANKS[7] | FILES[0] | FILES[7]
}

fn compute_bishop_masks() -> Vec<BitBoard> {
    let (border, dirs) = (board_border(), vec![NE, SE, SW, NW]);
    SQUARES
        .iter()
        .map(|&sq| sq.search_all(&dirs) - border)
        .collect()
}

fn search_remove_last(loc: Square, dir: Dir) -> BitBoard {
    let mut res = loc.search_vec(dir);
    if res.len() > 0 {
        res.remove(res.len() - 1);
    }
    res.into_iter().collect()
}

fn compute_rook_masks() -> Vec<BitBoard> {
    let dirs = vec![N, E, S, W];
    SQUARES
        .iter()
        .map(|&sq| {
            dirs.iter()
                .map(|&dir| search_remove_last(sq, dir))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod mask_tests {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_bishop_masks() {
        let (bmasks, rmasks) = (compute_bishop_masks(), compute_rook_masks());
        assert_eq!(C7 | C5 | D4 | E3 | F2, bmasks[B6.i as usize]);
        assert_eq!(A2 | A3 | A5 | A6 | A7 | B4 | C4 | D4 | E4 | F4 | G4, rmasks[A4.i as usize]);
    }
}

lazy_static! {
    static ref BISHOP_MASKS: Vec<BitBoard> = compute_bishop_masks();
    static ref ROOK_MASKS: Vec<BitBoard> = compute_rook_masks();
}


