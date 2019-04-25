use std::io::repeat;

use crate::{pieces, pieces::Piece};
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::dir::N;
use crate::base::dir::S;
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
use crate::board::PieceRef;
use crate::pieces::WP;
use crate::pieces::BP;

type RD = ReversalData;

impl Board {
    pub fn evolve(&mut self, action: Move) -> RD {
        match action {
            Castle { zone } => self.evolve_c(zone),
            Standard { piece, source, target } => self.evolve_s(piece, source, target),
            Enpassant { source } => self.evolve_e(source),
            Promotion { source, target, piece } => self.evolve_p(source, target, piece),
        }
    }

    fn evolve_s(&mut self, piece: PieceRef, source: Square, target: Square) -> RD {
        let discarded_piece = self.pieces.erase_square(target);
        self.pieces.toggle_piece(piece, &[source, target]);
        let discarded_rights = self.castling.remove_rights(source | target);
        let discarded_enpassant = self.enpassant;
        let discarded_clock = self.clock;
        self.clock = if discarded_piece.is_some() || piece.is_pawn() {self.clock + 1} else {0};
        self.enpassant = Board::compute_enpassant(source, target, piece);
        self.switch_side();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_rights,
            discarded_piece,
            discarded_enpassant,
            discarded_hash,
            discarded_clock
        }
    }

    fn evolve_c(&mut self, zone: CastleZone) -> RD {
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
        let discarded_rights = self.castling.set_status(self.active, zone);
        let discarded_enpassant = self.enpassant;
        let discarded_clock = self.clock;
        self.enpassant = None;
        self.clock += 1;
        self.switch_side();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_piece: None,
            discarded_enpassant,
            discarded_rights,
            discarded_hash,
            discarded_clock,
        }
    }

    fn evolve_e(&mut self, source: Square) -> RD {
        let active = self.active;
        let (active_pawn, passive_pawn) = match active {
            Side::White => (pieces::WP, pieces::BP),
            Side::Black => (pieces::BP, pieces::WP),
        };
        let enpassant_square = self.enpassant.unwrap();
        let removal_square = enpassant_square.next(active.pawn_dir().opposite()).unwrap();
        self.pieces.toggle_piece(active_pawn, &[source, enpassant_square]);
        self.pieces.toggle_piece(passive_pawn, &[removal_square]);
        self.enpassant = None;
        let discarded_clock = self.clock;
        self.clock = 0;
        self.switch_side();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_piece: Some(passive_pawn),
            discarded_hash,
            discarded_rights: CastleZoneSet::none(),
            discarded_enpassant: Some(enpassant_square),
            discarded_clock,
        }
    }

    fn evolve_p(&mut self, source: Square, target: Square, promotion_result: PieceRef) -> RD {
        let discarded_piece = self.pieces.erase_square(target);
        let moved_pawn = match self.active {
            Side::White => WP,
            Side::Black => BP,
        };
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(promotion_result, &[target]);
        let discarded_enpassant = self.enpassant;
        self.enpassant = None;
        let discarded_clock = self.clock;
        self.clock = 0;
        self.switch_side();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_piece,
            discarded_enpassant,
            discarded_hash,
            discarded_rights: CastleZoneSet::none(),
            discarded_clock,
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
    fn compute_enpassant(source: Square, target: Square, piece: PieceRef) -> Option<Square> {
        if piece.is_pawn() {
            let side = piece.side();
            if side.pawn_first_rank().contains(source) && side.pawn_third_rank().contains(target) {
                source.next(side.pawn_dir())
            } else {
                None
            }
        } else {
            None
        }
    }
}
