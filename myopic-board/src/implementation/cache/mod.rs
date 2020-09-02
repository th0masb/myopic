pub use constraints::MoveConstraints;

use crate::implementation::cache::rays::RaySet;
use crate::MutBoardImpl;
use crate::Termination;
use myopic_core::bitboard::BitBoard;

mod constraints;
mod control;
mod rays;
mod termination;

// TODO Can we switch to returning references from the cache?
#[derive(Debug, Clone)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<RaySet>,
    move_constraints: Option<MoveConstraints>,
}

impl CalculationCache {
    pub fn empty() -> CalculationCache {
        CalculationCache {
            termination_status: None,
            passive_control: None,
            pinned_set: None,
            move_constraints: None,
        }
    }
}

impl MutBoardImpl {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
        self.cache.move_constraints = None;
    }
}
