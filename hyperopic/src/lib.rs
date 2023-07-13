use crate::board::iter;

mod board;
mod hash;
mod moves;
mod parse;
mod position;
#[cfg(test)]
mod test;

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
            use crate::lift;
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
            use crate::lift;
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

pub const fn piece_side(piece: Piece) -> Side {
    piece / 6
}

pub const fn piece_class(piece: Piece) -> Class {
    piece % 6
}

pub const fn create_piece(side: Side, class: Class) -> Piece {
    side * 6 + class
}

pub const fn square_rank(square: Square) -> Rank {
    square / 8
}

pub const fn square_file(square: Square) -> File {
    square % 8
}

pub const fn lift(square: Square) -> Board {
    1u64 << (square as u64)
}

pub trait Symmetric {
    fn reflect(&self) -> Self;
}

pub const fn reflect_side(side: Side) -> Side {
    (side + 1) % 2
}

pub const fn reflect_corner(corner: Corner) -> Corner {
    (corner + 2) % 4
}

pub const fn reflect_square(square: Square) -> Square {
    8 * (7 - square_rank(square)) + square_file(square)
}

pub const fn reflect_piece(piece: Piece) -> Piece {
    (piece + 6) % 12
}

pub fn reflect_board(board: Board) -> Board {
    iter(board).map(|sq| reflect_square(sq)).fold(0u64, |a, n| a | lift(n))
}

pub const fn in_board(board: Board, square: Square) -> bool {
    board & lift(square) != 0
}

pub const fn is_superset(left: Board, right: Board) -> bool {
    (left & right) == right
}

#[rustfmt::skip]
pub mod constants {
    pub mod side {
        use crate::Side;
        pub const W: Side = 0; pub const B: Side = 1;
    }

    pub mod class {
        use crate::Class;
        pub const P: Class = 0; pub const N: Class = 1; pub const B: Class = 2;
        pub const R: Class = 3; pub const Q: Class = 4; pub const K: Class = 5;
    }

    pub mod corner {
        use crate::Corner;
        pub const WK: Corner = 0; pub const WQ: Corner = 1;
        pub const BK: Corner = 2; pub const BQ: Corner = 3;
    }

    pub mod piece {
        use crate::Piece;
        pub const WP: Piece = 0; pub const WN: Piece = 1;
        pub const WB: Piece = 2; pub const WR: Piece = 3;
        pub const WQ: Piece = 4; pub const WK: Piece = 5;

        pub const BP: Piece = 6; pub const BN: Piece = 7;
        pub const BB: Piece = 8; pub const BR: Piece = 9;
        pub const BQ: Piece = 10; pub const BK: Piece = 11;
    }

    pub mod dir {
        use crate::Dir;
        pub const   N: Dir = ( 1,  0); pub const   E: Dir = ( 0, -1);
        pub const   S: Dir = (-1,  0); pub const   W: Dir = ( 0,  1);
        pub const  NE: Dir = ( 1, -1); pub const  SE: Dir = (-1, -1);
        pub const  SW: Dir = (-1,  1); pub const  NW: Dir = ( 1,  1);
        pub const NNE: Dir = ( 2, -1); pub const NEE: Dir = ( 1, -2);
        pub const SEE: Dir = (-1, -2); pub const SSE: Dir = (-2, -1);
        pub const SSW: Dir = (-2,  1); pub const SWW: Dir = (-1,  2);
        pub const NWW: Dir = ( 1,  2); pub const NNW: Dir = ( 2,  1);
    }

    pub mod boards {
        use crate::{Board, board};
        use crate::constants::square::*;

        pub const EMPTY: Board = 0u64;
        pub const ALL: Board = !0u64;
        pub const RIM: Board = board!(A1 => A8, H1; H8 => A8, H1);
        pub const ENPASSANT_RANKS: Board = board!(A2 => H2; A4 => H4; A5 => H5; A7 => H7);

        pub const RANKS: [Board; 8] = [
            board!(A1 => H1),
            board!(A2 => H2),
            board!(A3 => H3),
            board!(A4 => H4),
            board!(A5 => H5),
            board!(A6 => H6),
            board!(A7 => H7),
            board!(A8 => H8),
        ];

        pub const FILES: [Board; 8] = [
            board!(H1 => H8),
            board!(G1 => G8),
            board!(F1 => F8),
            board!(E1 => E8),
            board!(F1 => F8),
            board!(C1 => C8),
            board!(B1 => B8),
            board!(A1 => A8),
        ];
    }

    pub mod square {
        use crate::Square;

        pub const H1: Square =  0; pub const G1: Square =  1; pub const F1: Square =  2; pub const E1: Square =  3;
        pub const D1: Square =  4; pub const C1: Square =  5; pub const B1: Square =  6; pub const A1: Square =  7;
        pub const H2: Square =  8; pub const G2: Square =  9; pub const F2: Square = 10; pub const E2: Square = 11;
        pub const D2: Square = 12; pub const C2: Square = 13; pub const B2: Square = 14; pub const A2: Square = 15;
        pub const H3: Square = 16; pub const G3: Square = 17; pub const F3: Square = 18; pub const E3: Square = 19;
        pub const D3: Square = 20; pub const C3: Square = 21; pub const B3: Square = 22; pub const A3: Square = 23;
        pub const H4: Square = 24; pub const G4: Square = 25; pub const F4: Square = 26; pub const E4: Square = 27;
        pub const D4: Square = 28; pub const C4: Square = 29; pub const B4: Square = 30; pub const A4: Square = 31;
        pub const H5: Square = 32; pub const G5: Square = 33; pub const F5: Square = 34; pub const E5: Square = 35;
        pub const D5: Square = 36; pub const C5: Square = 37; pub const B5: Square = 38; pub const A5: Square = 39;
        pub const H6: Square = 40; pub const G6: Square = 41; pub const F6: Square = 42; pub const E6: Square = 43;
        pub const D6: Square = 44; pub const C6: Square = 45; pub const B6: Square = 46; pub const A6: Square = 47;
        pub const H7: Square = 48; pub const G7: Square = 49; pub const F7: Square = 50; pub const E7: Square = 51;
        pub const D7: Square = 52; pub const C7: Square = 53; pub const B7: Square = 54; pub const A7: Square = 55;
        pub const H8: Square = 56; pub const G8: Square = 57; pub const F8: Square = 58; pub const E8: Square = 59;
        pub const D8: Square = 60; pub const C8: Square = 61; pub const B8: Square = 62; pub const A8: Square = 63;
    }
}

#[cfg(test)]
mod macro_test {
    use super::lift;

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
