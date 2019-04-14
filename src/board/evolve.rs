use crate::base::square::Square;
use crate::board::Board;
use crate::board::Move;
use crate::board::ReversalData;
use crate::pieces::PieceClass;
use crate::base::castlezone::CastleZone;
use crate::board::Move::Standard;
use crate::board::Move::Enpassant;
use crate::board::Move::Promotion;
use crate::board::Move::Castle;

type RD = ReversalData;

impl Board {
    pub fn evolve(&mut self, action: Move) -> RD {
        match action {
            Standard {source, target} => self.standard_evolve(source, target),
            Enpassant {source, target} => self.enpassant_evolve(source, target),
            Promotion {source, target, piece} => self.promotion_evolve(source, target, piece),
            Castle {zone} => self.castle_evolve(zone),
        }
    }

    fn standard_evolve(&mut self, source: Square, target: Square) -> RD {
        unimplemented!()
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
