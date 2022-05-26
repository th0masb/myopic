use itertools::Itertools;

use myopic_core::anyhow::{anyhow, Result};
use myopic_core::{Reflectable, Square};

use crate::imp::rights::Rights;
use crate::Move;

const REPETITION_WINDOW: usize = 15;

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

    pub fn has_three_repetitions(&self) -> bool {
        self.inner.len() >= REPETITION_WINDOW && {
            let hashes = self
                .inner
                .iter()
                .map(|(m, _)| m.source())
                .sorted()
                .collect_vec();
            let (mut last, mut count) = (hashes[0], 1);
            for &hash in hashes.iter().skip(1) {
                if hash == last {
                    count += 1;
                    if count == 3 {
                        break;
                    }
                } else {
                    count = 1;
                    last = hash;
                }
            }
            count == 3
        }
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
