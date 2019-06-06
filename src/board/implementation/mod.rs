use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::{
    castletracker::CastleTracker, hashcache::HashCache, piecetracker::PieceTracker,
};

pub mod evolve;
pub mod hash;
pub mod moves;

mod castletracker;
mod hashcache;
mod piecetracker;

#[cfg(test)]
mod testutils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardImpl {
    hashes: HashCache,
    pieces: PieceTracker,
    castling: CastleTracker,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
}

impl BoardImpl {
    fn switch_side(&mut self) {
        self.active = self.active.reflect();
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache, returning the overwritten value.
    fn update_hash(&mut self) {
        let next_hash = self.pieces.hash()
            ^ self.castling.hash()
            ^ hash::side_feature(self.active)
            ^ self.enpassant.map_or(0u64, |x| hash::enpassant_feature(x));
        self.hashes.push_head(next_hash)
    }
}
