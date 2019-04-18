use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::constants::A1;
use crate::base::square::constants::A8;
use crate::base::square::constants::E1;
use crate::base::square::constants::E8;
use crate::base::square::constants::H1;
use crate::base::square::constants::H8;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::Move;
use crate::board::Move::Castle;
use crate::board::Move::Enpassant;
use crate::board::Move::Promotion;
use crate::board::Move::Standard;
use crate::board::ReversalData;
use crate::pieces::PieceClass;
use crate::pieces::Piece;
use crate::base::Side;

type RD = ReversalData;

impl Board {
    pub fn evolve(&mut self, action: Move) -> RD {
        match action {
            Standard { source, target } => self.standard_evolve(source, target),
            Enpassant { source, target } => self.enpassant_evolve(source, target),
            Promotion { source, target, piece, } => self.promotion_evolve(source, target, piece),
            Castle { zone } => self.castle_evolve(zone),
        }
    }

    fn standard_evolve(&mut self, source: Square, target: Square) -> RD {
        let (rights_removed, rights_hash) = self.castling.remove_rights(source | target);
        let (piece_taken, piece_hash) = self.pieces.move_piece(source, target);
        self.active = self.active.other();
        let side_hash = crate::board::hash::side_feature(self.active);
//        self.active = Side::White;
//        let piece_taken3 = self.active;
        unimplemented!()
    }




    fn compute_enpassant(source: Square, target: Square, piece: &dyn Piece) -> (Option<Square>, u64) {
        match piece.class() {
            PieceClass::Pawn => {
                let (srank, trank) = (target.rank() as i32, source.rank() as i32);
                if (trank - srank).abs() == 2 {
                   unimplemented!()
                } else {
                    (None, 0u64)
                }
            },
            _ => (None, 0u64)
        }
    }

    fn enpassant_evolve(&mut self, source: Square, target: Square) -> RD {
        unimplemented!()
    }

    fn promotion_evolve(&mut self, source: Square, target: Square, piece: PieceClass) -> RD {
        unimplemented!()
    }

    fn castle_evolve(&mut self, zone: CastleZone) -> RD {
        unimplemented!()
    }
}
