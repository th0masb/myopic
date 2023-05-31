use enum_map::EnumMap;
use myopic_board::{ChessBoard, Move};
use crate::{CastleZone, Side};
use crate::eval::EvalComponent;

#[derive(Default, Eq, PartialEq)]
pub struct CastlingEvalComponent {
    castling_status: EnumMap<Side, Option<CastleZone>>,
}

impl CastlingEvalComponent {
    fn penalty<B : ChessBoard>(&self, side: Side, board: &B) -> i32 {
        if self.castling_status[side].is_some() {
            0
        } else {
            let rights_remaining = board.remaining_rights()
                .iter().filter(|z| z.side() == side).count();
            ((2 - rights_remaining) as i32) * -100
        }
    }
}

impl <B: ChessBoard> EvalComponent<B> for CastlingEvalComponent {
    fn static_eval(&self, board: &B) -> i32 {
        self.penalty(Side::White, board) - self.penalty(Side::Black, board)
    }

    fn make(&mut self, mv: &Move) {
        match mv {
            Move::Castle { zone, .. } => self.castling_status[zone.side()] = Some(*zone),
            _ => ()
        }
    }

    fn unmake(&mut self, mv: &Move) {
        match mv {
            Move::Castle { zone, .. } => self.castling_status[zone.side()] = None,
            _ => ()
        }
    }
}