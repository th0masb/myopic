use crate::{BitBoard, Square};
use itertools::izip;
use std::iter::repeat;
use std::num::Wrapping;

/// Applies the magic index mapping operation by multiplying the occupancy
/// and magic number together (allowing overflow) and then performing a right
/// shift on the result.
pub fn compute_index(occupancy: u64, magic: u64, shift: usize) -> usize {
    let (o, m) = (Wrapping(occupancy), Wrapping(magic));
    ((o * m).0 >> shift) as usize
}

/// Use brute force trial end error to compute a valid set of magic numbers.
/// A magic number for a base.square is considered to be valid if it causes no
/// conflicting collisions among the occupancy variations, that is no two
/// variations which map to the same index but have different control sets.
#[allow(dead_code)]
pub fn compute_magic_numbers<T, F>(masks: Vec<u64>, f: F) -> Vec<u64>
where
    T: PartialEq + Clone,
    F: Fn(Square, BitBoard) -> T,
{
    let shifts = compute_shifts(&masks);
    let mut magics: Vec<u64> = Vec::with_capacity(64);
    for (sq, &mask, &shift) in izip!(Square::iter(), &masks, &shifts) {
        let occupancy_variations = compute_powerset(&BitBoard(mask).into_iter().collect());
        let values_to_cache: Vec<_> = occupancy_variations.iter().map(|&ov| f(sq, ov)).collect();
        // Brute force try to find a number for which the mapping is consistent
        let upper = 1000000;
        'outer: for i in 1..=upper {
            let mut cache: Vec<Option<T>> = repeat(None).take(occupancy_variations.len()).collect();
            let magic = gen_magic_candidate();
            for (&variation, value) in occupancy_variations.iter().zip(values_to_cache.iter()) {
                let index = compute_index(variation.0, magic, shift);
                match cache[index].as_ref() {
                    None => cache[index] = Some(value.clone()),
                    Some(existing_value) => {
                        if existing_value != value {
                            // The magic candidate has failed as it has mapped two occupancy
                            // variations with different values to the same index.
                            continue 'outer;
                        }
                    }
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

/// Computes the magic bitshift values for all squares which is defined to
/// be the 1 count of the corresponding occupancy mask subtracted from 64.
fn compute_shifts(masks: &Vec<u64>) -> Vec<usize> {
    masks.iter().map(|&mask| 64 - BitBoard(mask).size()).collect()
}

/// Computes the powerset of some set of squares with the resulting elements
/// of the powerset represented as bitboards.
pub fn compute_powerset(squares: &Vec<Square>) -> Vec<BitBoard> {
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

/// Generates a random unsigned long with a sparse set of 1 bits.
fn gen_magic_candidate() -> u64 {
    rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
}

#[cfg(test)]
mod powerset_test {
    use std::collections::HashSet;

    use crate::square::Square::*;

    use super::*;

    #[test]
    fn test() {
        let empty = vec![BitBoard::EMPTY];
        assert_eq!(empty, compute_powerset(&vec![]));
        let non_empty = vec![A1, F3, H5];
        let mut expected = HashSet::new();
        expected.insert(BitBoard::EMPTY);
        expected.insert(A1.into());
        expected.insert(F3.into());
        expected.insert(H5.into());
        expected.insert(A1 | F3);
        expected.insert(A1 | H5);
        expected.insert(F3 | H5);
        expected.insert(A1 | F3 | H5);
        let actual: HashSet<_> = compute_powerset(&non_empty).into_iter().collect();
        assert_eq!(expected, actual);
    }
}
