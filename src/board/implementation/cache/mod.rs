use crate::board::Termination;
use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::pinning::PinnedSet;

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

    pub fn termination_status(&mut self) -> Option<Option<Termination>> {
        unimplemented!()
//        if self.termination_status.is_some() {
//            self.termination_status.unwrap()
//        } else {
//            self.termination_status =
//        }
    }
}

