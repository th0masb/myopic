use crate::square::{file, rank};

mod square;
mod board;
mod hash;
mod position;

pub type Side = usize;
pub type Flank = usize;
// H1 -> .. -> A1 -> H2 ... -> A8
pub type Square = usize;
pub type Rank = usize;
pub type File = usize;
pub type Board = u64;
pub type Class = usize;
pub type Line = (Square, Square);
pub type Piece = (Side, Class);
pub type Corner = (Side, Flank);
pub type Dir = (isize, isize);

pub type SquareMap<T> = [T; 64];
pub type SideMap<T> = [T; 2];

#[macro_export]
macro_rules! board {
    ($( $x:expr ),*) => {
        {
            let mut board = 0u64;
            $(board |= 1u64 << ($x as u64);)*
            board
        }
    };
    ($( $x:expr => $($y:expr),+ );+) => {
        {
            use crate::bitboard::cord;
            let mut board = 0u64;
            $($(board |= cord($x as usize, $y as usize);)+)+
            board
        }
    };
}

pub trait Symmetric {
    fn reflect(&self) -> Self;
}

impl Symmetric for Square {
    fn reflect(&self) -> Self {
        8 * (7 - rank(*self)) + file(*self)
    }
}

#[rustfmt::skip]
pub mod constants {
    pub mod side {
        use crate::Side;
        pub const W: Side = 0; pub const B: Side = 1;
    }

    pub mod flank {
        use crate::Flank;
        pub const K: Flank = 0; pub const Q: Flank = 1;
    }

    pub mod class {
        use crate::Class;
        pub const P: Class = 0; pub const N: Class = 1; pub const B: Class = 2;
        pub const R: Class = 3; pub const Q: Class = 4; pub const K: Class = 5;
    }

    pub mod piece {
        use crate::Piece;
        use crate::constants::side;
        use crate::constants::class;

        pub const WP: Piece = (side::W, class::P); pub const WN: Piece = (side::W, class::N);
        pub const WB: Piece = (side::W, class::B); pub const WR: Piece = (side::W, class::R);
        pub const WQ: Piece = (side::W, class::Q); pub const WK: Piece = (side::W, class::K);

        pub const BP: Piece = (side::B, class::P); pub const BN: Piece = (side::B, class::N);
        pub const BB: Piece = (side::B, class::B); pub const BR: Piece = (side::B, class::R);
        pub const BQ: Piece = (side::B, class::Q); pub const BK: Piece = (side::B, class::K);
    }

    pub mod dir {
        use crate::Dir;

        pub const   N: Dir = ( 1,  0); pub const   E: Dir = ( 0, -1);
        pub const   S: Dir = (-1,  0); pub const   W: Dir = ( 0,  1);
        pub const  NE: Dir = ( 1, -1); pub const  SE: Dir = (-1, -1);
        pub const  SW: Dir = (-1,  1); pub const  NW: Dir = ( 1,  1);
        pub const NNE: Dir = ( 2, -1); pub const NEE: Dir = ( 1, -2);
        pub const SEE: Dir = (-1, -2); pub const SSE: Dir = (-2, -1);
        pub const SSW: Dir = (-2,  1); pub const SWW: Dir = (-2,  1);
        pub const NWW: Dir = ( 1,  2); pub const NNW: Dir = ( 2,  1);
    }

    pub mod boards {
        use crate::{Board, board};
        use crate::constants::square::*;

        pub const RANKS: [Board; 8] = [
            board!(A1, B1, C1, D1, E1, F1, G1, H1),
            board!(A2, B2, C2, D2, E2, F2, G2, H2),
            board!(A3, B3, C3, D3, E3, F3, G3, H3),
            board!(A4, B4, C4, D4, E4, F4, G4, H4),
            board!(A5, B5, C5, D5, E5, F5, G5, H5),
            board!(A6, B6, C6, D6, E6, F6, G6, H6),
            board!(A7, B7, C7, D7, E7, F7, G7, H7),
            board!(A8, B8, C8, D8, E8, F8, G8, H8),
        ];

        pub const FILES: [Board; 8] = [
            board!(H1, H2, H3, H4, H5, H6, H7, H8),
            board!(G1, G2, G3, G4, G5, G6, G7, G8),
            board!(F1, F2, F3, F4, F5, F6, F7, F8),
            board!(E1, E2, E3, E4, E5, E6, E7, E8),
            board!(D1, D2, D3, D4, D5, D6, D7, D8),
            board!(C1, C2, C3, C4, C5, C6, C7, C8),
            board!(B1, B2, B3, B4, B5, B6, B7, B8),
            board!(A1, A2, A3, A4, A5, A6, A7, A8),
        ];

        pub const RIM: Board = RANKS[0] | RANKS[7] | FILES[0] | FILES[7];
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
