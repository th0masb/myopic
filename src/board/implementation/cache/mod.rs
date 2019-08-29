pub use constraints::MoveConstraints;

use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::rays::RaySet;
use crate::board::BoardImpl;
use crate::board::Termination;

mod constraints;
mod control;
mod rays;
mod termination;

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl BoardImpl {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
        self.cache.move_constraints = None;
    }
}
