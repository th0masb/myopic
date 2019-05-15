use crate::base::square::Square;
use crate::board::testutils::TestBoard;
use crate::base::castlezone::CastleZone;
use crate::board::Move;
use crate::base::bitboard::BitBoard;

type PrototypeStandardMove = (BitBoard, BitBoard);
type PrototypePromotionMove = (BitBoard, BitBoard);

//fn s(source: Square, targets: BitBoard) -> Vec<PrototypeStandardMove> {
//    targets.into_iter().map(|target| (source, target)).collect()
//}

//fn p(source: Square, targets: BitBoard) -> Vec<Move> {
//    targets.iter().flat_map(|target| Move::promotions())
//}

struct TestCase {
    board: TestBoard,

    expected_enpassant_moves: Vec<Move>,

    expected_castle_moves: Vec<CastleZone>,
    expected_promotion_moves: Vec<PrototypePromotionMove>,
    expected_standard_moves: Vec<PrototypeStandardMove>,

    expected_promotion_attacks: Vec<PrototypePromotionMove>,
    expected_standard_attacks: Vec<PrototypeStandardMove>,
}

fn execute_test(case: TestCase) {
    let board = case.board.to_board();
    // First generate the expected moveset
    unimplemented!()
}
