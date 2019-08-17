use crate::board::BoardImpl;
use crate::board::Termination;
use crate::board::Board;
use crate::board::implementation::cache::control;
use crate::base::Reflectable;
use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::MoveComputeType;
use crate::base::Side;


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
            self.checked_termination()
        } else {
            self.unchecked_termination()
        }
    }

    /// Assumes king cannot move
    fn checked_termination(&mut self) -> Option<Termination> {
        let constraints = self.constraints(MoveComputeType::All);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks) & constraints.get(loc);
        for &piece in qrbnp(self.active) {
            let locations = self.locs(piece);
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        // Mated
        return Some(Termination::Loss);
    }

    /// Assumes king cannot move
    fn unchecked_termination(&mut self) -> Option<Termination> {
        let king = self.king(self.active);
        let pin_rays = Piece::WQ.control(king, BitBoard::EMPTY, BitBoard::EMPTY);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // These pieces have no constraints since not in check and not on pin rays
        for &piece in qrbnp(self.active) {
            let locations = self.locs(piece) - pin_rays;
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        // Compute constraints as a last resort
        let constraints = self.constraints(MoveComputeType::All);
        let moves2 = |p: Piece, loc: Square| p.moves(loc, whites, blacks) - constraints.get(loc);
        for &piece in qrbnp(self.active) {
            let locations = self.locs(piece) & pin_rays;
            if locations.iter().any(|loc| moves2(piece, loc).is_populated()) {
                return None;
            }
        }
        // Stalemate
        return Some(Termination::Draw);
    }
}

fn qrbnp<'a>(side: Side) -> &'a [Piece] {
    match side {
        Side::White => &[Piece::WQ, Piece::WR, Piece::WB, Piece::WN, Piece::WP],
        Side::Black => &[Piece::BQ, Piece::BR, Piece::BB, Piece::BN, Piece::BP],
    }
}