use crate::bitboard::BitBoard;
use crate::pieces::BlackBishop;
use crate::pieces::BlackQueen;
use crate::pieces::BlackRook;
use crate::pieces::Piece;
use crate::pieces::WhiteBishop;
use crate::pieces::WhiteQueen;
use crate::pieces::WhiteRook;
use crate::square::Square;

impl Piece for WhiteQueen {
    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.controlset(loc, white, black) | WhiteRook.controlset(loc, white, black)
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.moveset(loc, white, black) | WhiteRook.moveset(loc, white, black)
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WhiteBishop.attackset(loc, white, black) | WhiteRook.attackset(loc, white, black)
    }
}

impl Piece for BlackQueen {
    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.controlset(loc, white, black) | BlackRook.controlset(loc, white, black)
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.moveset(loc, white, black) | BlackRook.moveset(loc, white, black)
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BlackBishop.attackset(loc, white, black) | BlackRook.attackset(loc, white, black)
    }
}
