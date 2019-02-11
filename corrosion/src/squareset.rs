use super::square::Square;
use super::square;

pub struct SquareSet(u64);

struct SquareSetIterator { src: u64, counter: usize }

//impl Iterator for SquareSetIterator {
//    type Item = &'static Square;
//
//    fn next(&mut self) -> Option<&'static Square> {
//        let x = square::ALL[1].loc;
//        Some(square::ALL[0])
//    }
//}
