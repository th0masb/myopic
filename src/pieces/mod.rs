use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

use crate::base::bitboard::BitBoard;
use crate::base::Side;
use crate::base::square::Square;

mod kings;
mod knights;
mod pawns;
mod sliding;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Piece(usize);
impl Piece {
    pub fn index(self) -> usize {
        self.0
    }

    pub fn side(self) -> Side {
        if self.0 < 6 {
            Side::White
        } else {
            Side::Black
        }
    }

    pub fn is_pawn(self) -> bool {
        self.0 % 6 == 0
    }

    pub fn control(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::CONTROL_FN[self.0](loc, whites, blacks)
    }

    pub fn moves(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::MOVE_FN[self.0](loc, whites, blacks)
    }

    const CONTROL_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_control,
        knights::control,
        sliding::bishops::control,
        sliding::rooks::control,
        sliding::queens::control,
        kings::control,
        pawns::black_control,
        knights::control,
        sliding::bishops::control,
        sliding::rooks::control,
        sliding::queens::control,
        kings::control,
    ];

    const MOVE_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_moves,
        knights::white_moves,
        sliding::bishops::white_moves,
        sliding::rooks::white_moves,
        sliding::queens::white_moves,
        kings::white_moves,
        pawns::black_moves,
        knights::black_moves,
        sliding::bishops::black_moves,
        sliding::rooks::black_moves,
        sliding::queens::black_moves,
        kings::black_moves,
    ];
}

pub fn king(side: Side) -> Piece {
    match side {
        Side::White => WK,
        Side::Black => BK,
    }
}

pub fn pawn(side: Side) -> Piece {
    match side {
        Side::White => WP,
        Side::Black => BP,
    }
}

pub fn on_side(side: Side) -> &'static [Piece] {
    match side {
        Side::White => &WHITE,
        Side::Black => &BLACK,
    }
}

pub fn army(side: Side) -> [Piece; 5] {
    match side {
        Side::White => [WP, WN, WB, WR, WQ],
        Side::Black => [BP, BN, BB, BR, BQ],
    }
}

/// Constant static references to each white piece.
pub const WP: Piece = Piece(0);
pub const WN: Piece = Piece(1);
pub const WB: Piece = Piece(2);
pub const WR: Piece = Piece(3);
pub const WQ: Piece = Piece(4);
pub const WK: Piece = Piece(5);

/// Constant static references to each black piece.
pub const BP: Piece = Piece(6);
pub const BN: Piece = Piece(7);
pub const BB: Piece = Piece(8);
pub const BR: Piece = Piece(9);
pub const BQ: Piece = Piece(10);
pub const BK: Piece = Piece(11);

/// Constant piece groupings.
pub const ALL: [Piece; 12] = [WP, WN, WB, WR, WQ, WK, BP, BN, BB, BR, BQ, BK];

pub const WHITE: [Piece; 6] = [WP, WN, WB, WR, WQ, WK];
pub const BLACK: [Piece; 6] = [BP, BN, BB, BR, BQ, BK];

pub const PAWNS:   [Piece; 2] = [WP, BP];
pub const KNIGHTS: [Piece; 2] = [WN, BN];
pub const BISHOPS: [Piece; 2] = [WB, BB];
pub const ROOKS:   [Piece; 2] = [WR, BR];
pub const QUEENS:  [Piece; 2] = [WQ, BQ];
pub const KINGS:   [Piece; 2] = [WK, BK];
