use std::str::FromStr;

use crate::anyhow::{Error, Result};
use crate::eval::antipattern::KnightRimFacet;
use crate::eval::castling::CastlingFacet;
use crate::eval::development::DevelopmentFacet;
use crate::eval::material::MaterialFacet;
use crate::eval::tables::PositionTables;
use crate::eval::values::PieceValues;
use crate::{Piece, Square};
use myopic_board::{Board, Move, TerminalState};
use crate::eval::phase::Phase;

mod antipattern;
mod castling;
mod development;
mod material;
pub mod tables;
pub mod values;
mod phase;

/// The evaluation upper/lower bound definition
pub const INFTY: i32 = 500_000i32;

/// The evaluation assigned to a won position.
pub const WIN_VALUE: i32 = INFTY - 1;

/// The evaluation assigned to a lost position.
pub const LOSS_VALUE: i32 = -WIN_VALUE;

/// The evaluation assigned to a drawn position.
pub const DRAW_VALUE: i32 = 0;

/// Represents some (possibly stateful) feature of a position which can be
/// evaluated.
pub trait EvalFacet {
    /// Return the static evaluation of the given position. Implementors are
    /// guaranteed that exactly the same move sequence will have been passed to
    /// this component and the given board position. I.e the internal states
    /// are aligned. It must follow the rule 'A LARGER +VE SCORE BETTER FOR
    /// WHITE, LARGER -VE SCORE BETTER FOR BLACK'.
    fn static_eval(&self, board: &Board) -> i32;

    /// Update internal state by making the given move FROM the given position
    fn make(&mut self, mv: &Move, board: &Board);

    /// Update internal state by unmaking the given move which is guaranteed to
    /// have previously been passed to the "make" method.
    fn unmake(&mut self, mv: &Move);
}

/// Wrapper around a chess board which adds position evaluation capabilities.
/// The evaluation function is decomposed into orthogonal "facets".
pub struct Evaluator {
    board: Board,
    phase: Phase,
    material: MaterialFacet,
    facets: Vec<Box<dyn EvalFacet>>,
}

impl Evaluator {
    /// Get an immutable reference to the underlying board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Make the given move on the underlying board and update all the internal
    /// facets
    pub fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action, &self.board);
        self.phase.make(&action);
        for cmp in self.facets.iter_mut() {
            cmp.make(&action, &self.board);
        }
        self.board.make(action)
    }

    /// Unmake the given move on the underlying board and update all the internal
    /// facets
    pub fn unmake(&mut self) -> Result<Move> {
        let action = self.board.unmake()?;
        self.material.unmake(&action);
        self.phase.unmake(&action);
        for cmp in self.facets.iter_mut() {
            cmp.unmake(&action);
        }
        Ok(action)
    }

    /// Parse and make the pgn encoded moves and returns the parsed moves
    pub fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_pgn(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    /// Parse and make the uci encoded moves and returns the parsed moves
    pub fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_uci(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    /// The relative evaluation function assigns a score to this exact position
    /// at the point of time it is called. It does not take into account
    /// potential captures/recaptures etc. It must follow the rule that 'A
    /// LARGER +VE SCORE BETTER FOR ACTIVE, LARGER -VE SCORE BETTER FOR PASSIVE'.
    /// That is if it is white to move next then a high positive score indicates
    /// a favorable position for white and if it is black to move a high positive
    /// score indicates a favorable position for black. If the state it terminal
    /// it must return the LOSS_VALUE or DRAW_VALUE depending on the type of
    /// termination.
    pub fn relative_eval(&self) -> i32 {
        match self.board.terminal_state() {
            Some(TerminalState::Draw) => DRAW_VALUE,
            Some(TerminalState::Loss) => LOSS_VALUE,
            None => {
                self.board.active().parity()
                    * (self.material.static_eval(&self.board)
                        + self.facets.iter().map(|cmp| cmp.static_eval(&self.board)).sum::<i32>())
            }
        }
    }

    // TODO For now we just use midgame values, should take into account phase
    pub fn piece_values(&self) -> &[i32; 6] {
        &self.material.values().midgame
    }

    // TODO For now we just use midgame values, should take into account phase
    pub fn positional_eval(&self, piece: Piece, location: Square) -> i32 {
        self.material.tables().midgame(piece, location)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        crate::START_FEN.parse().unwrap()
    }
}

impl From<Board> for Evaluator {
    fn from(board: Board) -> Self {
        Evaluator {
            material: MaterialFacet::new(&board, PieceValues::default(), PositionTables::default()),
            phase: Phase::from(&board),
            facets: if board.to_fen().as_str() == crate::START_FEN {
                vec![
                    Box::new(CastlingFacet::default()),
                    Box::new(DevelopmentFacet::default()),
                    Box::new(KnightRimFacet::default()),
                ]
            } else {
                vec![]
            },
            board,
        }
    }
}

impl FromStr for Evaluator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Board>().map(|b| Evaluator::from(b))
    }
}

#[cfg(test)]
mod test {
    use myopic_board::{Reflectable, UciMove};

    use crate::eval::material::MaterialFacet;
    use crate::eval::{material, Evaluator};
    use crate::{Board, PieceValues, PositionTables};
    use crate::eval::phase::Phase;

    #[derive(Clone)]
    struct TestCase {
        start_position: Board,
        moves: Vec<UciMove>,
    }

    impl Reflectable for TestCase {
        fn reflect(&self) -> Self {
            TestCase { start_position: self.start_position.reflect(), moves: self.moves.reflect() }
        }
    }

    fn execute_test(test_case: TestCase) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl(test_case: TestCase) {
        let (tables, values) = (PositionTables::default(), PieceValues::default());
        let board = test_case.start_position;
        let mut start = Evaluator {
            material: MaterialFacet::new(&board, values.clone(), tables.clone()),
            phase: Phase::from(&board),
            facets: vec![],
            board,
        };
        for uci_move in test_case.moves {
            let move_to_make = start.board.parse_uci(uci_move.as_str()).unwrap();
            start.make(move_to_make).unwrap();
            assert_eq!(
                material::compute_midgame(start.board(), &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(start.board(), &values, &tables),
                start.material.end_eval()
            );
            let move_made = start.unmake().unwrap();
            assert_eq!(
                material::compute_midgame(start.board(), &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(start.board(), &values, &tables),
                start.material.end_eval()
            );
            start.make(move_made).unwrap();
        }
    }

    fn test(start_fen: &'static str, moves: Vec<UciMove>) {
        execute_test(TestCase { start_position: start_fen.parse::<Board>().unwrap(), moves })
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
