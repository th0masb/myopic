use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::board::castletracker::CastleTracker;
use crate::base::Side;
use crate::base::castlezone::CastleZone;
use crate::pieces::PieceClass;
use crate::board::Move::Standard;
use crate::board::Move::Enpassant;
use crate::board::Move::Promotion;
use crate::board::Move::Castle;

pub mod hash;
pub mod tables;
mod piecetracker;
mod castletracker;
mod hashcache;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Board {
    hashes: HashCache,
    pieces: PieceTracker,
    castling: CastleTracker,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
}

pub enum Move {
    Standard {source: Square, target: Square},
    Enpassant {source: Square, target: Square},
    Promotion {source: Square, target: Square, piece: PieceClass},
    Castle {zone: CastleZone}
}

impl Move {
    pub fn standard(source: Square, target: Square) -> Move {
        Standard {source, target}
    }

    pub fn enpassant(source: Square, target: Square) -> Move {
        Enpassant {source, target}
    }

    pub fn promotion(source: Square, target: Square, piece: PieceClass) -> Move {
        Promotion {source, target, piece}
    }

    pub fn castle(zone: CastleZone) -> Move {
        Castle {zone}
    }
}


impl Board {
    pub fn evolve(&mut self, action: Move)  {
        unimplemented!()
    }
}
