use crate::board::Termination;
use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::pinning::PinnedSet;
use crate::board::BoardImpl;
use crate::pieces::Piece;
use crate::base::Reflectable;

pub(in crate::board::implementation) mod constraints;
mod control;
mod pinning;
mod termination;

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<PinnedSet>,
}

impl CalculationCache {
    pub fn empty() -> CalculationCache {
        CalculationCache {
            termination_status: None,
            passive_control: None,
            pinned_set: None,
        }
    }
}

impl BoardImpl {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
    }


//    pub fn whites_impl(&mut self) -> BitBoard {
//        match &self.cache.whites {
//            Some(x) => *x,
//            None => {
//                let result = Piece::iter_w()
//                    .fold(BitBoard::EMPTY, |a, p| a | self.pieces.locs_impl(p));
//                self.cache.whites = Some(result);
//                result
//            }
//        }
//    }
//
//    pub fn blacks_impl(&mut self) -> BitBoard {
//        match &self.cache.blacks {
//            Some(x) => *x,
//            None => {
//                let result = Piece::iter_b()
//                    .fold(BitBoard::EMPTY, |a, p| a | self.pieces.locs_impl(p));
//                self.cache.blacks = Some(result);
//                result
//            }
//        }
//    }

}

