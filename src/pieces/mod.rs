use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

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

    const CONTROL_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 2] = [
        knights::control,
        kings::control
    ];

    const MOVE_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 2] = [
        knights::white_moves,
        kings::control
    ];
}

//pub type PieceRef = &'static dyn Piece;
//pub trait Piece {
//    fn index(&self) -> usize;
//
//    fn id(&self) -> &str;
//
//    fn side(&self) -> Side;
//
//    fn control(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
//
//    fn moves(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
//
//    fn attacks(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard;
//
//    fn is_pawn(&self) -> bool {
//        self.index() % 6 == 0
//    }
//
//    fn empty_control(&self, location: Square) -> BitBoard {
//        self.control(location, BitBoard::EMPTY, BitBoard::EMPTY)
//    }
//}
//
//impl Debug for Piece {
//    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//        write!(f, "{}", self.id())
//    }
//}
//
//impl PartialEq<Piece> for Piece {
//    fn eq(&self, other: &Piece) -> bool {
//        self.index() == other.index()
//    }
//}

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

pub fn army(side: Side) -> [PieceRef; 5] {
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

///// Encapsulated singleton structs for each piece type.
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhitePawn;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackPawn;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhiteKnight;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackKnight;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhiteBishop;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackBishop;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhiteRook;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackRook;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhiteQueen;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackQueen;
//
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct WhiteKing;
//#[derive(Copy, Clone, PartialEq, Eq)]
//struct BlackKing;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indices() {
        assert_eq!("wp", ALL[WhitePawn.index()].id());
        assert_eq!("wn", ALL[WhiteKnight.index()].id());
        assert_eq!("wb", ALL[WhiteBishop.index()].id());
        assert_eq!("wr", ALL[WhiteRook.index()].id());
        assert_eq!("wq", ALL[WhiteQueen.index()].id());
        assert_eq!("wk", ALL[WhiteKing.index()].id());
        assert_eq!("bp", ALL[BlackPawn.index()].id());
        assert_eq!("bn", ALL[BlackKnight.index()].id());
        assert_eq!("bb", ALL[BlackBishop.index()].id());
        assert_eq!("br", ALL[BlackRook.index()].id());
        assert_eq!("bq", ALL[BlackQueen.index()].id());
        assert_eq!("bk", ALL[BlackKing.index()].id());
    }
}
