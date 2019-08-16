use crate::board::Termination;
use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::pinning::PinnedSet;
use crate::board::BoardImpl;
use crate::pieces::Piece;
use crate::base::Reflectable;

pub(in crate::board::implementation) mod control;
pub(in crate::board::implementation) mod pinning;
pub(in crate::board::implementation) mod termination;
pub(in crate::board::implementation) mod constraints;
#[cfg(test)]
pub(in crate::board::implementation) mod test;


#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    whites: Option<BitBoard>,
    blacks: Option<BitBoard>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<PinnedSet>,
}

impl CalculationCache {
    pub fn empty() -> CalculationCache {
        CalculationCache {
            termination_status: None,
            whites: None,
            blacks: None,
            passive_control: None,
            pinned_set: None,
        }
    }
}

impl BoardImpl {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.whites = None;
        self.cache.blacks = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
    }


    pub fn termination_status_impl(&mut self) -> Option<Termination> {
        match &self.cache.termination_status {
            Some(x) => *x,
            None => {
                let result = self.compute_termination();
                self.cache.termination_status = Some(result);
                result
            }
        }
    }

    pub fn whites_impl(&mut self) -> BitBoard {
        match &self.cache.whites {
            Some(x) => *x,
            None => {
                let result = Piece::iter_w()
                    .fold(BitBoard::EMPTY, |a, p| a | self.pieces.locs_impl(p));
                self.cache.whites = Some(result);
                result
            }
        }
    }

    pub fn blacks_impl(&mut self) -> BitBoard {
        match &self.cache.blacks {
            Some(x) => *x,
            None => {
                let result = Piece::iter_b()
                    .fold(BitBoard::EMPTY, |a, p| a | self.pieces.locs_impl(p));
                self.cache.blacks = Some(result);
                result
            }
        }
    }

    pub fn passive_control_impl(&mut self) -> BitBoard {
        match &self.cache.passive_control {
            Some(x) => *x,
            None => {
                let result = self.compute_control(self.active.reflect());
                self.cache.passive_control = Some(result);
                result
            }
        }
    }

    pub fn pinned_set_impl(&mut self) -> PinnedSet {
        match &self.cache.pinned_set {
            Some(x) => x.clone(),
            None => {
                let result = self.compute_pinned();
                self.cache.pinned_set = Some(result.clone());
                result
            }
        }
    }
}

