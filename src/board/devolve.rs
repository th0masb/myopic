use crate::base::castlezone::CastleZone;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::Move;
use crate::board::Move::*;
use crate::board::PieceRef;
use crate::board::ReversalData;
use crate::pieces::{BP, Piece, WP};

type RD = ReversalData;

impl Board {
    pub fn devolve(&mut self, action: Move, discards: RD) {
        match action {
            Castle { zone } => self.devolve_c(zone, discards),
            Standard { piece, source, target } => self.devolve_s(piece, source, target, discards),
            Enpassant { source } => self.devolve_e(source, discards),
            Promotion { source, target, piece } => self.devolve_p(source, target, piece, discards),
        }
    }

    fn devolve_p(&mut self, source: Square, target: Square, piece: PieceRef, discards: RD) {
        self.switch_side();
        let moved_pawn = match self.active { Side::White => WP, _ => BP, };
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(piece, &[target]);
        match discards.discarded_piece {
            Some(p) => self.pieces.toggle_piece(p, &[target]),
            _ => (),
        };
        self.devolve_meta(discards);
    }

    fn devolve_e(&mut self, source: Square, discards: RD) {
        self.switch_side();
        let active = self.active;
        let (active_pawn, passive_pawn) = match active { Side::White => (WP, BP), _ => (BP, WP), };
        let enpassant_square = discards.discarded_enpassant.unwrap();
        let removal_square = enpassant_square.next(active.pawn_dir().opposite()).unwrap();
        self.pieces.toggle_piece(active_pawn, &[source, enpassant_square]);
        self.pieces.toggle_piece(passive_pawn, &[removal_square]);
        self.devolve_meta(discards);
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

    fn devolve_c(&mut self, zone: CastleZone, discards: RD) {
        self.switch_side();
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
        self.castling.clear_status(self.active);
        self.devolve_meta(discards);
    }

    fn devolve_meta(&mut self, discards: RD) {
        self.castling.add_rights(discards.discarded_rights);
        self.hashes.pop_head(discards.discarded_hash);
        self.enpassant = discards.discarded_enpassant;
        self.clock = discards.discarded_clock;
    }
}
