use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::castletracker::CastleTracker;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::pieces;
use crate::pieces::Piece;

pub mod hash;
//pub mod tables;// To be removed
pub mod evolve;
pub mod moves;

mod castletracker;
mod hashcache;
mod piecetracker;

#[cfg(test)]
mod testutils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    hashes: HashCache,
    pieces: PieceTracker,
    castling: CastleTracker,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
}

impl Board {
    fn switch_side(&mut self) {
        self.active = self.active.other();
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

#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<Piece>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
    discarded_clock: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Standard(Piece, Square, Square),
    Enpassant(Square),
    Promotion(Square, Square, Piece),
    Castle(CastleZone),
}

fn promotion_targets<'a>(side: Side) -> &'a [Piece; 4] {
    match side {
        Side::White => &[pieces::WQ, pieces::WR, pieces::WB, pieces::WN],
        Side::Black => &[pieces::BQ, pieces::BR, pieces::BB, pieces::BN],
    }
}


impl Move {
    pub fn standards(
        moving_piece: Piece,
        source: Square,
        targets: BitBoard,
    ) -> impl Iterator<Item = Move> {
        targets
            .into_iter()
            .map(move |target| Move::Standard(moving_piece, source, target))
    }

    pub fn enpassant(source: Square) -> Move {
        Move::Enpassant(source)
    }

    pub fn promotions(side: Side, source: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().flat_map(move |target| {
            promotion_targets(side)
                .iter()
                .map(move |&piece| Move::Promotion(source, target, piece))
        })
    }

    pub fn castle(zone: CastleZone) -> Move {
        Move::Castle(zone)
    }
}
