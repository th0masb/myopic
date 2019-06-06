use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;

mod kings;
mod knights;
mod pawns;
mod sliding;

/// Value type wrapping a single integer representing one of the 12
/// different pieces in a game of chess.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Piece {
    WP = 0,
    WN = 1,
    WB = 2,
    WR = 3,
    WQ = 4,
    WK = 5,

    BP = 6,
    BN = 7,
    BB = 8,
    BR = 9,
    BQ = 10,
    BK = 11,
}
impl Piece {
    /// Create an iterator traversing over all pieces in order.
    pub fn iter() -> impl Iterator<Item = Piece> {
        ALL.iter().cloned()
    }

    /// Returns the side that this piece belongs to.
    pub fn side(self) -> Side {
        if (self as u8) < 6 {
            Side::White
        } else {
            Side::Black
        }
    }

    /// Checks whether this piece is either a white or black pawn.
    pub fn is_pawn(self) -> bool {
        (self as u8) % 6 == 0
    }

    /// Checks whether this piece is either a white or black knight.
    pub fn is_knight(self) -> bool {
        (self as u8) % 6 == 1
    }

    /// Computes the control set for this piece given it's location and the
    /// locations of all the white and black pieces on the board.
    pub fn control(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::CONTROL_FN[self as usize](loc, whites, blacks)
    }

    /// Computes the set of legal moves for this piece given it's location
    /// and the locations of all the white and black pieces on the board.
    /// Note that this method does not take into account special restrictions
    /// for or due to the king, e.g. can't move in such a way to put the king
    /// into check.
    pub fn moves(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::MOVE_FN[self as usize](loc, whites, blacks)
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

/// We reflect a piece to it's correspondent on the opposite side.
impl Reflectable for Piece {
    fn reflect(&self) -> Self {
        ALL[(*self as usize + 6) % 12]
    }
}

/// Returns the king which belongs to the given side.
pub fn king(side: Side) -> Piece {
    match side {
        Side::White => Piece::WK,
        Side::Black => Piece::BK,
    }
}

/// Returns the pawn which belongs to the given side.
pub fn pawn(side: Side) -> Piece {
    match side {
        Side::White => Piece::WP,
        Side::Black => Piece::BP,
    }
}

/// Returns a slice containing all pieces belonging to the given side.
pub fn on_side<'a>(side: Side) -> &'a [Piece] {
    match side {
        Side::White => &WHITE,
        Side::Black => &BLACK,
    }
}

/// Constant piece groupings.
const ALL: [Piece; 12] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

const WHITE: [Piece; 6] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
];

const BLACK: [Piece; 6] = [
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

const PAWNS: [Piece; 2] = [Piece::WP, Piece::BP];
const KNIGHTS: [Piece; 2] = [Piece::WN, Piece::BN];
const BISHOPS: [Piece; 2] = [Piece::WB, Piece::BB];
const ROOKS: [Piece; 2] = [Piece::WR, Piece::BR];
const QUEENS: [Piece; 2] = [Piece::WQ, Piece::BQ];
const KINGS: [Piece; 2] = [Piece::WK, Piece::BK];
