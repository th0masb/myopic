use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::board::castletracker::CastleTracker;
use crate::base::Side;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;

pub mod hash;
pub mod tables;// To be removed
pub mod evolve;

mod piecetracker;
mod castletracker;
mod hashcache;



pub type PieceRef = &'static dyn Piece;

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<PieceRef>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
    discarded_clock: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Standard {piece: PieceRef, source: Square, target: Square},
    Enpassant {source: Square},
    Promotion {source: Square, target: Square, piece: PieceRef},
    Castle {zone: CastleZone}
}

impl Move {
    pub fn standard(moving_piece: PieceRef, source: Square, target: Square) -> Move {
        Move::Standard {piece: moving_piece, source, target}
    }

    pub fn enpassant(source: Square) -> Move {
        Move::Enpassant {source}
    }

    pub fn promotion(source: Square, target: Square, piece: PieceRef) -> Move {
        Move::Promotion {source, target, piece}
    }

    pub fn castle(zone: CastleZone) -> Move {
        Move::Castle {zone}
    }
}


