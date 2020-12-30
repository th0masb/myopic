use crate::eval::EvalComponent;
use crate::{CastleZone, Move, Piece, Reflectable};
use myopic_board::{Side, Square};

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
    white: DevTracker,
    black: DevTracker,
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
            white: DevTracker {
                e_pawn: PieceTracker::new(Square::E2),
                d_pawn: PieceTracker::new(Square::D2),
                b_knight: PieceTracker::new(Square::B1),
                g_knight: PieceTracker::new(Square::G1),
                c_bishop: PieceTracker::new(Square::C1),
                f_bishop: PieceTracker::new(Square::F1),
            },
            black: DevTracker {
                e_pawn: PieceTracker::new(Square::E7),
                d_pawn: PieceTracker::new(Square::D7),
                b_knight: PieceTracker::new(Square::B8),
                g_knight: PieceTracker::new(Square::G8),
                c_bishop: PieceTracker::new(Square::C8),
                f_bishop: PieceTracker::new(Square::F8),
            },
        }
    }

    fn tracker(&mut self, side: Side) -> &mut DevTracker {
        match side {
            Side::White => &mut self.white,
            Side::Black => &mut self.black,
        }
    }
}

#[derive(Clone)]
struct DevTracker {
    e_pawn: PieceTracker,
    d_pawn: PieceTracker,
    b_knight: PieceTracker,
    g_knight: PieceTracker,
    c_bishop: PieceTracker,
    f_bishop: PieceTracker,
}

#[derive(Clone)]
struct PieceTracker {
    loc: Square,
    count: usize,
}

impl PieceTracker {
    fn new(start: Square) -> PieceTracker {
        PieceTracker {
            loc: start,
            count: 0,
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
}

impl Reflectable for OpeningComponent {
    fn reflect(&self) -> Self {
        OpeningComponent {
            rewards: self.rewards.clone(),
            white: self.black.clone(),
            black: self.white.clone(),
            score: -self.score,
        }
    }
}

pub fn parity(side: Side) -> i32 {
    match side {
        Side::White => 1,
        Side::Black => -1,
    }
}

impl EvalComponent for OpeningComponent {
    fn static_eval(&self) -> i32 {
        self.score
    }

    fn make(&mut self, mv: &Move) {
        match mv {
            &Move::Standard {
                moving, from, dest, ..
            } => match moving {
                Piece::WP | Piece::BP => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.d_pawn.loc == from {
                        if tracker.d_pawn.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.d_pawn;
                        }
                    } else if tracker.e_pawn.loc == from {
                        if tracker.e_pawn.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.e_pawn;
                        }
                    }
                }
                Piece::WN | Piece::BN => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.b_knight.loc == from {
                        if tracker.b_knight.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.b_knight;
                        }
                    } else if tracker.g_knight.loc == from {
                        if tracker.g_knight.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.g_knight;
                        }
                    }
                }
                Piece::WB | Piece::BB => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.c_bishop.loc == from {
                        if tracker.c_bishop.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.c_bishop;
                        }
                    } else if tracker.f_bishop.loc == from {
                        if tracker.f_bishop.move_forward(dest) == 1 {
                            self.score += parity(side) * self.rewards.f_bishop;
                        }
                    }
                }
                _ => {}
            },
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
                moving, from, dest, ..
            } => match moving {
                Piece::WP | Piece::BP => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.d_pawn.loc == dest {
                        if tracker.d_pawn.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.d_pawn;
                        }
                    } else if tracker.e_pawn.loc == dest {
                        if tracker.e_pawn.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.e_pawn;
                        }
                    }
                }
                Piece::WN | Piece::BN => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.b_knight.loc == dest {
                        if tracker.b_knight.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.b_knight;
                        }
                    } else if tracker.g_knight.loc == dest {
                        if tracker.g_knight.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.g_knight;
                        }
                    }
                }
                Piece::WB | Piece::BB => {
                    let (side, tracker) = (moving.side(), self.tracker(moving.side()));
                    if tracker.c_bishop.loc == dest {
                        if tracker.c_bishop.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.c_bishop;
                        }
                    } else if tracker.f_bishop.loc == dest {
                        if tracker.f_bishop.move_backward(from) == 0 {
                            self.score -= parity(side) * self.rewards.f_bishop;
                        }
                    }
                }
                _ => {}
            },
            &Move::Castle { zone, .. } => match zone {
                CastleZone::WK => self.score -= self.rewards.k_castle,
                CastleZone::WQ => self.score -= self.rewards.q_castle,
                CastleZone::BK => self.score += self.rewards.k_castle,
                CastleZone::BQ => self.score += self.rewards.q_castle,
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::eval::additional_components::opening::{OpeningComponent, OpeningRewards};
    use crate::eval::EvalComponent;
    use crate::{Board, EvalBoard, Reflectable, UciMove};
    use anyhow::Result;
    use myopic_board::{ChessBoard, Move};

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
            board: EvalBoard::start(),
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
            board: EvalBoard::start(),
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
            board: EvalBoard::start(),
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
        board: EvalBoard<Board>,
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
