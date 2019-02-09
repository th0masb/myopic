use super::dir::Dir;
use std::fmt;

#[derive(Debug)]
pub struct Square {
    pub i: i8,
    pub rank: i8,
    pub file: i8,
    pub loc: u64,
    pub name: &'static str
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "
        panic!()
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

    const fn init(index: i8, name: &str) -> Square {
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

pub const H2: Square = Square::init(8 );
pub const G2: Square = Square::init(9 );
pub const F2: Square = Square::init(10);
pub const E2: Square = Square::init(11);
pub const D2: Square = Square::init(12);
pub const C2: Square = Square::init(13);
pub const B2: Square = Square::init(14);
pub const A2: Square = Square::init(15);

pub const H3: Square = Square::init(16);
pub const G3: Square = Square::init(17);
pub const F3: Square = Square::init(18);
pub const E3: Square = Square::init(19);
pub const D3: Square = Square::init(20);
pub const C3: Square = Square::init(21);
pub const B3: Square = Square::init(22);
pub const A3: Square = Square::init(23);

pub const H4: Square = Square::init(24);
pub const G4: Square = Square::init(25);
pub const F4: Square = Square::init(26);
pub const E4: Square = Square::init(27);
pub const D4: Square = Square::init(28);
pub const C4: Square = Square::init(29);
pub const B4: Square = Square::init(30);
pub const A4: Square = Square::init(31);

pub const H5: Square = Square::init(32);
pub const G5: Square = Square::init(33);
pub const F5: Square = Square::init(34);
pub const E5: Square = Square::init(35);
pub const D5: Square = Square::init(36);
pub const C5: Square = Square::init(37);
pub const B5: Square = Square::init(38);
pub const A5: Square = Square::init(39);

pub const H6: Square = Square::init(40);
pub const G6: Square = Square::init(41);
pub const F6: Square = Square::init(42);
pub const E6: Square = Square::init(43);
pub const D6: Square = Square::init(44);
pub const C6: Square = Square::init(45);
pub const B6: Square = Square::init(46);
pub const A6: Square = Square::init(47);

pub const H7: Square = Square::init(48);
pub const G7: Square = Square::init(49);
pub const F7: Square = Square::init(50);
pub const E7: Square = Square::init(51);
pub const D7: Square = Square::init(52);
pub const C7: Square = Square::init(53);
pub const B7: Square = Square::init(54);
pub const A7: Square = Square::init(55);

pub const H8: Square = Square::init(56);
pub const G8: Square = Square::init(57);
pub const F8: Square = Square::init(58);
pub const E8: Square = Square::init(59);
pub const D8: Square = Square::init(60);
pub const C8: Square = Square::init(61);
pub const B8: Square = Square::init(62);
pub const A8: Square = Square::init(63);

pub const ALL: [Square; 64] = [
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8
];

