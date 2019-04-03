use crate::bitboard::simple::{FILES, RANKS};
use crate::bitboard::BitBoard;
use crate::dir::Dir;
use crate::dir::{E, N, S, W};
use crate::dir::{NE, NW, SE, SW};
use crate::square::constants::SQUARES;
use crate::square::Square;
use rand::prelude::*;
use std::collections::HashSet;
use std::iter::repeat;
use std::num::Wrapping;
use std::vec::IntoIter;

/// Computes a bitboard representation of every square
/// on the edge of a chessboard.
///
fn board_border() -> BitBoard {
    RANKS[0] | RANKS[7] | FILES[0] | FILES[7]
}

fn bishop_dirs() -> Vec<Dir> {
    vec![NE, SE, SW, NW]
}

fn rook_dirs() -> Vec<Dir> {
    vec![N, E, S, W]
}

/// Computes a vector containing the occupancy
/// masks for each square.
///
fn compute_masks(dirs: Vec<Dir>) -> Vec<BitBoard> {
    SQUARES
        .iter()
        .map(|&sq| {
            dirs.iter()
                .map(|&dir| search_remove_last(sq, dir))
                .collect()
        })
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

#[cfg(test)]
mod mask_tests {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_bishop_masks() {
        let bmasks = compute_masks(bishop_dirs());
        assert_eq!(C7 | C5 | D4 | E3 | F2, bmasks[B6.i as usize]);
        let rmasks = compute_masks(rook_dirs());
        assert_eq!(
            A2 | A3 | A5 | A6 | A7 | B4 | C4 | D4 | E4 | F4 | G4,
            rmasks[A4.i as usize]
        );
    }
}

lazy_static! {
    static ref BISHOP_MASKS: Vec<BitBoard> = compute_masks(bishop_dirs());
    static ref ROOK_MASKS: Vec<BitBoard> = compute_masks(rook_dirs());
}

///
///
fn compute_shifts(dirs: Vec<Dir>) -> Vec<usize> {
    compute_masks(dirs).iter().map(|x| 64 - x.size()).collect()
}

lazy_static! {
    static ref BISHOP_SHIFTS: Vec<usize> = compute_shifts(bishop_dirs());
    static ref ROOK_SHIFTS: Vec<usize> = compute_shifts(rook_dirs());
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

/// Computes the control set for a piece assumed to be
/// located at a given source square and which is permitted
/// to move in a specified set of directions.
///
fn sliding_control(loc: Square, occ: BitBoard, dirs: &Vec<Dir>) -> BitBoard {
    let mut res = 0u64;
    for &dir in dirs {
        for sq in loc.search_vec(dir) {
            res |= (1u64 << sq.i);
            if !(occ & sq).is_empty() {
                break;
            }
        }
    }
    BitBoard::wrap(res)
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
            sliding_control(loc, whites | blacks, &dirs)
        );
    }
}

/// Use brute force trial end error to compute a valid set
/// of magic numbers
///
fn compute_magic_numbers(dirs: Vec<Dir>) -> Vec<u64> {
    let masks = compute_masks(dirs.clone());
    let shifts = compute_shifts(dirs.clone());
    let mut magics: Vec<u64> = Vec::with_capacity(64);

    for (&sq, &mask, &shift) in izip!(SQUARES.iter(), &masks, &shifts) {
        let occ_vars = compute_powerset(&mask.into_iter().collect());
        let control: Vec<_> = occ_vars
            .iter()
            .map(|&ov| sliding_control(sq, ov, &dirs).loc())
            .collect();
        let mut indices: Vec<_> = repeat(064).take(occ_vars.len()).collect();
        let mut moves: Vec<_> = repeat(064).take(occ_vars.len()).collect();
        let upper = 100000000;
        'outer: for i in 1..=upper {
            let magic = gen_magic_candidate();

            for (&occ_var, &control) in occ_vars.iter().zip(control.iter()) {
                let (w_occ_var, w_num) = (Wrapping(occ_var.loc()), Wrapping(magic));
                let index = ((w_occ_var * w_num).0 >> shift) as usize;
                if indices[index] == i {
                    if moves[index] != control {
                        // The magic candidate has failed
                        continue 'outer;
                    }
                } else {
                    indices[index] = i;
                    moves[index] = control;
                }
            }

            if i == upper {
                panic!("Failed to generate number!")
            } else {
                magics.push(magic);
                break;
            }
        }
    }
    magics
}

fn gen_magic_candidate() -> u64 {
    rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
}

#[cfg(test)]
mod magic_num_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, compute_magic_numbers(vec![NE, SE, SW, NW]).len() > 0);
    }
}
