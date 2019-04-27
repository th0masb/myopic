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
use crate::board::PieceRef;
use crate::board::ReversalData;
use crate::pieces::BP;
use crate::pieces::WP;

type RD = ReversalData;

impl Board {
    pub fn evolve(&mut self, action: Move) -> RD {
        match action {
            Standard { piece, source, target } => self.evolve_s(piece, source, target),
            _ => unimplemented!(),
//            Castle { zone } => self.evolve_c(zone),
//            Enpassant { source } => self.evolve_e(source),
//            Promotion { source, target, piece } => self.evolve_p(source, target, piece),
        }
    }

    pub fn devolve(&mut self, action: Move, discards: RD) {
        match action {
            Standard { piece, source, target } => self.devolve_s(piece, source, target, discards),
            _ => unimplemented!(),
//            Castle { zone } => self.devolve_c(zone, discards),
//            Enpassant { source } => self.devolve_e(source, discards),
//            Promotion { source, target, piece } => self.devolve_p(source, target, piece, discards),
        }
    }

    fn evolve_s(&mut self, piece: PieceRef, source: Square, target: Square) -> RD {
        let discarded_piece = self.pieces.erase_square(target);
        let discarded_rights = self.castling.remove_rights(source | target);
        let rev_data = self.create_rev_data(discarded_piece, discarded_rights);
        self.pieces.toggle_piece(piece, &[source, target]);
        self.clock = if discarded_piece.is_some() || piece.is_pawn() {self.clock + 1} else {0};
        self.enpassant = Board::compute_enpassant(source, target, piece);
        self.switch_side_and_update_hash();
        rev_data
    }

    fn switch_side_and_update_hash(&mut self) {
        self.switch_side();
        self.update_hash();
    }

    fn create_rev_data(&self, discarded_piece: Option<PieceRef>, discarded_rights: CastleZoneSet) -> RD {
        ReversalData {
            discarded_piece,
            discarded_rights,
            discarded_enpassant: self.enpassant,
            discarded_hash: self.hashes.tail(),
            discarded_clock: self.clock,
        }
    }

    fn devolve_s(&mut self, piece: PieceRef, source: Square, target: Square, discards: RD) {
        self.switch_side();
        self.pieces.toggle_piece(piece, &[target, source]);
        match discards.discarded_piece {
            Some(discarded) => self.pieces.toggle_piece(discarded, &[target]),
            _ => (),
        };
        self.devolve_meta(discards);
    }


//    fn evolve_c(&mut self, zone: CastleZone) -> RD {
//        let discarded_rights = self.castling.set_status(self.active, zone);
//        let rev_data = self.rev_data(None, discarded_rights);
//        self.toggle_castle_pieces(zone);
//        let discarded_enpassant = self.enpassant;
//        let discarded_clock = self.clock;
//        self.enpassant = None;
//        self.clock += 1;
//        self.switch_side();
//        let discarded_hash = self.update_hash();
//
//        ReversalData {
//            discarded_piece: None,
//            discarded_enpassant,
//            discarded_rights,
//            discarded_hash,
//            discarded_clock,
//        }
//    }
//
//    fn devolve_c(&mut self, zone: CastleZone, discards: RD) {
//        self.switch_side();
//        self.toggle_castle_pieces(zone);
//        self.castling.clear_status(self.active);
//        self.devolve_meta(discards);
//    }
//
//    fn toggle_castle_pieces(&mut self, zone: CastleZone) {
//        let (rook, r_source, r_target) = zone.rook_data();
//        let (king, k_source, k_target) = zone.king_data();
//        self.pieces.toggle_piece(rook, &[r_source, r_target]);
//        self.pieces.toggle_piece(king, &[k_source, k_target]);
//    }
//
//
//    fn evolve_e(&mut self, source: Square) -> RD {
//        let enpassant_square = self.enpassant.unwrap();
//        self.toggle_enpassant_pieces(source, enpassant_square);
//        self.enpassant = None;
//        let discarded_clock = self.clock;
//        self.clock = 0;
//        self.switch_side();
//        let discarded_hash = self.update_hash();
//
//        ReversalData {
//            discarded_piece: Some(match self.active {Side::White => WP, _ => BP}),
//            discarded_hash,
//            discarded_rights: CastleZoneSet::none(),
//            discarded_enpassant: Some(enpassant_square),
//            discarded_clock,
//        }
//    }
//
//    fn devolve_e(&mut self, source: Square, discards: RD) {
//        self.switch_side();
//        self.toggle_enpassant_pieces(source, discards.discarded_enpassant.unwrap());
//        self.devolve_meta(discards);
//    }
//
//    fn toggle_enpassant_pieces(&mut self, source: Square, enpassant: Square) {
//        let active = self.active;
//        let (active_pawn, passive_pawn) = match active { Side::White => (WP, BP), _ => (BP, WP), };
//        let removal_square = enpassant.next(active.pawn_dir().opposite()).unwrap();
//        self.pieces.toggle_piece(active_pawn, &[source, enpassant]);
//        self.pieces.toggle_piece(passive_pawn, &[removal_square]);
//    }
//
//
//    fn evolve_p(&mut self, source: Square, target: Square, promotion_result: PieceRef) -> RD {
//        let discarded_piece = self.pieces.erase_square(target);
//        let moved_pawn = match self.active {
//            Side::White => WP,
//            Side::Black => BP,
//        };
//        self.pieces.toggle_piece(moved_pawn, &[source]);
//        self.pieces.toggle_piece(promotion_result, &[target]);
//        let discarded_enpassant = self.enpassant;
//        self.enpassant = None;
//        let discarded_clock = self.clock;
//        self.clock = 0;
//        self.switch_side();
//        let discarded_hash = self.update_hash();
//
//        ReversalData {
//            discarded_piece,
//            discarded_enpassant,
//            discarded_hash,
//            discarded_rights: CastleZoneSet::none(),
//            discarded_clock,
//        }
//    }
//
//    fn devolve_p(&mut self, source: Square, target: Square, piece: PieceRef, discards: RD) {
//        self.switch_side();
//        let moved_pawn = match self.active { Side::White => WP, _ => BP, };
//        self.pieces.toggle_piece(moved_pawn, &[source]);
//        self.pieces.toggle_piece(piece, &[target]);
//        match discards.discarded_piece {
//            Some(p) => self.pieces.toggle_piece(p, &[target]),
//            _ => (),
//        };
//        self.devolve_meta(discards);
//    }

    fn devolve_meta(&mut self, discards: RD) {
        self.castling.add_rights(discards.discarded_rights);
        self.hashes.pop_head(discards.discarded_hash);
        self.enpassant = discards.discarded_enpassant;
        self.clock = discards.discarded_clock;
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
