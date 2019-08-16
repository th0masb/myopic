use crate::board::BoardImpl;
use crate::board::Termination;
use crate::board::Board;
use crate::board::implementation::cache::control;
use crate::base::Reflectable;
use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::MoveComputeType;


impl BoardImpl {
    pub fn termination_status_impl(&mut self) -> Option<Termination> {
        match &self.cache.termination_status {
            Some(x) => *x,
            None => {
                let result = self.compute_termination();
                self.cache.termination_status = Some(result);
                result
            }
        }
    }

    fn compute_termination(&mut self) -> Option<Termination> {
        if self.half_move_clock() >= 50 || self.history.has_three_repetitions() {
            return Some(Termination::Draw);
        }
        let (active, passive) = (self.active, self.active.reflect());
        let active_king = self.king(active);
        let passive_control = self.passive_control_impl();
        let (whites, blacks) = self.sides();
        // If king can move somewhere which is usually the case then not terminal.
        let king_moves = Piece::king(active).moves(active_king, whites, blacks);
        if !(king_moves - passive_control).is_empty() {
            None
        } else if passive_control.contains(active_king) {
            self.checked_termination(passive_control, active_king)
        } else {
            self.unchecked_termination(passive_control, active_king)
        }
    }

    fn checked_termination(&mut self, passive_ctrl: BitBoard, king: Square) -> Option<Termination> {
        let constraints = self.constraints(MoveComputeType::All);
        unimplemented!()

    }

    fn unchecked_termination(&mut self, passive_ctrl: BitBoard, king: Square) -> Option<Termination> {
        unimplemented!()
    }
}