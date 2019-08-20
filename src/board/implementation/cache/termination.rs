use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Board;
use crate::board::BoardImpl;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::pieces::Piece;

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
        let active = self.active;
        let active_king = self.king(active);
        let passive_control = self.passive_control_impl();
        let (whites, blacks) = self.sides();
        // If king can move somewhere which is usually the case then not terminal.
        let king_moves = Piece::king(active).moves(active_king, whites, blacks);
        if (king_moves - passive_control).is_populated() {
            None
        } else if passive_control.contains(active_king) {
            self.checked_termination()
        } else {
            self.unchecked_termination()
        }
    }

    /// Assumes king cannot move
    fn checked_termination(&mut self) -> Option<Termination> {
        let constraints = self.constraints_impl(MoveComputeType::All);
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
        let constraints = self.constraints_impl(MoveComputeType::All);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::base::Reflectable;

    #[derive(Clone, Debug)]
    struct TestCase {
        board: BoardImpl,
        expected: Option<Termination>,
    }

    fn test(expected: Option<Termination>, fen: &str) {
        let mut board = crate::board::from_fen(fen).unwrap();
        assert_eq!(expected, board.termination_status_impl());
        assert_eq!(expected, board.reflect().termination_status_impl());
    }

    #[test]
    fn checkmate() {
        test(Some(Termination::Loss), "5R1k/pp2R2p/8/1b2r3/3p3q/8/PPB3P1/6K1 b - - 0 36")
    }

    #[test]
    fn not_terminal() {
        test(None, "r1b1qrk1/pp5p/1np2b2/3nNP2/3P2p1/1BN5/PP1BQ1P1/4RRK1 b - - 0 18");
    }

    #[test]
    fn stalemate() {
        test(Some(Termination::Draw), "6k1/6p1/7p/8/1p6/p1qp4/8/3K4 w - - 0 45");
    }
}
