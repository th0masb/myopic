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
        let rights_removed = Board::compute_rights_removed(source | target);
        //let new_rights = self.castling.
        let (piece_taken, hash) = self.pieces.move_piece(source, target);
//        self.active = Side::White;
//        let piece_taken3 = self.active;
        unimplemented!()
    }


    fn compute_enpassant(source: Square, target: Square, piece: &dyn Piece) -> Option<Square> {
        match piece.class() {
            PieceClass::Pawn => {
                let (srank, trank) = (target.rank() as i32, source.rank() as i32);
                if (trank - srank).abs() == 2 {
                   unimplemented!()
                } else {
                    None
                }
            },
            _ => None
        }
    }

    fn compute_rights_removed(move_components: BitBoard) -> CastleZoneSet {
        CastleZone::ALL.iter()
            .filter(|&x| move_components.intersects(x.king_source() | x.rook_source()))
            .collect()
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
