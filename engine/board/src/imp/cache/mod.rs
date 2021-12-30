use std::rc::Rc;
pub use constraints::MoveConstraints;
use myopic_core::BitBoard;
use myopic_core::enum_map::EnumMap;

use crate::{Board, MoveComputeType};
use crate::imp::cache::rays::RaySet;
use crate::Termination;

mod constraints;
mod control;
mod rays;
mod termination;

#[derive(Debug, Clone, Default)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<Rc<RaySet>>,
    move_constraints: EnumMap<MoveComputeType, Option<Rc<MoveConstraints>>>,
}

impl Board {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
        self.cache.move_constraints = EnumMap::default();
    }
}
