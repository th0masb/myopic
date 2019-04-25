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
pub mod tables;
pub mod evolve;
pub mod devolve;
mod piecetracker;
mod castletracker;
mod hashcache;


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
    //pub fn compute
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<&'static dyn Piece>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
    discarded_clock: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Standard {source: Square, target: Square},
    Enpassant {source: Square},
    Promotion {source: Square, target: Square, piece: &'static dyn Piece},
    Castle {zone: CastleZone}
}

impl Move {
    pub fn standard(source: Square, target: Square) -> Move {
        Move::Standard {source, target}
    }

    pub fn enpassant(source: Square) -> Move {
        Move::Enpassant {source}
    }

    pub fn promotion(source: Square, target: Square, piece: &'static dyn Piece) -> Move {
        Move::Promotion {source, target, piece}
    }

    pub fn castle(zone: CastleZone) -> Move {
        Move::Castle {zone}
    }
}


