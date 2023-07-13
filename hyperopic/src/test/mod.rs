use crate::moves::Move;
use crate::position::Position;
use crate::{reflect_corner, reflect_piece, reflect_side, reflect_square, Board, Symmetric};
use std::array;

mod control;
mod make;

pub fn assert_boards_equal(expected: Board, actual: Board) {
    assert_eq!(expected, actual, "expected ^ actual {:#064b}", expected ^ actual)
}

impl Symmetric for Move {
    fn reflect(&self) -> Self {
        use crate::moves::Move::*;
        use crate::{reflect_corner, reflect_piece, reflect_side, reflect_square};
        match self {
            Null => Null,
            Castle { corner } => Castle { corner: reflect_corner(*corner) },
            Standard { moving, from, dest, capture } => Standard {
                moving: reflect_piece(*moving),
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                capture: capture.map(|p| reflect_piece(p)),
            },
            Enpassant { side, from, dest, capture } => Enpassant {
                side: reflect_side(*side),
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                capture: reflect_square(*capture),
            },
            Promotion { from, dest, promoted, capture } => Promotion {
                from: reflect_square(*from),
                dest: reflect_square(*dest),
                promoted: reflect_piece(*promoted),
                capture: capture.map(|p| reflect_piece(p)),
            },
        }
    }
}

impl Symmetric for Position {
    fn reflect(&self) -> Self {
        let mut cloned = self.clone();
        let mut moves = vec![];
        while let Ok(m) = cloned.unmake() {
            moves.insert(0, m);
        }
        let mut reflected = Position::new(
            reflect_side(cloned.active),
            cloned.enpassant.map(|sq| reflect_square(sq)),
            cloned.clock,
            array::from_fn(|c| cloned.castling_rights[reflect_corner(c)]),
            array::from_fn(|sq| cloned.piece_locs[reflect_square(sq)].map(|p| reflect_piece(p))),
        );
        moves.into_iter().for_each(|m| reflected.make(m.reflect()).unwrap());
        reflected
    }
}
