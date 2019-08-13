use crate::base::castlezone::CastleZone;
use crate::base::square::Square::*;
use crate::base::Reflectable;
use crate::board;
use crate::board::Board;
use crate::board::Move;
use crate::board::Move::*;
use crate::eval::evalboardimpl::SimpleEvalBoard;
use crate::pieces::Piece::*;

#[derive(Clone, Eq, PartialEq)]
struct TestCase<B: Board> {
    start_position: B,
    moves: Vec<Move>,
}

impl<B: Board> Reflectable for TestCase<B> {
    fn reflect(&self) -> Self {
        TestCase {
            start_position: self.start_position.reflect(),
            moves: self.moves.reflect(),
        }
    }
}

fn execute_test<B: Board>(test_case: TestCase<B>) {
    execute_test_impl(test_case.clone());
    execute_test_impl(test_case.reflect());
}

fn execute_test_impl<B: Board>(test_case: TestCase<B>) {
    let mut start = SimpleEvalBoard::new(test_case.start_position);
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
        start_position: board::from_fen(start_fen).unwrap(),
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
            Enpassant(E5),
            Standard(BK, C8, B8),
            Standard(WP, D6, E7),
            Standard(BR, H8, H7),
            Promotion(E7, D8, WQ),
        ],
    );
}
