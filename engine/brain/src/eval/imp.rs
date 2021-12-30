use std::str::FromStr;

use myopic_board::{
    BitBoard, CastleZone, ChessBoard, FenComponent, Move, MoveComputeType, Piece,
    Side, Square, Termination,
};
use myopic_board::anyhow::{Error, Result};

use crate::{Board, eval, PieceValues, PositionTables};
use crate::enumset::EnumSet;
use crate::eval::{EvalChessBoard, EvalComponent};
use crate::eval::material::Material;
use crate::eval::opening::OpeningComponent;

pub struct EvalBoard<B: ChessBoard> {
    board: B,
    material: Material,
    cmps: Vec<Box<dyn EvalComponent>>,
}

pub struct BoardBuilder<B: ChessBoard> {
    board: B,
    piece_values: PieceValues,
    position_tables: PositionTables,
    cmps: Vec<Box<dyn EvalComponent>>,
}

impl Default for EvalBoard<Board> {
    fn default() -> Self {
        crate::START_FEN.parse().unwrap()
    }
}

impl<B: ChessBoard> From<B> for EvalBoard<B> {
    fn from(board: B) -> Self {
        BoardBuilder::from(board).build()
    }
}

impl<B: ChessBoard> From<B> for BoardBuilder<B> {
    fn from(board: B) -> Self {
        BoardBuilder {
            board,
            position_tables: PositionTables::default(),
            piece_values: PieceValues::default(),
            cmps: Vec::default(),
        }
    }
}

impl FromStr for BoardBuilder<Board> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BoardBuilder {
            board: s.parse::<Board>()?,
            piece_values: PieceValues::default(),
            position_tables: PositionTables::default(),
            cmps: if s == crate::START_FEN {
                vec![Box::new(OpeningComponent::default())]
            } else {
                vec![]
            },
        })
    }
}

impl FromStr for EvalBoard<Board> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<BoardBuilder<Board>>().map(|b| b.build())
    }
}

impl<B: ChessBoard> BoardBuilder<B> {
    pub fn set_piece_values(mut self, piece_values: PieceValues) -> BoardBuilder<B> {
        self.piece_values = piece_values;
        self
    }

    pub fn set_position_tables(mut self, position_tables: PositionTables) -> BoardBuilder<B> {
        self.position_tables = position_tables;
        self
    }

    pub fn add_eval_component(mut self, cmp: Box<dyn EvalComponent>) -> BoardBuilder<B> {
        self.cmps.push(cmp);
        self
    }

    pub fn build(self) -> EvalBoard<B> {
        EvalBoard {
            material: Material::new(&self.board, self.piece_values, self.position_tables),
            board: self.board,
            cmps: self.cmps,
        }
    }
}

impl<B: ChessBoard> ChessBoard for EvalBoard<B> {
    fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action);
        for cmp in self.cmps.iter_mut() {
            cmp.make(&action);
        }
        self.board.make(action)
    }

    fn unmake(&mut self) -> Result<Move> {
        let action = self.board.unmake()?;
        self.material.unmake(&action);
        for cmp in self.cmps.iter_mut() {
            cmp.unmake(&action);
        }
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

    fn remaining_rights(&self) -> EnumSet<CastleZone> {
        self.board.remaining_rights()
    }

    fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let parsed_moves = self.board.play_pgn(moves)?;
        for mv in parsed_moves.iter() {
            self.material.make(mv);
            for cmp in self.cmps.iter_mut() {
                cmp.make(mv);
            }
        }
        Ok(parsed_moves)
    }

    fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let parsed_moves = self.board.play_uci(moves)?;
        for mv in parsed_moves.iter() {
            self.material.make(mv);
            for cmp in self.cmps.iter_mut() {
                cmp.make(mv);
            }
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

impl<B: ChessBoard> EvalChessBoard for EvalBoard<B> {
    fn static_eval(&mut self) -> i32 {
        match self.termination_status() {
            Some(Termination::Draw) => eval::DRAW_VALUE,
            Some(Termination::Loss) => eval::LOSS_VALUE,
            None => {
                let eval = self.material.static_eval()
                    + self.cmps.iter().map(|cmp| cmp.static_eval()).sum::<i32>();
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
    use myopic_board::{ChessBoard, Reflectable, UciMove};

    use crate::{Board, BoardBuilder, PieceValues, PositionTables};
    use crate::eval::material;

    #[derive(Clone, Eq, PartialEq)]
    struct TestCase<B: ChessBoard> {
        start_position: B,
        moves: Vec<UciMove>,
    }

    impl<B: ChessBoard + Reflectable> Reflectable for TestCase<B> {
        fn reflect(&self) -> Self {
            TestCase {
                start_position: self.start_position.reflect(),
                moves: self.moves.reflect(),
            }
        }
    }

    fn execute_test<B: ChessBoard + Reflectable + Clone>(test_case: TestCase<B>) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl<B: ChessBoard>(test_case: TestCase<B>) {
        let (tables, values) = (PositionTables::default(), PieceValues::default());
        let mut start = BoardBuilder::from(test_case.start_position)
            .set_piece_values(values.clone())
            .set_position_tables(tables.clone())
            .build();

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
