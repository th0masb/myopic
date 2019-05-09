use crate::base::bitboard::BitBoard;
use crate::base::dir::Dir;
use crate::base::dir::E;
use crate::base::dir::N;
use crate::base::dir::NE;
use crate::base::dir::NW;
use crate::base::dir::S;
use crate::base::dir::SE;
use crate::base::dir::SW;
use crate::base::dir::W;
use crate::base::square::{constants::SQUARES, Square};

use std::cmp;

pub fn get_cord(source: Square, target: Square) -> BitBoard {
    debug_assert!(source != target);
    let (min, max) = (cmp::min(source, target), cmp::max(source, target));
    CACHE[OFFSETS[min.i as usize] + (max.i - min.i - 1) as usize]
}

lazy_static! {
    static ref CACHE: Vec<BitBoard> = compute_cord_cache();
    static ref OFFSETS: Vec<usize> = compute_offsets();
}

fn compute_offsets() -> Vec<usize> {
    let mut dest: Vec<usize> = Vec::with_capacity(63);
    dest.push(0);
    for i in 1..63 {
        dest.push(dest[i - 1] + 64 - i);
    }
    dest
}

fn compute_cord_cache() -> Vec<BitBoard> {
    let mut dest: Vec<BitBoard> = Vec::with_capacity(2016);
    for i in 0..63 {
        for j in i + 1..64 {
            dest.push(compute_cord_impl(SQUARES[i], SQUARES[j]))
        }
    }
    dest
}

fn compute_cord_impl(source: Square, target: Square) -> BitBoard {
    [N, NE, E, SE, S, SW, W, NW]
        .iter()
        .find(|&d| source.search(*d).contains(target))
        .map_or(BitBoard::EMPTY, |&d| {
            takewhile_inc(source, target, d) | source
        })
}

fn takewhile_inc(source: Square, target: Square, dir: Dir) -> BitBoard {
    source
        .search_vec(dir)
        .into_iter()
        .take_while(|&sq| sq != target)
        .collect::<BitBoard>()
        | target
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base::square::constants::*;

    #[test]
    fn test_compute_cord() {
        assert_eq!(H1 | H2 | H3, compute_cord_impl(H1, H3));
    }

    #[test]
    fn test_get_cord() {
        assert_eq!(H1 | H2 | H3, get_cord(H1, H3));
        assert_eq!(H1 | H2 | H3, get_cord(H3, H1));
        assert_eq!(F3 | E3 | D3 | C3, get_cord(C3, F3));
        assert_eq!(D5 | E6 | F7, get_cord(D5, F7));
        assert_eq!(A8 | B7, get_cord(A8, B7));
        assert_eq!(B8 | A8, get_cord(B8, A8));
    }
}
