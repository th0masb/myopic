use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::board::castletracker::CastleTracker;
use crate::base::Side;
use crate::base::castlezone::CastleZone;
use crate::pieces::PieceClass;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<PieceClass>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Standard {source: Square, target: Square},
    Enpassant {source: Square, target: Square},
    Promotion {source: Square, target: Square, piece: PieceClass},
    Castle {zone: CastleZone}
}

impl Move {
    pub fn standard(source: Square, target: Square) -> Move {
        Move::Standard {source, target}
    }

    pub fn enpassant(source: Square, target: Square) -> Move {
        Move::Enpassant {source, target}
    }

    pub fn promotion(source: Square, target: Square, piece: PieceClass) -> Move {
        Move::Promotion {source, target, piece}
    }

    pub fn castle(zone: CastleZone) -> Move {
        Move::Castle {zone}
    }
}


