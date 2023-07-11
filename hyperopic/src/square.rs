use std::cmp::{max, min};
use crate::{Board, Dir, File, Rank, Square};


pub const fn rank(square: Square) -> Rank {
    square / 8
}

pub const fn file(square: Square) -> File {
    square % 8
}

pub const fn lift(square: Square) -> Board {
    1u64 << (square as u64)
}

pub fn next(square: Square, (dr, df): Dir) -> Option<Square> {
    let next_r = (rank(square) as isize) + dr;
    let next_f = (file(square) as isize) + df;
    if 0 <= min(next_f, next_r) && max(next_f, next_r) < 8 {
        Some(8 * (next_r as usize) + next_f as usize)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::constants::square::*;
    use crate::constants::dir::*;

    #[test]
    fn next() {
        assert_eq!(Some(A2), super::next(A1, N));
        assert_eq!(Some(B5), super::next(D4, NWW));
        assert_eq!(None, super::next(A8, N));
        assert_eq!(None, super::next(C2, SSE));
        assert_eq!(None, super::next(H6, NE));
        assert_eq!(None, super::next(A7, W));
    }
}

