use crate::eval::EvalComponent;
use crate::{CastleZone, Move, Piece};
use myopic_board::Square;

#[derive(Clone)]
pub struct OpeningRewards {
    pub e_pawn: i32,
    pub d_pawn: i32,
    pub b_knight: i32,
    pub g_knight: i32,
    pub c_bishop: i32,
    pub f_bishop: i32,
    pub k_castle: i32,
    pub q_castle: i32,
}

impl Default for OpeningRewards {
    fn default() -> Self {
        OpeningRewards {
            e_pawn: 200,
            d_pawn: 150,
            b_knight: 100,
            g_knight: 150,
            c_bishop: 100,
            f_bishop: 150,
            k_castle: 200,
            q_castle: 100,
        }
    }
}

#[derive(Clone)]
pub struct OpeningComponent {
    rewards: OpeningRewards,
    score: i32,
    pieces: DevTracker,
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
            score: 0,
            move_dist: 0,
            pieces: DevTracker {
                w_e_pawn: PieceTracker::new(Square::E2, rewards.e_pawn),
                w_d_pawn: PieceTracker::new(Square::D2, rewards.d_pawn),
                w_b_knight: PieceTracker::new(Square::B1, rewards.b_knight),
                w_g_knight: PieceTracker::new(Square::G1, rewards.g_knight),
                w_c_bishop: PieceTracker::new(Square::C1, rewards.c_bishop),
                w_f_bishop: PieceTracker::new(Square::F1, rewards.f_bishop),

                b_e_pawn: PieceTracker::new(Square::E7, -rewards.e_pawn),
                b_d_pawn: PieceTracker::new(Square::D7, -rewards.d_pawn),
                b_b_knight: PieceTracker::new(Square::B8, -rewards.b_knight),
                b_g_knight: PieceTracker::new(Square::G8, -rewards.g_knight),
                b_c_bishop: PieceTracker::new(Square::C8, -rewards.c_bishop),
                b_f_bishop: PieceTracker::new(Square::F8, -rewards.f_bishop),
            },
            rewards,
        }
    }
}

#[derive(Debug, Clone)]
struct DevTracker {
    // whites
    w_e_pawn: PieceTracker,
    w_d_pawn: PieceTracker,
    w_b_knight: PieceTracker,
    w_g_knight: PieceTracker,
    w_c_bishop: PieceTracker,
    w_f_bishop: PieceTracker,
    // blacks
    b_e_pawn: PieceTracker,
    b_d_pawn: PieceTracker,
    b_b_knight: PieceTracker,
    b_g_knight: PieceTracker,
    b_c_bishop: PieceTracker,
    b_f_bishop: PieceTracker,
}

impl DevTracker {
    fn get_piece_trackers(&mut self, p: Piece) -> Vec<&mut PieceTracker> {
        match p {
            Piece::WP => vec![&mut self.w_d_pawn, &mut self.w_e_pawn],
            Piece::WN => vec![&mut self.w_b_knight, &mut self.w_g_knight],
            Piece::WB => vec![&mut self.w_c_bishop, &mut self.w_f_bishop],
            Piece::BP => vec![&mut self.b_d_pawn, &mut self.b_e_pawn],
            Piece::BN => vec![&mut self.b_b_knight, &mut self.b_g_knight],
            Piece::BB => vec![&mut self.b_c_bishop, &mut self.b_f_bishop],
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct PieceTracker {
    /// The most recent location of the piece on the board
    loc: Square,
    /// How many moves this piece has made from it's start
    /// position
    count: usize,
    /// What reward this tracker is associated with
    reward: i32,
    /// The move dist at which this piece was removed
    /// from the board. None if still on board.
    capture_dist: Option<usize>,
}

impl PieceTracker {
    fn new(start: Square, reward: i32) -> PieceTracker {
        PieceTracker {
            loc: start,
            count: 0,
            capture_dist: None,
            reward,
        }
    }

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
            &Move::Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => {
                // Update location of moving piece
                for pt in self.pieces.get_piece_trackers(moving) {
                    if pt.matches_on(from) && pt.move_forward(dest) == 1 {
                        self.score += pt.reward
                    }
                }
                // Remove any captured piece
                if capture.is_some() {
                    for pt in self.pieces.get_piece_trackers(capture.unwrap()) {
                        if pt.matches_on(dest) {
                            pt.remove_piece(self.move_dist);
                        }
                    }
                }
            }
            &Move::Castle { zone, .. } => match zone {
                CastleZone::WK => self.score += self.rewards.k_castle,
                CastleZone::WQ => self.score += self.rewards.q_castle,
                CastleZone::BK => self.score -= self.rewards.k_castle,
                CastleZone::BQ => self.score -= self.rewards.q_castle,
            },
            _ => {}
        }
    }

    fn unmake(&mut self, mv: &Move) {
        match mv {
            &Move::Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => {
                // Update location of moving piece
                for pt in self.pieces.get_piece_trackers(moving) {
                    if pt.matches_on(dest) && pt.move_backward(from) == 0 {
                        self.score -= pt.reward
                    }
                }
                // Replace any captured piece
                if capture.is_some() {
                    for pt in self.pieces.get_piece_trackers(capture.unwrap()) {
                        if pt.matches_off(self.move_dist) {
                            pt.add_piece();
                        }
                    }
                }
            }
            &Move::Castle { zone, .. } => match zone {
                CastleZone::WK => self.score -= self.rewards.k_castle,
                CastleZone::WQ => self.score -= self.rewards.q_castle,
                CastleZone::BK => self.score += self.rewards.k_castle,
                CastleZone::BQ => self.score += self.rewards.q_castle,
            },
            _ => {}
        };
        self.move_dist -= 1;
    }
}

#[cfg(test)]
mod test {
    use crate::eval::additional_components::opening::{OpeningComponent, OpeningRewards};
    use crate::eval::EvalComponent;
    use crate::{Board, EvalBoard, Reflectable, UciMove};
    use anyhow::Result;
    use myopic_board::ChessBoard;

    #[test]
    fn issue_97() -> Result<()> {
        let mut state = EvalBoard::start();
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
            d_pawn: 1,
            e_pawn: 10,
            g_knight: 100,
            b_knight: 1000,
            f_bishop: 10000,
            c_bishop: 100000,
            k_castle: 1000000,
            q_castle: 10000000,
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
