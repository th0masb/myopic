use crate::base::bitboard::BitBoard;
use crate::base::Side;
use crate::base::square::Square;
use crate::pieces::BlackBishop;
use crate::pieces::BlackQueen;
use crate::pieces::BlackRook;
use crate::pieces::Piece;
use crate::pieces::WhiteBishop;
use crate::pieces::WhiteQueen;
use crate::pieces::WhiteRook;

impl Piece for WhiteQueen {
    fn index(&self) -> usize {
        4
    }

    fn id(&self) -> &'static str {
        "wq"
    }

    fn side(&self) -> Side {
        Side::White
    }

    fn control(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.control(loc, white, black) | WhiteRook.control(loc, white, black)
    }

    fn moves(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.moves(loc, white, black) | WhiteRook.moves(loc, white, black)
    }

    fn attacks(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.attacks(loc, white, black) | WhiteRook.attacks(loc, white, black)
    }
}

impl Piece for BlackQueen {
    fn index(&self) -> usize {
        10
    }

    fn id(&self) -> &'static str {
        "bq"
    }

    fn side(&self) -> Side {
        Side::Black
    }

    fn control(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.control(loc, white, black) | BlackRook.control(loc, white, black)
    }

    fn moves(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.moves(loc, white, black) | BlackRook.moves(loc, white, black)
    }

    fn attacks(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.attacks(loc, white, black) | BlackRook.attacks(loc, white, black)
    }
}
