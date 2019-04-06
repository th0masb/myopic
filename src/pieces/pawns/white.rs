use crate::base::bitboard::{simple::RANKS, BitBoard};
use crate::base::square::Square;
use crate::pieces::{Piece, WhitePawn};

use super::WHITE_CONTROL;

/// Piece trait implementation for the white pawn struct. the control sets for
/// each base.square are cached whereas the moves is currently calculated each time.
impl Piece for WhitePawn {
    fn control(self, loc: Square, _white: BitBoard, _black: BitBoard) -> BitBoard {
        WHITE_CONTROL[loc.i as usize]
    }

    fn moves(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let all = white | black;
        let mut result = ((loc - RANKS[7]) << 8) - all;
        if on_start_rank(loc) && !result.is_empty() {
            result = result | ((loc.lift() << 16) - all)
        }
        result | self.attacks(loc, white, black)
    }

    fn attacks(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(loc, white, black) & black
    }
}

/// Computes whether a white pawn starts a game of standard chess on a given
/// base.square.
fn on_start_rank(loc: Square) -> bool {
    !(loc & RANKS[1]).is_empty()
}

#[cfg(test)]
mod test {
    use crate::base::square::constants::*;

    use super::*;

    #[test]
    fn test_on_start_rank() {
        assert_eq!(true, on_start_rank(B2));
        assert_eq!(true, on_start_rank(A2));
        assert_eq!(false, on_start_rank(D4));
        assert_eq!(false, on_start_rank(G7));
    }

    #[test]
    fn test_control() {
        assert_eq!(D4 | F4, WhitePawn.control(E3, A1 | B6, D8 | D4));
        assert_eq!(BitBoard::EMPTY, WhitePawn.control(E8, A1 | B6, D8 | D4));
        assert_eq!(B4.lift(), WhitePawn.control(A3, A4 | C5, F4 | H8));
        assert_eq!(G4.lift(), WhitePawn.control(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moves() {
        assert_eq!(D3 | D4 | E3, WhitePawn.moves(D2, E2 | D5, G1 | F7 | E3));
        assert_eq!(D3.lift(), WhitePawn.moves(D2, D4 | G6, A2 | D7));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moves(D2, D3 | A1, B5 | D5));
        assert_eq!(G7.lift(), WhitePawn.moves(G6, A1 | F5, H6 | B3));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moves(G6, A1 | G7, H6 | B3));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moves(G6, A1 | A5, H6 | G7));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moves(A8, D3 | H7, F4 | C3));
    }
}
