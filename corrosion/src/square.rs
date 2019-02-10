use super::dir::Dir;
use std::fmt;

#[derive(Debug)]
pub struct Square {
    pub i: i8,
    pub rank: i8,
    pub file: i8,

    // Get rid of these and make copyable type
    pub loc: u64,
    pub name: &'static str
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Square {
    pub fn next(&self, dir: &Dir) -> Option<&Square> {
        let new_rank = self.rank + dir.dr;
        let new_file = self.file + dir.df;
        if -1 < new_rank && new_rank < 8 && -1 < new_file && new_file < 8 {
            Some(&ALL[(8 * new_rank + new_file) as usize])
        }
        else {
            None
        }
    }
    
    fn search_dir(&self, dir: &Dir) -> Vec<&Square> { 
        match self.next(dir) {
            None => vec!(),
            Some(sq) => {
                let mut recursion = sq.search_dir(dir);
                recursion.push(sq);
                recursion
            }
        }
    }

    pub fn search_all(&self, dirs: Vec<&Dir>) -> Vec<&Square> {
        dirs.iter().flat_map(|dir| self.search_dir(dir)).collect()
    }

    pub fn search_one(&self, dirs: Vec<&Dir>) -> Vec<&Square> {
        dirs.iter().flat_map(|dir| self.next(dir).into_iter()).collect()
    }

    const fn init(index: i8, name: &'static str) -> Square {
        Square { i: index, 
            rank: index / 8, 
            file: index % 8, 
            loc: 1u64 << index,
            name
        }
    }
}

pub const H1: Square = Square::init(0, "H1");
pub const G1: Square = Square::init(1, "G1");
pub const F1: Square = Square::init(2, "F1");
pub const E1: Square = Square::init(3, "E1");
pub const D1: Square = Square::init(4, "D1");
pub const C1: Square = Square::init(5, "C1");
pub const B1: Square = Square::init(6, "B1");
pub const A1: Square = Square::init(7, "A1");

pub const H2: Square = Square::init(8 , "H2");
pub const G2: Square = Square::init(9 , "G2");
pub const F2: Square = Square::init(10, "F2");
pub const E2: Square = Square::init(11, "E2");
pub const D2: Square = Square::init(12, "D2");
pub const C2: Square = Square::init(13, "C2");
pub const B2: Square = Square::init(14, "B2");
pub const A2: Square = Square::init(15, "A2");

pub const H3: Square = Square::init(16, "H3");
pub const G3: Square = Square::init(17, "G3");
pub const F3: Square = Square::init(18, "F3");
pub const E3: Square = Square::init(19, "E3");
pub const D3: Square = Square::init(20, "D3");
pub const C3: Square = Square::init(21, "C3");
pub const B3: Square = Square::init(22, "B3");
pub const A3: Square = Square::init(23, "A3");

pub const H4: Square = Square::init(24, "H4");
pub const G4: Square = Square::init(25, "G4");
pub const F4: Square = Square::init(26, "F4");
pub const E4: Square = Square::init(27, "E4");
pub const D4: Square = Square::init(28, "D4");
pub const C4: Square = Square::init(29, "C4");
pub const B4: Square = Square::init(30, "B4");
pub const A4: Square = Square::init(31, "A4");

pub const H5: Square = Square::init(32, "H5");
pub const G5: Square = Square::init(33, "G5");
pub const F5: Square = Square::init(34, "F5");
pub const E5: Square = Square::init(35, "E5");
pub const D5: Square = Square::init(36, "D5");
pub const C5: Square = Square::init(37, "C5");
pub const B5: Square = Square::init(38, "B5");
pub const A5: Square = Square::init(39, "A5");

pub const H6: Square = Square::init(40, "H6");
pub const G6: Square = Square::init(41, "G6");
pub const F6: Square = Square::init(42, "F6");
pub const E6: Square = Square::init(43, "E6");
pub const D6: Square = Square::init(44, "D6");
pub const C6: Square = Square::init(45, "C6");
pub const B6: Square = Square::init(46, "B6");
pub const A6: Square = Square::init(47, "A6");

pub const H7: Square = Square::init(48, "H7");
pub const G7: Square = Square::init(49, "G7");
pub const F7: Square = Square::init(50, "F7");
pub const E7: Square = Square::init(51, "E7");
pub const D7: Square = Square::init(52, "D7");
pub const C7: Square = Square::init(53, "C7");
pub const B7: Square = Square::init(54, "B7");
pub const A7: Square = Square::init(55, "A7");

pub const H8: Square = Square::init(56, "H8");
pub const G8: Square = Square::init(57, "G8");
pub const F8: Square = Square::init(58, "F8");
pub const E8: Square = Square::init(59, "E8");
pub const D8: Square = Square::init(60, "D8");
pub const C8: Square = Square::init(61, "C8");
pub const B8: Square = Square::init(62, "B8");
pub const A8: Square = Square::init(63, "A8");

pub const ALL: [&'static Square; 64] = [
    &H1, &G1, &F1, &E1, &D1, &C1, &B1, &A1,
    &H2, &G2, &F2, &E2, &D2, &C2, &B2, &A2,
    &H3, &G3, &F3, &E3, &D3, &C3, &B3, &A3,
    &H4, &G4, &F4, &E4, &D4, &C4, &B4, &A4,
    &H5, &G5, &F5, &E5, &D5, &C5, &B5, &A5,
    &H6, &G6, &F6, &E6, &D6, &C6, &B6, &A6,
    &H7, &G7, &F7, &E7, &D7, &C7, &B7, &A7,
    &H8, &G8, &F8, &E8, &D8, &C8, &B8, &A8
];

