use crate::eval::EvalBoard;
use crate::{eval, tables, values};
use myopic_board::{Discards, Move, MoveComputeType, MutBoard, Termination};
use myopic_core::bitboard::BitBoard;
use myopic_core::castlezone::CastleZone;
use myopic_core::pieces::Piece;
use myopic_core::reflectable::Reflectable;
use myopic_core::{Side, Square};

#[derive(Clone, Eq, PartialEq)]
pub struct EvalBoardImpl<B: MutBoard> {
    mid_eval: i32,
    end_eval: i32,
    phase: i32,
    board: B,
}

impl<B: MutBoard> Reflectable for EvalBoardImpl<B> {
    fn reflect(&self) -> Self {
        EvalBoardImpl {
            mid_eval: -self.mid_eval,
            end_eval: -self.end_eval,
            phase: self.phase,
            board: self.board.reflect(),
        }
    }
}

const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];
const TOTAL_PHASE: i32 = 16 * PHASE_VALUES[0]
    + 4 * (PHASE_VALUES[1] + PHASE_VALUES[2] + PHASE_VALUES[3])
    + 2 * PHASE_VALUES[4];

fn compute_phase<B: MutBoard>(board: &B) -> i32 {
    let pieces: Vec<_> = Piece::iter_w().take(5).chain(Piece::iter_b().take(5)).collect();
    let phase_sub: i32 = pieces
        .into_iter()
        .map(|p| board.locs(p).size() as i32 * PHASE_VALUES[(p as usize) % 6])
        .sum();
    TOTAL_PHASE - phase_sub
}

fn compute_midgame<B: MutBoard>(board: &B) -> i32 {
    Piece::iter()
        .flat_map(|p| board.locs(p).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables::midgame(p, loc) + values::midgame(p))
        .sum()
}

fn compute_endgame<B: MutBoard>(board: &B) -> i32 {
    Piece::iter()
        .flat_map(|p| board.locs(p).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables::endgame(p, loc) + values::endgame(p))
        .sum()
}

impl<B: MutBoard> EvalBoardImpl<B> {
    pub fn new(board: B) -> EvalBoardImpl<B> {
        EvalBoardImpl {
            mid_eval: compute_midgame(&board),
            end_eval: compute_endgame(&board),
            phase: compute_phase(&board),
            board,
        }
    }

    fn remove(&mut self, piece: Piece, location: Square) {
        self.mid_eval -= tables::midgame(piece, location) + values::midgame(piece);
        self.end_eval -= tables::endgame(piece, location) + values::endgame(piece);
        self.phase += PHASE_VALUES[(piece as usize) % 6];
    }

    fn add(&mut self, piece: Piece, location: Square) {
        self.mid_eval += tables::midgame(piece, location) + values::midgame(piece);
        self.end_eval += tables::endgame(piece, location) + values::endgame(piece);
        self.phase -= PHASE_VALUES[(piece as usize) % 6];
    }
}

impl<B: MutBoard> MutBoard for EvalBoardImpl<B> {
    fn evolve(&mut self, action: &Move) -> Discards {
        match action {
            &Move::Standard(moving, src, target) => {
                self.remove(moving, src);
                self.add(moving, target);
                self.piece(target).map(|taken| self.remove(taken, target));
            }
            &Move::Promotion(source, target, promoting) => {
                let pawn = Piece::pawn(self.active());
                self.remove(pawn, source);
                self.add(promoting, target);
                self.piece(target).map(|taken| self.remove(taken, target));
            }
            &Move::Enpassant(source, _) => {
                let active_pawn = Piece::pawn(self.active());
                let passive_pawn = active_pawn.reflect();
                let enpassant = self.enpassant().unwrap();
                let removal_square = match self.active() {
                    Side::White => enpassant >> 8,
                    Side::Black => enpassant << 8,
                };
                self.remove(active_pawn, source);
                self.add(active_pawn, enpassant);
                self.remove(passive_pawn, removal_square);
            }
            &Move::Castle(zone) => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
                self.remove(rook, r_src);
                self.add(rook, r_target);
                self.remove(king, k_src);
                self.add(king, k_target);
            }
        };
        self.board.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: Discards) {
        match action {
            &Move::Standard(moving, src, target) => {
                self.remove(moving, target);
                self.add(moving, src);
                discards.piece.map(|taken| self.add(taken, target));
            }
            &Move::Promotion(source, target, promoting) => {
                let pawn = Piece::pawn(self.active().reflect());
                self.add(pawn, source);
                self.remove(promoting, target);
                discards.piece.map(|taken| self.add(taken, target));
            }
            &Move::Enpassant(source, _) => {
                let active_pawn = Piece::pawn(self.active());
                let passive_pawn = active_pawn.reflect();
                let enpassant = discards.enpassant.unwrap();
                let removal_square = match self.active() {
                    Side::White => enpassant << 8,
                    Side::Black => enpassant >> 8,
                };
                self.remove(passive_pawn, enpassant);
                self.add(passive_pawn, source);
                self.add(active_pawn, removal_square);
            }
            &Move::Castle(zone) => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
                self.add(rook, r_src);
                self.remove(rook, r_target);
                self.add(king, k_src);
                self.remove(king, k_target);
            }
        };
        self.board.devolve(action, discards)
    }

    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        self.board.compute_moves(computation_type)
    }

    fn termination_status(&mut self) -> Option<Termination> {
        self.board.termination_status()
    }

    fn hash(&self) -> u64 {
        self.board.hash()
    }

    fn active(&self) -> Side {
        self.board.active()
    }

    fn enpassant(&self) -> Option<Square> {
        self.board.enpassant()
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.board.castle_status(side)
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.board.locs(piece)
    }

    fn king(&self, side: Side) -> Square {
        self.board.king(side)
    }

    fn side(&self, side: Side) -> BitBoard {
        self.board.side(side)
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        self.board.sides()
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.board.piece(location)
    }

    fn half_move_clock(&self) -> usize {
        self.board.half_move_clock()
    }

    fn history_count(&self) -> usize {
        self.board.history_count()
    }

    fn in_check(&mut self) -> bool {
        self.board.in_check()
    }
}

