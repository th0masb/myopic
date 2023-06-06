pub use constraints::MoveConstraints;
use myopic_core::enum_map::EnumMap;
use myopic_core::BitBoard;

use crate::private::cache::rays::RaySet;
use crate::TerminalState;
use crate::{Board, MoveComputeType};

mod constraints;
mod control;
mod rays;
mod termination;

#[derive(Debug, Clone, Default)]
pub struct CalculationCache {
    termination_status: Option<Option<TerminalState>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<RaySet>,
    move_constraints: EnumMap<MoveComputeType, Option<MoveConstraints>>,
}

impl Board {
    pub fn clear_cache(&self) {
        let mut cache = self.cache.borrow_mut();
        cache.termination_status = None;
        cache.passive_control = None;
        cache.pinned_set = None;
        cache.move_constraints = EnumMap::default();
    }
}
