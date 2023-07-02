pub use constraints::MoveConstraints;
use myopic_core::enum_map::EnumMap;
use myopic_core::BitBoard;

use crate::TerminalState;
use crate::Board;
pub use rays::RaySet;

mod constraints;
mod control;
mod rays;
mod termination;


#[derive(Debug, Clone, Default)]
pub struct CalculationCache {
    termination_status: Option<Option<TerminalState>>,
    passive_control: Option<BitBoard>,
    pinned: Option<RaySet>,
    discoveries: Option<RaySet>,
}

impl Board {
    pub fn clear_cache(&self) {
        let mut cache = self.cache.borrow_mut();
        cache.termination_status = None;
        cache.passive_control = None;
        cache.pinned = None;
        cache.discoveries = None;
    }
}
