mod board;
mod eval;
mod format;
mod hash;
mod material;
pub mod moves;
pub mod node;
mod parse;
mod phase;
pub mod position;
pub mod search;
mod see;
mod tables;
#[cfg(test)]
mod test;
mod timing;
#[rustfmt::skip]
mod constants;

pub type Side = usize;
// H1 -> .. -> A1 -> H2 ... -> A8
pub type Square = usize;
pub type Rank = usize;
pub type File = usize;
pub type Board = u64;
pub type Class = usize;
pub type Piece = usize;
pub type Corner = usize;
pub type Dir = (isize, isize);

pub type SquareMap<T> = [T; 64];
pub type SquareMatrix<T> = SquareMap<SquareMap<T>>;
pub type SideMap<T> = [T; 2];
pub type ClassMap<T> = [T; 6];
pub type PieceMap<T> = [T; 12];
pub type CornerMap<T> = [T; 4];

#[macro_export]
macro_rules! board {
    // Individual squares
    ($( $x:expr ),*) => {
        {
            use crate::constants::lift;
            let mut board = 0u64;
            $(board |= lift($x);)*
            board
        }
    };
    // Cords inclusive of source
    ($( $x:expr => $($y:expr),+ );+) => {
        {
            use crate::board::compute_cord;
            let mut board = 0u64;
            $($(board |= compute_cord($x as usize, $y as usize);)+)+
            board
        }
    };
    // Cords exclusive of source
    ($( ~$x:expr => $($y:expr),+ );+) => {
        {
            use crate::board::compute_cord;
            use crate::constants::lift;
            let mut board = 0u64;
            $($(board |= compute_cord($x as usize, $y as usize) & !lift($x);)+)+
            board
        }
    };
}

#[macro_export]
macro_rules! square_map {
    ($( $($x:expr),+ => $y:expr),+) => {
        {
            use std::default::Default;
            let mut result = [Default::default(); 64];
            $($(result[$x as usize] = $y;)+)+
            result
        }
    };
}

pub trait Symmetric {
    fn reflect(&self) -> Self;
}

#[cfg(test)]
mod macro_test {
    use crate::constants::lift;

    use crate::constants::piece;
    use crate::constants::square::*;
    use crate::{board, Piece, SquareMap};

    #[test]
    fn board_macro() {
        assert_eq!(lift(A1) | lift(A2) | lift(B5), board!(A1, A2, B5));
        assert_eq!(lift(A1) | lift(A2) | lift(A3), board!(A1 => A3));
        assert_eq!(board!(C3, C2, C1, A3, B3), board!(C3 => A3, C1));
        assert_eq!(
            board!(C3, C2, C1, A3, B3, F2, E3, D4, C5, B6, G4, H6),
            board!(C3 => A3, C1; F2 => B6, H6),
        );
        assert_eq!(
            board!(C2, C1, A3, B3, E3, D4, C5, B6, G4, H6),
            board!(~C3 => A3, C1; ~F2 => B6, H6),
        );
    }

    #[test]
    fn square_map_macro() {
        let mut expected: SquareMap<Option<Piece>> = [None; 64];
        expected[F5] = Some(piece::WB);
        expected[A8] = Some(piece::WB);
        expected[D2] = Some(piece::BR);
        assert_eq!(expected, square_map!(F5, A8 => Some(piece::WB), D2 => Some(piece::BR)));
    }
}
