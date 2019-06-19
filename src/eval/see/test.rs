//use crate::board::

use crate::board::test_board::TestBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::base::Side;

fn value(piece: Piece) -> i32 {
    match piece.side() {
        Side::White => piece as i32,
        Side::Black => -(piece as i32),
    }
}

struct TestCase {
    board: TestBoard,
    expected: Vec<(Square, Square, i32)>,
}
