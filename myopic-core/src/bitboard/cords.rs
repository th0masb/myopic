use std::cmp;

use crate::bitboard::BitBoard;
use crate::direction::Dir;
use crate::direction::E;
use crate::direction::N;
use crate::direction::NE;
use crate::direction::NW;
use crate::direction::S;
use crate::direction::SE;
use crate::direction::SW;
use crate::direction::W;
use crate::square::Square;

pub fn get_cord(source: Square, target: Square) -> BitBoard {
    let (min, max) = (cmp::min(source, target), cmp::max(source, target));
    CACHE[OFFSETS[min as usize] + (max as usize) - (min as usize) - 1]
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
            dest.push(compute_cord_impl(Square::from_index(i), Square::from_index(j)));
        }
    }
    dest
}

fn compute_cord_impl(source: Square, target: Square) -> BitBoard {
    [N, NE, E, SE, S, SW, W, NW]
        .iter()
        .find(|&d| source.search(*d).contains(target))
        .map_or(BitBoard::EMPTY, |&d| takewhile_inc(source, target, d) | source)
}

fn takewhile_inc(source: Square, target: Square, dir: Dir) -> BitBoard {
    source.search_vec(dir).into_iter().take_while(|&sq| sq != target).collect::<BitBoard>() | target
}

#[cfg(test)]
mod test {
    use crate::base::square::Square::*;

    use super::*;

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
