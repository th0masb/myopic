use crate::board::Termination;
use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::pinning::PinnedSet;
use crate::board::BoardImpl;
use crate::base::Reflectable;
use crate::board::implementation::cache::constraints::MoveConstraints;

pub(in crate::board::implementation) mod constraints;
mod control;
mod pinning;
mod termination;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<PinnedSet>,
    //move_constraints: Option<MoveConstraints>,
}

impl CalculationCache {
    pub fn empty() -> CalculationCache {
        CalculationCache {
            termination_status: None,
            passive_control: None,
            pinned_set: None,
            //move_constraints: None,
        }
    }
}

impl BoardImpl {
    pub fn clear(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
        //self.cache.move_constraints = None;
    }
}

