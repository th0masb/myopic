use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::{
    castletracker::CastleTracker, hashcache::HashCache, piecetracker::PieceTracker,
};
use crate::board::Board;
use crate::board::Move;
use crate::board::ReversalData;
use crate::board::MoveComputationType;
use crate::base::castlezone::CastleZone;
use crate::pieces::Piece;
use crate::base::bitboard::BitBoard;

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

impl Board for BoardImpl {
    fn evolve(&mut self, action: &Move) -> ReversalData {
        self.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: ReversalData) {
        self.devolve(action, discards)
    }

    fn compute_moves(&self, computation_type: MoveComputationType) -> Vec<Move> {
        unimplemented!()
    }

    fn hash(&self) -> u64 {
        unimplemented!()
    }

    fn active(&self) -> Side {
        unimplemented!()
    }

    fn enpassant_square(&self) -> Option<Square> {
        unimplemented!()
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        unimplemented!()
    }

    fn piece_locations(&self, piece: Piece) -> BitBoard {
        unimplemented!()
    }

    fn king_location(&self, side: Side) -> Square {
        unimplemented!()
    }

    fn whites_blacks(&self) -> (BitBoard, BitBoard) {
        unimplemented!()
    }

    fn piece_at(&self, location: Square) -> Option<Piece> {
        unimplemented!()
    }

    fn half_move_clock(&self) -> usize {
        unimplemented!()
    }

    fn game_counter(&self) -> usize {
        unimplemented!()
    }
}

impl Reflectable for BoardImpl {
    fn reflect(&self) -> Self {
        unimplemented!()
    }
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

//    fn king_locations(&self) -> (Square, Square) {
//        let (active, passive) = (self.active, self.active.reflect());
//        (self.pieces.king_location(active), self.pieces.king_location(passive))
//    }
}
