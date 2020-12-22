use crate::eval::material::Material;
use crate::eval::{EvalBoard, EvalComponent};
use crate::{eval, PieceValues, PositionTables};
use myopic_board::{
    BitBoard, CastleZone, CastleZoneSet, Discards, FenComponent, Move, MoveComputeType, MutBoard,
    Piece, Reflectable, Side, Square, Termination,
};

pub struct EvalBoardImpl2<B: MutBoard> {
    board: B,
    material: Material,
    cmps: Vec<Box<dyn EvalComponent>>,
}

impl<B: MutBoard> Clone for EvalBoardImpl2<B> {
    fn clone(&self) -> Self {
        EvalBoardImpl2 {
            board: self.board.clone(),
            material: self.material.clone(),
            cmps: self.cmps.iter().map(|cmp| cmp.replicate()).collect(),
        }
    }
}

#[derive(Clone)]
pub struct EvalBoardImpl<B: MutBoard> {
    board: B,
    material: Material,
}

impl<B: MutBoard> Reflectable for EvalBoardImpl<B> {
    fn reflect(&self) -> Self {
        EvalBoardImpl {
            board: self.board.reflect(),
            material: self.material.reflect(),
        }
    }
}

impl<B: MutBoard> EvalBoardImpl<B> {
    pub fn new(board: B, values: PieceValues, tables: PositionTables) -> EvalBoardImpl<B> {
        EvalBoardImpl {
            material: Material::new(&board, values, tables),
            board,
        }
    }
}

impl<B: MutBoard> MutBoard for EvalBoardImpl<B> {
    fn evolve(&mut self, action: &Move) -> Discards {
        self.material.evolve(&self.board, action);
        self.board.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: Discards) {
        self.material.devolve(&self.board, action, &discards);
        self.board.devolve(action, discards)
    }

    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        self.board.compute_moves(computation_type)
    }

    fn termination_status(&mut self) -> Option<Termination> {
        self.board.termination_status()
    }

    fn in_check(&mut self) -> bool {
        self.board.in_check()
    }

    fn side(&self, side: Side) -> BitBoard {
        self.board.side(side)
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        self.board.sides()
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

    fn piece(&self, location: Square) -> Option<Piece> {
        self.board.piece(location)
    }

    fn half_move_clock(&self) -> usize {
        self.board.half_move_clock()
    }

    fn history_count(&self) -> usize {
        self.board.history_count()
    }

    fn remaining_rights(&self) -> CastleZoneSet {
        self.board.remaining_rights()
    }

    fn to_partial_fen(&self, cmps: &[FenComponent]) -> String {
        self.board.to_partial_fen(cmps)
    }
}

impl<B: MutBoard> EvalBoard for EvalBoardImpl<B> {
    fn static_eval(&mut self) -> i32 {
        match self.termination_status() {
            Some(Termination::Draw) => eval::DRAW_VALUE,
            Some(Termination::Loss) => eval::LOSS_VALUE,
            None => {
                let eval = self.material.static_eval();
                match self.active() {
                    Side::White => eval,
                    Side::Black => -eval,
                }
            }
        }
    }

    // TODO For now we just use midgame values, should take into account phase
    fn piece_values(&self) -> &[i32; 6] {
        &self.material.values().midgame
    }

    // TODO For now we just use midgame values, should take into account phase
    fn positional_eval(&self, piece: Piece, location: Square) -> i32 {
        self.material.tables().midgame(piece, location)
    }
}

#[cfg(test)]
mod test {
    use crate::eval::eval_impl::EvalBoardImpl;
    use crate::{PieceValues, PositionTables};
    use myopic_board::{CastleZone, Move, Move::*, MutBoard, Piece::*, Reflectable, Square::*};
    use crate::eval::material;

    #[derive(Clone, Eq, PartialEq)]
    struct TestCase<B: MutBoard> {
        start_position: B,
        moves: Vec<Move>,
    }

    impl<B: MutBoard> Reflectable for TestCase<B> {
        fn reflect(&self) -> Self {
            TestCase {
                start_position: self.start_position.reflect(),
                moves: self.moves.reflect(),
            }
        }
    }

    fn execute_test<B: MutBoard>(test_case: TestCase<B>) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl<B: MutBoard>(test_case: TestCase<B>) {
        let (tables, values) = (PositionTables::default(), PieceValues::default());
        let mut start =
            EvalBoardImpl::new(test_case.start_position, values.clone(), tables.clone());

        for evolution in test_case.moves {
            let discards = start.evolve(&evolution);
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
            start.devolve(&evolution, discards);
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
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
