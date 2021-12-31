use myopic_core::*;

use crate::Board;
use crate::ChessBoard;
use crate::MoveComputeType;
use crate::TerminalState;

impl Board {
    pub fn terminal_state(&self) -> Option<TerminalState> {
        let cache = self.cache.borrow();
        let terminal_status = cache.termination_status;
        drop(cache);
        match terminal_status {
            Some(x) => x,
            None => {
                let result = self.compute_terminal_state();
                self.cache.borrow_mut().termination_status = Some(result);
                result
            }
        }
    }

    fn compute_terminal_state(&self) -> Option<TerminalState> {
        if self.half_move_clock() >= 50 || self.history.has_three_repetitions() {
            return Some(TerminalState::Draw);
        }
        let active = self.active;
        let active_king = self.king(active);
        let passive_control = self.passive_control();
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

    /// Assumes king is in check and cannot move out of it
    fn checked_termination(&self) -> Option<TerminalState> {
        let constraints = self.move_constraints(MoveComputeType::All);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks) & constraints.get(loc);
        for &piece in qrbnp(self.active) {
            let locations = self.locs(&[piece]);
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        // Mated
        return Some(TerminalState::Loss);
    }

    /// Assumes king cannot move but not in check
    fn unchecked_termination(&self) -> Option<TerminalState> {
        let king = self.king(self.active);
        let pin_rays = Piece::WQ.control(king, BitBoard::EMPTY, BitBoard::EMPTY);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // These pieces have no constraints since not in check and not on pin rays
        for &piece in qrbnp(self.active) {
            let locations = self.locs(&[piece]) - pin_rays;
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        // Compute constraints as a last resort
        let constraints = self.move_constraints(MoveComputeType::All);
        let moves2 = |p: Piece, loc: Square| p.moves(loc, whites, blacks) & constraints.get(loc);
        for &piece in qrbnp(self.active) {
            let locations = self.locs(&[piece]) & pin_rays;
            if locations
                .iter()
                .any(|loc| moves2(piece, loc).is_populated())
            {
                return None;
            }
        }
        // Stalemate
        return Some(TerminalState::Draw);
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
    use myopic_core::Reflectable;

    use super::*;

    #[derive(Clone, Debug)]
    struct TestCase {
        board: Board,
        expected: Option<TerminalState>,
    }

    fn test(expected: Option<TerminalState>, fen: &str) {
        let mut board = fen.parse::<Board>().unwrap();
        assert_eq!(expected, board.terminal_state());
        assert_eq!(expected, board.reflect().terminal_state());
    }

    #[test]
    fn checkmate() {
        test(
            Some(TerminalState::Loss),
            "5R1k/pp2R2p/8/1b2r3/3p3q/8/PPB3P1/6K1 b - - 0 36",
        )
    }

    #[test]
    fn not_terminal() {
        test(
            None,
            "r1b1qrk1/pp5p/1np2b2/3nNP2/3P2p1/1BN5/PP1BQ1P1/4RRK1 b - - 0 18",
        );
    }

    #[test]
    fn not_terminal2() {
        test(None, "4R3/1p4rk/6p1/2p1BpP1/p1P1pP2/P7/1P6/K2Q4 b - - 0 2");
    }

    #[test]
    fn stalemate() {
        test(
            Some(TerminalState::Draw),
            "6k1/6p1/7p/8/1p6/p1qp4/8/3K4 w - - 0 45",
        );
    }
}
