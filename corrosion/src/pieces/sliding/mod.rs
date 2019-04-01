use crate::bitboard::simple::{FILES, RANKS};
use crate::bitboard::BitBoard;
use crate::dir::Dir;
use crate::dir::{E, N, S, W};
use crate::dir::{NE, NW, SE, SW};
use crate::square::constants::SQUARES;
use crate::square::Square;
use std::vec::IntoIter;

/// Computes a bitboard representation of every square
/// on the edge of a chessboard.
///
fn board_border() -> BitBoard {
    RANKS[0] | RANKS[7] | FILES[0] | FILES[7]
}

/// Computes a vector containing the bishop occupancy
/// masks for each square.
///
fn compute_bishop_masks() -> Vec<BitBoard> {
    let (border, dirs) = (board_border(), vec![NE, SE, SW, NW]);
    SQUARES
        .iter()
        .map(|&sq| sq.search_all(&dirs) - border)
        .collect()
}

/// Computes the set of squares in a given direction from
/// some source square with the furthest away excluded
///
fn search_remove_last(loc: Square, dir: Dir) -> BitBoard {
    let mut res = loc.search_vec(dir);
    if res.len() > 0 {
        res.remove(res.len() - 1);
    }
    res.into_iter().collect()
}

/// Computes a vector containing the rook occupancy
/// masks for each square.
///
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
        assert_eq!(
            A2 | A3 | A5 | A6 | A7 | B4 | C4 | D4 | E4 | F4 | G4,
            rmasks[A4.i as usize]
        );
    }
}

lazy_static! {
    static ref BISHOP_MASKS: Vec<BitBoard> = compute_bishop_masks();
    static ref ROOK_MASKS: Vec<BitBoard> = compute_rook_masks();
}

fn compute_bishop_shifts() -> Vec<usize> {
    compute_bishop_masks()
        .iter()
        .map(|x| 64 - x.size())
        .collect()
}

fn compute_rook_shifts() -> Vec<usize> {
    compute_rook_masks().iter().map(|x| 64 - x.size()).collect()
}

lazy_static! {
    static ref BISHOP_SHIFTS: Vec<usize> = compute_bishop_shifts();
    static ref ROOK_SHIFTS: Vec<usize> = compute_rook_shifts();
}

fn bishop_control(loc: Square, occ: BitBoard) -> BitBoard {
    sliding_control(loc, occ, vec![NE, SE, SW, NW])
}

fn rook_control(loc: Square, occ: BitBoard) -> BitBoard {
    sliding_control(loc, occ, vec![N, E, S, W])
}

/// Computes the control set for a piece assumed to be
/// located at a given source square and which is permitted
/// to move in a specified set of directions.
///
fn sliding_control(loc: Square, occ: BitBoard, dirs: Vec<Dir>) -> BitBoard {
    let mut res = vec![];
    for dir in dirs {
        for sq in loc.search_vec(dir) {
            res.push(sq);
            if !(occ & sq).is_empty() {
                break;
            }
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod control_tests {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_sliding_control() {
        // Could split this up to test rook and bishop separately.
        let loc = D4;
        let whites = D1 | F4 | D6 | G7 | H8;
        let blacks = B2 | B4 | E3 | A7;
        let dirs = vec![N, NE, E, SE, S, SW, W, NW];
        let expected_control =
            D5 | D6 | E5 | F6 | G7 | E4 | F4 | E3 | D3 | D2 | D1 | C3 | B2 | C4 | B4 | C5 | B6 | A7;
        assert_eq!(
            expected_control,
            sliding_control(loc, whites | blacks, dirs)
        );
    }
}

/// Computes the powerset of some set of squares with the
/// resulting elements of the powerset represented as
/// bitboards.
///
fn compute_powerset(squares: &Vec<Square>) -> Vec<BitBoard> {
    if squares.is_empty() {
        vec![BitBoard::EMPTY]
    } else {
        let (head, rest) = (squares[0], &squares[1..].to_vec());
        let recursive = compute_powerset(rest);
        let mut res = vec![];
        for set in recursive {
            res.push(set);
            res.push(set | head);
        }
        res
    }
}

#[cfg(test)]
mod powerset_test {
    use super::*;
    use crate::square::constants::*;
    use std::collections::HashSet;

    #[test]
    fn test_powerset() {
        let empty = vec![BitBoard::EMPTY];
        assert_eq!(empty, compute_powerset(&vec![]));
        let non_empty = vec![A1, F3, H5];
        let mut expected = HashSet::new();
        expected.insert(BitBoard::EMPTY);
        expected.insert(A1.lift());
        expected.insert(F3.lift());
        expected.insert(H5.lift());
        expected.insert(A1 | F3);
        expected.insert(A1 | H5);
        expected.insert(F3 | H5);
        expected.insert(A1 | F3 | H5);
        let actual: HashSet<_> = compute_powerset(&non_empty).into_iter().collect();
        assert_eq!(expected, actual);
    }
}
