use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Side;
use crate::base::square::constants::A1;
use crate::base::square::constants::A8;
use crate::base::square::constants::E1;
use crate::base::square::constants::E8;
use crate::base::square::constants::H1;
use crate::base::square::constants::H8;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::hash;
use crate::board::Move;
use crate::board::Move::*;
use crate::board::ReversalData;
use crate::pieces::Piece;
use crate::base::dir::N;
use crate::base::dir::S;

type RD = ReversalData;

impl Board {
    pub fn evolve(&mut self, action: Move) -> RD {
        match action {
            Castle { zone } => self.castle_evolve(zone),
            Standard { source, target } => self.standard_evolve(source, target),
            Enpassant { source, target } => self.enpassant_evolve(source, target),
            Promotion {
                source,
                target,
                piece,
            } => self.promotion_evolve(source, target, piece),
        }
    }

    fn standard_evolve(&mut self, source: Square, target: Square) -> RD {
        let (moved_piece, discarded_piece) = self.pieces.move_piece(source, target);
        let discarded_rights = self.castling.remove_rights(source | target);
        let discarded_enpassant = self.enpassant;
        self.enpassant = Board::compute_enpassant(source, target, moved_piece);
        self.active = self.active.other();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_rights,
            discarded_piece,
            discarded_enpassant,
            discarded_hash,
        }
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache, returning the overwritten value.
    fn update_hash(&mut self) -> u64 {
        let next_hash = self.pieces.hash()
            ^ self.castling.hash()
            ^ hash::side_feature(self.active)
            ^ self.enpassant.map_or(0u64, |x| hash::enpassant_feature(x));
        self.hashes.push_head(next_hash)
    }

    /// Determines the enpassant square for the next board state given a
    /// piece which has just moved from the source to the target.
    fn compute_enpassant(source: Square, target: Square, piece: &dyn Piece) -> Option<Square> {
        if piece.index() % 6 == 0 { // piece is a pawn
            let is_white = piece.side() == Side::White;
            let start_rank = BitBoard::RANKS[if is_white {1} else {6}];
            let ep_rank = BitBoard::RANKS[if is_white {3} else {4}];
            if start_rank.contains(source) && ep_rank.contains(target) {
                source.next(if is_white {N} else {S})
            } else {
                None
            }
        } else {
            None
        }
    }

    fn enpassant_evolve(&mut self, source: Square, target: Square) -> RD {
        unimplemented!()
    }

    fn promotion_evolve(&mut self, source: Square, target: Square, piece: &dyn Piece) -> RD {
        unimplemented!()
    }

    fn castle_evolve(&mut self, zone: CastleZone) -> RD {
        unimplemented!()
    }
}
