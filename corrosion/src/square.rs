#[derive(Debug)]
pub struct Square {
    pub index: u8,
    pub rank: u8,
    pub file: u8,
    pub loc: u64,
}


pub const H1: Square = Square { index: 0, rank: 0, file: 0, loc: (1 as u64) << 0 };
pub const G1: Square = Square { index: 1, rank: 0, file: 1, loc: (1 as u64) << 1 };
pub const F1: Square = Square { index: 2, rank: 0, file: 2, loc: (1 as u64) << 2 };
pub const E1: Square = Square { index: 3, rank: 0, file: 3, loc: (1 as u64) << 3 };
pub const D1: Square = Square { index: 4, rank: 0, file: 4, loc: (1 as u64) << 4 };
pub const C1: Square = Square { index: 5, rank: 0, file: 5, loc: (1 as u64) << 5 };
pub const B1: Square = Square { index: 6, rank: 0, file: 6, loc: (1 as u64) << 6 };
pub const A1: Square = Square { index: 7, rank: 0, file: 7, loc: (1 as u64) << 7 };

pub const H2: Square = Square { index: 8 , rank: 1, file: 0, loc: (1 as u64) << 8  };
pub const G2: Square = Square { index: 9 , rank: 1, file: 1, loc: (1 as u64) << 9  };
pub const F2: Square = Square { index: 10, rank: 1, file: 2, loc: (1 as u64) << 10 };
pub const E2: Square = Square { index: 11, rank: 1, file: 3, loc: (1 as u64) << 11 };
pub const D2: Square = Square { index: 12, rank: 1, file: 4, loc: (1 as u64) << 12 };
pub const C2: Square = Square { index: 13, rank: 1, file: 5, loc: (1 as u64) << 13 };
pub const B2: Square = Square { index: 14, rank: 1, file: 6, loc: (1 as u64) << 14 };
pub const A2: Square = Square { index: 15, rank: 1, file: 7, loc: (1 as u64) << 15 };

pub const H3: Square = Square { index: 16, rank: 2, file: 0, loc: (1 as u64) << 16 };
pub const G3: Square = Square { index: 17, rank: 2, file: 1, loc: (1 as u64) << 17 };
pub const F3: Square = Square { index: 18, rank: 2, file: 2, loc: (1 as u64) << 18 };
pub const E3: Square = Square { index: 19, rank: 2, file: 3, loc: (1 as u64) << 19 };
pub const D3: Square = Square { index: 20, rank: 2, file: 4, loc: (1 as u64) << 20 };
pub const C3: Square = Square { index: 21, rank: 2, file: 5, loc: (1 as u64) << 21 };
pub const B3: Square = Square { index: 22, rank: 2, file: 6, loc: (1 as u64) << 22 };
pub const A3: Square = Square { index: 23, rank: 2, file: 7, loc: (1 as u64) << 23 };

pub const H4: Square = Square { index: 24, rank: 3, file: 0, loc: (1 as u64) << 24 };
pub const G4: Square = Square { index: 25, rank: 3, file: 1, loc: (1 as u64) << 25 };
pub const F4: Square = Square { index: 26, rank: 3, file: 2, loc: (1 as u64) << 26 };
pub const E4: Square = Square { index: 27, rank: 3, file: 3, loc: (1 as u64) << 27 };
pub const D4: Square = Square { index: 28, rank: 3, file: 4, loc: (1 as u64) << 28 };
pub const C4: Square = Square { index: 29, rank: 3, file: 5, loc: (1 as u64) << 29 };
pub const B4: Square = Square { index: 30, rank: 3, file: 6, loc: (1 as u64) << 30 };
pub const A4: Square = Square { index: 31, rank: 3, file: 7, loc: (1 as u64) << 31 };

pub const H5: Square = Square { index: 32, rank: 4, file: 0, loc: (1 as u64) << 32 };
pub const G5: Square = Square { index: 33, rank: 4, file: 1, loc: (1 as u64) << 33 };
pub const F5: Square = Square { index: 34, rank: 4, file: 2, loc: (1 as u64) << 34 };
pub const E5: Square = Square { index: 35, rank: 4, file: 3, loc: (1 as u64) << 35 };
pub const D5: Square = Square { index: 36, rank: 4, file: 4, loc: (1 as u64) << 36 };
pub const C5: Square = Square { index: 37, rank: 4, file: 5, loc: (1 as u64) << 37 };
pub const B5: Square = Square { index: 38, rank: 4, file: 6, loc: (1 as u64) << 38 };
pub const A5: Square = Square { index: 39, rank: 4, file: 7, loc: (1 as u64) << 39 };

pub const H6: Square = Square { index: 40, rank: 5, file: 0, loc: (1 as u64) << 40 };
pub const G6: Square = Square { index: 41, rank: 5, file: 1, loc: (1 as u64) << 41 };
pub const F6: Square = Square { index: 42, rank: 5, file: 2, loc: (1 as u64) << 42 };
pub const E6: Square = Square { index: 43, rank: 5, file: 3, loc: (1 as u64) << 43 };
pub const D6: Square = Square { index: 44, rank: 5, file: 4, loc: (1 as u64) << 44 };
pub const C6: Square = Square { index: 45, rank: 5, file: 5, loc: (1 as u64) << 45 };
pub const B6: Square = Square { index: 46, rank: 5, file: 6, loc: (1 as u64) << 46 };
pub const A6: Square = Square { index: 47, rank: 5, file: 7, loc: (1 as u64) << 47 };

pub const H7: Square = Square { index: 48, rank: 6, file: 0, loc: (1 as u64) << 48 };
pub const G7: Square = Square { index: 49, rank: 6, file: 1, loc: (1 as u64) << 49 };
pub const F7: Square = Square { index: 50, rank: 6, file: 2, loc: (1 as u64) << 50 };
pub const E7: Square = Square { index: 51, rank: 6, file: 3, loc: (1 as u64) << 51 };
pub const D7: Square = Square { index: 52, rank: 6, file: 4, loc: (1 as u64) << 52 };
pub const C7: Square = Square { index: 53, rank: 6, file: 5, loc: (1 as u64) << 53 };
pub const B7: Square = Square { index: 54, rank: 6, file: 6, loc: (1 as u64) << 54 };
pub const A7: Square = Square { index: 55, rank: 6, file: 7, loc: (1 as u64) << 55 };

pub const H8: Square = Square { index: 56, rank: 7, file: 0, loc: (1 as u64) << 56 };
pub const G8: Square = Square { index: 57, rank: 7, file: 1, loc: (1 as u64) << 57 };
pub const F8: Square = Square { index: 58, rank: 7, file: 2, loc: (1 as u64) << 58 };
pub const E8: Square = Square { index: 59, rank: 7, file: 3, loc: (1 as u64) << 59 };
pub const D8: Square = Square { index: 60, rank: 7, file: 4, loc: (1 as u64) << 60 };
pub const C8: Square = Square { index: 61, rank: 7, file: 5, loc: (1 as u64) << 61 };
pub const B8: Square = Square { index: 62, rank: 7, file: 6, loc: (1 as u64) << 62 };
pub const A8: Square = Square { index: 63, rank: 7, file: 7, loc: (1 as u64) << 63 };

pub static ALL: Vec<&Square> = vec!(&H1);//, G1, F1, E1, D1, C1, B1, A1);

impl Square {

}
    