impl<B: MutBoard> EvalBoard for EvalBoardImpl<B> {
    fn static_eval(&mut self) -> i32 {
        match self.termination_status() {
            Some(Termination::Draw) => eval::DRAW_VALUE,
            Some(Termination::Loss) => eval::LOSS_VALUE,
            None => {
                let phase: i32 = ((self.phase * 256 + TOTAL_PHASE / 2) / TOTAL_PHASE) as i32;
                let (mid, end) = (self.mid_eval, self.end_eval);
                let eval = ((mid * (256 - phase)) + end * phase) / 256;
                match self.active() {
                    Side::White => eval,
                    Side::Black => -eval,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::eval_impl::EvalBoardImpl;
    use myopic_board::Move::*;
    use myopic_board::{Move, MutBoard};
    use myopic_core::castlezone::CastleZone;
    use myopic_core::pieces::Piece::*;
    use myopic_core::reflectable::Reflectable;
    use myopic_core::Square::*;

    #[derive(Clone, Eq, PartialEq)]
    struct TestCase<B: MutBoard> {
        start_position: B,
        moves: Vec<Move>,
    }

    impl<B: MutBoard> Reflectable for TestCase<B> {
        fn reflect(&self) -> Self {
            TestCase { start_position: self.start_position.reflect(), moves: self.moves.reflect() }
        }
    }

    fn execute_test<B: MutBoard>(test_case: TestCase<B>) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl<B: MutBoard>(test_case: TestCase<B>) {
        let mut start = EvalBoardImpl::new(test_case.start_position);
        for evolution in test_case.moves {
            let discards = start.evolve(&evolution);
            assert_eq!(super::compute_midgame(&start), start.mid_eval);
            assert_eq!(super::compute_endgame(&start), start.end_eval);
            start.devolve(&evolution, discards);
            assert_eq!(super::compute_midgame(&start), start.mid_eval);
            assert_eq!(super::compute_endgame(&start), start.end_eval);
            start.evolve(&evolution);
        }
    }

    fn test(start_fen: &'static str, moves: Vec<Move>) {
        execute_test(TestCase {
            start_position: myopic_board::fen_position(start_fen).unwrap(),
            moves,
        })
    }

    #[test]
    fn case_1() {
        test(
            "rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4",
            vec![
                Standard(WP, C2, C3),
                Standard(BN, G8, F6),
                Castle(CastleZone::WK),
                Standard(BP, B7, B6),
                Standard(WP, D2, D3),
                Standard(BB, C8, B7),
                Standard(WB, C1, G5),
                Standard(BN, B8, C6),
                Standard(WN, B1, D2),
                Standard(BQ, D8, C7),
                Standard(WQ, D1, C2),
                Castle(CastleZone::BQ),
                Standard(WP, E4, E5),
                Standard(BP, D7, D5),
                Enpassant(E5, D6),
                Standard(BK, C8, B8),
                Standard(WP, D6, E7),
                Standard(BR, H8, H7),
                Promotion(E7, D8, WQ),
            ],
        );
    }
}
