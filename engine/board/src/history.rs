use myopic_core::anyhow::{anyhow, Result};
use myopic_core::Square;

use crate::rights::Rights;
use crate::Move;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Discards {
    pub rights: Rights,
    pub enpassant: Option<Square>,
    pub half_move_clock: usize,
    pub hash: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct History {
    /// Number of previous positions before we started
    /// making moves on this board
    prev_position_count: usize,
    /// The stack which tracks the moves and positional
    /// information which gets lost when you make/unmake
    /// moves
    inner: Vec<(Discards, Move)>,
}

impl History {
    pub fn new(prev_position_count: usize) -> History {
        History { prev_position_count, inner: Vec::new() }
    }

    pub fn position_count(&self) -> usize {
        self.prev_position_count + self.inner.len()
    }

    pub fn push(&mut self, discards: Discards, m: Move) {
        self.inner.push((discards, m));
    }

    pub fn historical_moves(&self) -> impl DoubleEndedIterator<Item = Move> + '_ {
        self.inner.iter().map(|(_, m)| m.clone())
    }

    pub fn historical_positions(&self) -> impl DoubleEndedIterator<Item = (u64, &Move)> + '_ {
        self.inner.iter().map(|(d, m)| (d.hash, m))
    }

    pub fn attempt_pop(&mut self) -> Result<(Discards, Move)> {
        self.inner.pop().ok_or(anyhow!("Empty history, could not pop last move!"))
    }
}
