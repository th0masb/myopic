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
use crate::board::hash;
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
        let (moved_piece, discarded_piece) = self.pieces.move_piece(source, target);
        let discarded_rights = self.castling.remove_rights(source | target);
        let discarded_enpassant = self.enpassant;
        let next_enpassant = Board::compute_enpassant(source, target, moved_piece);
        self.enpassant = next_enpassant;
        self.active = self.active.other();
        let discarded_hash = self.update_hash();

        ReversalData {
            discarded_rights,
            discarded_piece,//: discarded_piece.map(|x| x.class()),
            discarded_enpassant,
            discarded_hash
        }
    }

    fn update_hash(&mut self) -> u64 {
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
