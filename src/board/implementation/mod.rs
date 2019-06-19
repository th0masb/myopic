use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::implementation::{
    castletracker::CastleTracker, hashcache::HashCache, piecetracker::PieceTracker,
};
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::ReversalData;
use crate::pieces::Piece;

pub mod evolve;
pub mod hash;
pub mod moves;

mod castletracker;
mod hashcache;
mod piecetracker;
#[cfg(test)]
pub mod testutils;

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

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn hash(&self) -> u64 {
        self.hashes.head()
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant_square(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn piece_locations(&self, piece: Piece) -> BitBoard {
        self.pieces.locations(piece)
    }

    fn king_location(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn whites_blacks(&self) -> (BitBoard, BitBoard) {
        (
            self.pieces.side_locations(Side::White),
            self.pieces.side_locations(Side::Black),
        )
    }

    fn piece_at(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn game_counter(&self) -> usize {
        self.hashes.pop_dist()
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
}
