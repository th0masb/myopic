use myopic_board::enum_map::{enum_map, Enum, EnumMap};
use myopic_board::Square;
use std::borrow::BorrowMut;

use crate::eval::EvalComponent;
use crate::{CastleZone, Move, Piece, Reflectable, Side};

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
enum OpeningPiece {
    EPawn,
    DPawn,
    BKnight,
    GKnight,
    CBishop,
    FBishop,
}

impl OpeningPiece {
    fn start(self, side: Side) -> Square {
        let white_position = match self {
            OpeningPiece::EPawn => Square::E2,
            OpeningPiece::DPawn => Square::D2,
            OpeningPiece::BKnight => Square::B1,
            OpeningPiece::GKnight => Square::G1,
            OpeningPiece::CBishop => Square::C1,
            OpeningPiece::FBishop => Square::F1,
        };
        if side == Side::White {
            white_position
        } else {
            white_position.reflect()
        }
    }

    fn applicable_for<'a>(p: Piece) -> &'a [OpeningPiece] {
        match p {
            Piece::WP | Piece::BP => &[OpeningPiece::DPawn, OpeningPiece::EPawn],
            Piece::WN | Piece::BN => &[OpeningPiece::BKnight, OpeningPiece::GKnight],
            Piece::WB | Piece::BB => &[OpeningPiece::CBishop, OpeningPiece::FBishop],
            _ => &[],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
enum CastleSide {
    King,
    Queen,
}

#[derive(Debug)]
pub struct OpeningRewards {
    pieces: EnumMap<OpeningPiece, i32>,
    castle: EnumMap<CastleSide, i32>,
}

impl OpeningRewards {
    fn parity(side: Side) -> i32 {
        match side {
            Side::White => 1,
            Side::Black => -1,
        }
    }

    fn zone(&self, zone: CastleZone) -> i32 {
        OpeningRewards::parity(zone.side())
            * match zone {
                CastleZone::WK | CastleZone::BK => self.castle[CastleSide::King],
                CastleZone::WQ | CastleZone::BQ => self.castle[CastleSide::Queen],
            }
    }

    fn piece(&self, side: Side, piece: OpeningPiece) -> i32 {
        OpeningRewards::parity(side) * self.pieces[piece]
    }
}

impl Default for OpeningRewards {
    fn default() -> Self {
        OpeningRewards {
            pieces: enum_map! {
                OpeningPiece::EPawn => 200,
                OpeningPiece::DPawn => 150,
                OpeningPiece::BKnight => 100,
                OpeningPiece::GKnight => 150,
                OpeningPiece::CBishop => 100,
                OpeningPiece::FBishop => 150,
            },
            castle: enum_map! {
                CastleSide::King => 200,
                CastleSide::Queen => 100,
            },
        }
    }
}

#[derive(Debug)]
pub struct OpeningComponent {
    rewards: OpeningRewards,
    score: i32,
    development: EnumMap<Side, DevelopmentTracker>,
    move_dist: usize,
}

impl Default for OpeningComponent {
    fn default() -> Self {
        OpeningComponent::new(OpeningRewards::default())
    }
}

impl OpeningComponent {
    pub fn new(rewards: OpeningRewards) -> OpeningComponent {
        OpeningComponent {
            rewards,
            score: 0,
            move_dist: 0,
            development: enum_map! {
                Side::White => DevelopmentTracker::new(Side::White),
                Side::Black => DevelopmentTracker::new(Side::Black),
            },
        }
    }

    fn borrow_tracker_mut(&mut self, side: Side, piece: OpeningPiece) -> &mut PieceTracker {
        self.development[side].borrow_mut().pieces[piece].borrow_mut()
    }
}

#[derive(Debug)]
struct DevelopmentTracker {
    pieces: EnumMap<OpeningPiece, PieceTracker>,
}

impl DevelopmentTracker {
    fn new(side: Side) -> DevelopmentTracker {
        let mut pieces: EnumMap<OpeningPiece, PieceTracker> = EnumMap::default();
        for (piece, value) in pieces.iter_mut() {
            value.loc = piece.start(side);
        }
        DevelopmentTracker { pieces }
    }
}

#[derive(Debug)]
struct PieceTracker {
    /// The most recent location of the piece on the board
    loc: Square,
    /// How many moves this piece has made from it's start
    /// position
    count: usize,
    /// The move dist at which this piece was removed
    /// from the board. None if still on board.
    capture_dist: Option<usize>,
}

impl Default for PieceTracker {
    fn default() -> Self {
        PieceTracker {
            loc: Square::A1,
            count: 0,
            capture_dist: None,
        }
    }
}

impl PieceTracker {
    fn move_forward(&mut self, new_loc: Square) -> usize {
        self.loc = new_loc;
        self.count += 1;
        self.count
    }

    fn move_backward(&mut self, old_loc: Square) -> usize {
        self.loc = old_loc;
        self.count -= 1;
        self.count
    }

    fn remove_piece(&mut self, capture_dist: usize) {
        self.capture_dist = Some(capture_dist);
    }

    fn add_piece(&mut self) {
        self.capture_dist = None;
    }

    fn matches_on(&self, loc: Square) -> bool {
        self.capture_dist.is_none() && loc == self.loc
    }

    fn matches_off(&self, move_dist: usize) -> bool {
        match self.capture_dist {
            None => false,
            Some(cd) => cd == move_dist,
        }
    }
}

impl EvalComponent for OpeningComponent {
    fn static_eval(&self) -> i32 {
        self.score
    }

    fn make(&mut self, mv: &Move) {
        self.move_dist += 1;
        match mv {
            &Move::Castle { zone, .. } => self.score += self.rewards.zone(zone),
            &Move::Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => {
                // Update location of moving piece
                for &opener in OpeningPiece::applicable_for(moving) {
                    let side = moving.side();
                    let piece_tracker = self.borrow_tracker_mut(side, opener);
                    if piece_tracker.matches_on(from) && piece_tracker.move_forward(dest) == 1 {
                        self.score += self.rewards.piece(side, opener);
                    }
                }
                // Record removal of any relevant captured piece
                if capture.is_some() {
                    let captured = capture.unwrap();
                    for &opener in OpeningPiece::applicable_for(captured) {
                        let side = captured.side();
                        let move_dist = self.move_dist;
                        let piece_tracker = self.borrow_tracker_mut(side, opener);
                        if piece_tracker.matches_on(dest) {
                            piece_tracker.remove_piece(move_dist);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn unmake(&mut self, mv: &Move) {
        match mv {
            &Move::Castle { zone, .. } => self.score -= self.rewards.zone(zone),
            &Move::Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => {
                // Update location of moving piece
                for &opener in OpeningPiece::applicable_for(moving) {
                    let side = moving.side();
                    let piece_tracker = self.borrow_tracker_mut(side, opener);
                    if piece_tracker.matches_on(dest) && piece_tracker.move_backward(from) == 0 {
                        self.score -= self.rewards.piece(side, opener);
                    }
                }
                // Record removal of any relevant captured piece
                if capture.is_some() {
                    let captured = capture.unwrap();
                    for &opener in OpeningPiece::applicable_for(captured) {
                        let side = captured.side();
                        let move_dist = self.move_dist;
                        let piece_tracker = self.borrow_tracker_mut(side, opener);
                        if piece_tracker.matches_off(move_dist) {
                            piece_tracker.add_piece();
                        }
                    }
                }
            }
            _ => {}
        };
        self.move_dist -= 1;
    }
}

#[cfg(test)]
mod test {
    use myopic_board::anyhow::Result;
    use myopic_board::ChessBoard;

    use super::*;
    use crate::eval::opening::{OpeningComponent, OpeningRewards};
    use crate::eval::EvalComponent;
    use crate::{Board, EvalBoard, Reflectable, UciMove};

    #[test]
    fn issue_97() -> Result<()> {
        let mut state = EvalBoard::default();
        state.play_uci(
            "e2e4 g7g6 d2d4 f8g7 c2c4 d7d6 b1c3 g8f6 g1f3 e8g8 f1d3 e7e5 f3d2 e5d4 d2b1 d4c3 b2c3",
        )?;
        state.unmake()?;
        state.play_uci("b1c3")?;
        state.unmake()?;
        state.unmake()?;
        state.unmake()?;
        Ok(())
    }

    fn dummy_rewards() -> OpeningRewards {
        OpeningRewards {
            castle: enum_map! {
                CastleSide::King => 1000000,
                CastleSide::Queen => 10000000,
            },
            pieces: enum_map! {
                OpeningPiece::DPawn => 1,
                OpeningPiece::EPawn => 10,
                OpeningPiece::GKnight => 100,
                OpeningPiece::BKnight => 1000,
                OpeningPiece::FBishop => 10000,
                OpeningPiece::CBishop => 100000,
            },
        }
    }

    #[test]
    fn case_3() -> Result<()> {
        execute_case(TestCase {
            board: crate::start(),
            moves_evals: vec![
                (UciMove::new("e2e4")?, 10),
                (UciMove::new("g7g6")?, 10),
                (UciMove::new("d2d4")?, 11),
                (UciMove::new("f8g7")?, -9989),
                (UciMove::new("c2c4")?, -9989),
                (UciMove::new("d7d6")?, -9990),
                (UciMove::new("b1c3")?, -8990),
                (UciMove::new("g8f6")?, -9090),
                (UciMove::new("g1f3")?, -8990),
                (UciMove::new("e8g8")?, -1008990),
                (UciMove::new("f1d3")?, -998990),
            ],
        })
    }

    #[test]
    fn case_1() -> Result<()> {
        execute_case(TestCase {
            board: crate::start(),
            moves_evals: vec![
                (UciMove::new("d2d4")?, 1),
                (UciMove::new("d7d5")?, 0),
                (UciMove::new("e2e4")?, 10),
                (UciMove::new("e7e5")?, 0),
                (UciMove::new("a2a3")?, 0),
                (UciMove::new("g8f6")?, -100),
                (UciMove::new("b1c3")?, 900), // w
                (UciMove::new("b8a6")?, -100),
                (UciMove::new("g1f3")?, 0),
                (UciMove::new("c8d7")?, -100000),
                (UciMove::new("c1d2")?, 0),
                (UciMove::new("f8b4")?, -10000),
                (UciMove::new("f1b5")?, 0),
                (UciMove::new("d8e7")?, 0),
                (UciMove::new("d1e2")?, 0),
                // Castle kingside
                (UciMove::new("e8g8")?, -1000000),
                (UciMove::new("e1g1")?, 0),
            ],
        })
    }

    #[test]
    fn case_2() -> Result<()> {
        execute_case(TestCase {
            board: crate::start(),
            moves_evals: vec![
                (UciMove::new("d2d4")?, 1),
                (UciMove::new("d7d5")?, 0),
                (UciMove::new("e2e4")?, 10),
                (UciMove::new("e7e5")?, 0),
                (UciMove::new("a2a3")?, 0),
                (UciMove::new("g8f6")?, -100),
                (UciMove::new("b1c3")?, 900), // w
                (UciMove::new("b8a6")?, -100),
                (UciMove::new("g1f3")?, 0),
                (UciMove::new("c8d7")?, -100000),
                (UciMove::new("c1d2")?, 0),
                (UciMove::new("f8b4")?, -10000),
                (UciMove::new("f1b5")?, 0),
                (UciMove::new("d8e7")?, 0),
                (UciMove::new("d1e2")?, 0),
                // Castle queenside
                (UciMove::new("e8c8")?, -10000000),
                (UciMove::new("e1c1")?, 0),
            ],
        })
    }

    fn execute_case(case: TestCase) -> Result<()> {
        execute_case_impl(case.reflect())?;
        execute_case_impl(case)?;
        Ok(())
    }

    fn execute_case_impl(case: TestCase) -> Result<()> {
        let mut board = case.board;
        let mut component = OpeningComponent::new(dummy_rewards());
        for (uci_mv, expected_eval) in case.moves_evals {
            let curr_eval = component.static_eval();
            let mv = board.parse_uci(uci_mv.as_str())?;
            component.make(&mv);
            assert_eq!(
                expected_eval,
                component.static_eval(),
                "make {}",
                uci_mv.as_str()
            );
            component.unmake(&mv);
            assert_eq!(
                curr_eval,
                component.static_eval(),
                "unmake {}",
                uci_mv.as_str()
            );
            component.make(&mv);
            board.make(mv)?;
        }
        Ok(())
    }

    struct TestCase {
        board: Board,
        moves_evals: Vec<(UciMove, i32)>,
    }

    impl Reflectable for TestCase {
        fn reflect(&self) -> Self {
            TestCase {
                board: self.board.reflect(),
                moves_evals: self.moves_evals.reflect(),
            }
        }
    }
}
