use crate::bitboard::BitBoard;
use crate::dir;
use crate::square::constants::*;
use crate::square::Square;

fn create_files() -> Vec<BitBoard> {
    (H1.search(dir::W) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::N) | sq)
        .collect()
}

fn create_ranks() -> Vec<BitBoard> {
    (H1.search(dir::N) | H1)
        .into_iter()
        .map(|sq| sq.search(dir::W) | sq)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::square::constants::*;
    use super::*;

    #[test]
    fn test_create_files() {
        assert_eq!(H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8, create_files()[0]);
    }

    #[test]
    fn test_create_ranks() {
       assert_eq!(A3 | B3 | C3 | D3 | E3 | F3 | G3 | H3, create_ranks()[2]);
    }
}

lazy_static! {
    pub static ref RANKS: Vec<BitBoard> = create_ranks();
    pub static ref FILES: Vec<BitBoard> = create_files();
}

//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhitePawn;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackPawn;
//
//trait Indexable {
//    fn index(&self) -> u8;
//}
//
//const test: [&dyn Indexable; 0] = [];
//
//impl WhitePawn {
//    fn index(&self) -> u8 {
//        0
//    }
//}
