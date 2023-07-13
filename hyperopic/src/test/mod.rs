use crate::Board;

mod make;
mod control;

pub fn assert_boards_equal(expected: Board, actual: Board) {
    assert_eq!(expected, actual, "expected ^ actual {:#064b}", expected ^ actual)
}