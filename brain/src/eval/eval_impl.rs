use crate::eval::material::Material;
use crate::eval::{EvalBoard, EvalComponent};
use crate::{eval, PieceValues, PositionTables};
use anyhow::Result;
use myopic_board::{
    BitBoard, CastleZone, CastleZoneSet, ChessBoard, FenComponent, Move, MoveComputeType, Piece,
    Reflectable, Side, Square, Termination,
};

pub struct EvalBoardImpl2<B: ChessBoard> {
    board: B,
    material: Material,
    cmps: Vec<Box<dyn EvalComponent>>,
}

impl<B: ChessBoard> Clone for EvalBoardImpl2<B> {
    fn clone(&self) -> Self {
        EvalBoardImpl2 {
            board: self.board.clone(),
            material: self.material.clone(),
            cmps: self.cmps.iter().map(|cmp| cmp.replicate()).collect(),
        }
    }
}

#[derive(Clone)]
pub struct EvalBoardImpl<B: ChessBoard> {
    board: B,
    material: Material,
}

impl<B: ChessBoard> Reflectable for EvalBoardImpl<B> {
    fn reflect(&self) -> Self {
        EvalBoardImpl {
            board: self.board.reflect(),
            material: self.material.reflect(),
        }
    }
}

impl<B: ChessBoard> EvalBoardImpl<B> {
    pub fn new(board: B, values: PieceValues, tables: PositionTables) -> EvalBoardImpl<B> {
        EvalBoardImpl {
            material: Material::new(&board, values, tables),
            board,
        }
    }
}

impl<B: ChessBoard> ChessBoard for EvalBoardImpl<B> {
    fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action);
        self.board.make(action)
    }

    fn unmake(&mut self) -> Result<Move> {
        let action = self.board.unmake()?;
        self.material.unmake(&action);
        Ok(action)
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

    fn locs(&self, pieces: &[Piece]) -> BitBoard {
        self.board.locs(pieces)
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

    fn position_count(&self) -> usize {
        self.board.position_count()
    }

    fn remaining_rights(&self) -> CastleZoneSet {
        self.board.remaining_rights()
    }

    fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let parsed_moves = self.board.play_pgn(moves)?;
        for mv in parsed_moves.iter() {
            self.material.make(mv);
        }
        Ok(parsed_moves)
    }

    fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let parsed_moves = self.board.play_uci(moves)?;
        for mv in parsed_moves.iter() {
            self.material.make(mv);
        }
        Ok(parsed_moves)
    }

    fn parse_uci(&mut self, uci_move: &str) -> Result<Move> {
        self.board.parse_uci(uci_move)
    }

    fn to_partial_fen(&self, cmps: &[FenComponent]) -> String {
        self.board.to_partial_fen(cmps)
    }
}

impl<B: ChessBoard> EvalBoard for EvalBoardImpl<B> {
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
    use crate::eval::material;
    use crate::{Board, PieceValues, PositionTables};
    use myopic_board::{
        CastleZone, ChessBoard, Move, Move::*, Piece::*, Reflectable, Square::*, UciMove,
    };

    #[derive(Clone, Eq, PartialEq)]
    struct TestCase<B: ChessBoard> {
        start_position: B,
        moves: Vec<UciMove>,
    }

    impl<B: ChessBoard> Reflectable for TestCase<B> {
        fn reflect(&self) -> Self {
            TestCase {
                start_position: self.start_position.reflect(),
                moves: self.moves.reflect(),
            }
        }
    }

    fn execute_test<B: ChessBoard>(test_case: TestCase<B>) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl<B: ChessBoard>(test_case: TestCase<B>) {
        let (tables, values) = (PositionTables::default(), PieceValues::default());
        let mut start =
            EvalBoardImpl::new(test_case.start_position, values.clone(), tables.clone());

        for uci_move in test_case.moves {
            let move_to_make = start.parse_uci(uci_move.as_str()).unwrap();
            start.make(move_to_make).unwrap();
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
            let move_made = start.unmake().unwrap();
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
            start.make(move_made).unwrap();
        }
    }

    fn test(start_fen: &'static str, moves: Vec<UciMove>) {
        execute_test(TestCase {
            start_position: start_fen.parse::<Board>().unwrap(),
            moves,
        })
    }

    #[test]
    fn case_1() {
        test(
            "rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4",
            vec![
                UciMove::new("c2c3").unwrap(),
                UciMove::new("g8f6").unwrap(),
                UciMove::new("e1g1").unwrap(),
                UciMove::new("b7b6").unwrap(),
                UciMove::new("d2d3").unwrap(),
                UciMove::new("c8b7").unwrap(),
                UciMove::new("c1g5").unwrap(),
                UciMove::new("b8c6").unwrap(),
                UciMove::new("b1d2").unwrap(),
                UciMove::new("d8c7").unwrap(),
                UciMove::new("d1c2").unwrap(),
                UciMove::new("e8c8").unwrap(),
                UciMove::new("e4e5").unwrap(),
                UciMove::new("d7d5").unwrap(),
                UciMove::new("e5d6").unwrap(),
                UciMove::new("c8b8").unwrap(),
                UciMove::new("d6e7").unwrap(),
                UciMove::new("h8g8").unwrap(),
                UciMove::new("e7d8q").unwrap(),
            ],
        );
    }
}
