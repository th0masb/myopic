use myopic_core::{Reflectable, Square};
use myopic_core::anyhow::{anyhow, Result};

use crate::imp::rights::Rights;
use crate::Move;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Discards {
    pub rights: Rights,
    pub enpassant: Option<Square>,
    pub half_move_clock: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct History {
    /// Number of previous positions before we started
    /// making moves on this board
    prev_position_count: usize,
    /// The stack which tracks the moves and positional
    /// information which gets lost when you make/unmake
    /// moves
    inner: Vec<(Move, Discards)>,
}

impl History {
    pub fn new(prev_position_count: usize) -> History {
        History {
            prev_position_count,
            inner: Vec::new(),
        }
    }

    pub fn position_count(&self) -> usize {
        self.prev_position_count + self.inner.len()
    }

    pub fn push(&mut self, mv: Move, discards: Discards) {
        self.inner.push((mv, discards));
    }

    pub fn historical_positions(&self) -> impl Iterator<Item = u64> + '_ {
        self.inner.iter().map(|(m, _)| m.source())
    }

    pub fn attempt_pop(&mut self) -> Result<(Move, Discards)> {
        self.inner
            .pop()
            .ok_or(anyhow!("Empty history, could not pop last move!"))
    }

    pub(crate) fn reflect_for(&self, new_hash: u64) -> History {
        History {
            prev_position_count: self.prev_position_count,
            inner: self
                .inner
                .iter()
                .map(|(m, d)| {
                    (
                        m.reflect_for(new_hash),
                        Discards {
                            rights: d.rights.reflect(),
                            enpassant: d.enpassant.reflect(),
                            half_move_clock: d.half_move_clock,
                        },
                    )
                })
                .collect(),
        }
    }
}
