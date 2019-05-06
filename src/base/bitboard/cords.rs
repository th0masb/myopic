use crate::base::square::Square;
use crate::base::bitboard::BitBoard;
use crate::base::dir::Dir;
use crate::base::dir::N;
use crate::base::dir::E;
use crate::base::dir::S;
use crate::base::dir::W;
use crate::base::dir::NE;
use crate::base::dir::SE;
use crate::base::dir::SW;
use crate::base::dir::NW;

use std::cmp;

pub fn get_coord(source: Square, target: Square) -> BitBoard {
    debug_assert!(source != target);
    let (min, max) = (cmp::min(source, target), cmp::max(source, target));
    let offset = OFFSETS[min.i as usize] + (max.i - min.i) as usize;
    CACHE[offset]
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
    unimplemented!()
}

fn compute_cord_impl(source: Square, target: Square) -> BitBoard {
    [N, NE, E, SE, S, SW, W, NW].iter()
        .find(|&d| source.search(*d).contains(target))
        .map_or(BitBoard::EMPTY, |&d| takewhile_inc(source, target, d))
}

fn takewhile_inc(source: Square, target: Square, dir: Dir) -> BitBoard {
    source.search_vec(dir).into_iter()
        .take_while(|&sq| sq != target).collect::<BitBoard>() | target
}
