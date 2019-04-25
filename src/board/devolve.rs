use crate::board::Board;
use crate::board::Move;
use crate::board::ReversalData;
use crate::board::Move::*;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::PieceRef;

impl Board {
    pub fn devolve(&mut self, action: Move, discards: ReversalData) {
        match action {
            Castle { zone } => self.devolve_c(zone, discards),
            Standard { piece, source, target } => self.devolve_s(piece, source, target, discards),
            Enpassant { source } => self.devolve_e(source, discards),
            Promotion { source, target, piece } => self.devolve_p(source, target, piece, discards),
        }
    }

    fn devolve_p(&mut self, source: Square, target: Square, piece: PieceRef, discards: ReversalData) {
        unimplemented!()
    }

    fn devolve_e(&mut self, source: Square, discards: ReversalData) {
        unimplemented!()
    }

    fn devolve_s(&mut self, piece: PieceRef, source: Square, target: Square, discards: ReversalData) {
        self.switch_side();
        self.pieces.toggle_piece(piece, &[target, source]);
        match discards.discarded_piece {
            Some(discarded) => self.pieces.toggle_piece(discarded, &[target]),
            _ => (),
        };
        self.castling.add_rights(discards.discarded_rights);
        self.enpassant = discards.discarded_enpassant;
        self.clock = discards.discarded_clock;
        self.hashes.pop_head(discards.discarded_hash);
    }

    fn devolve_c(&mut self, zone: CastleZone, discards: ReversalData) {
        self.switch_side();
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
        self.castling.clear_status(self.active);
        self.castling.add_rights(discards.discarded_rights);
        unimplemented!()
    }
}
