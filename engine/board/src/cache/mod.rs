pub use constraints::MoveConstraints;
use myopic_core::BitBoard;

use crate::{Move, TerminalState};
use crate::Board;
pub use rays::RaySet;

mod constraints;
mod control;
mod rays;
mod termination;


#[derive(Debug, Clone, Default)]
pub struct CalculationCache {
    pub termination_status: Option<Option<TerminalState>>,
    pub passive_control: Option<BitBoard>,
    pub pinned: Option<RaySet>,
    pub discoveries: Option<RaySet>,
    pub moves: Option<Vec<Move>>,
}

impl Board {
    pub fn clear_cache(&self) {
        let mut cache = self.cache.borrow_mut();
        cache.termination_status = None;
        cache.passive_control = None;
        cache.pinned = None;
        cache.discoveries = None;
        cache.moves = None;
    }
}
