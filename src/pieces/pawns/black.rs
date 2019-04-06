use crate::base::bitboard::{simple::RANKS, BitBoard};
use crate::base::square::Square;
use crate::pieces::{BlackPawn, Piece};

use super::BLACK_CONTROL;

/// Piece trait implementation for the black pawn struct. the control sets for
/// each base.square are cached whereas the moves is currently calculated each time.
impl Piece for BlackPawn {
    fn index(&self) -> usize {
        6
    }

    fn id(&self) -> &'static str {
        "bp"
    }

    fn control(&self, loc: Square, _white: BitBoard, _black: BitBoard) -> BitBoard {
        BLACK_CONTROL[loc.i as usize]
    }

    fn moves(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let all = white | black;
        let mut result = ((loc - RANKS[0]) >> 8) - all;
        if on_start_rank(loc) && !result.is_empty() {
            result = result | ((loc.lift() >> 16) - all)
        }
        result | self.attacks(loc, white, black)
    }

    fn attacks(&self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(loc, white, black) & white
    }
}

/// Computes whether a black pawn starts a game of standard chess on a given
/// base.square.
fn on_start_rank(loc: Square) -> bool {
    !(loc & RANKS[6]).is_empty()
}

#[cfg(test)]
mod test {
    use crate::base::square::constants::*;

    use super::*;

    #[test]
    fn test_on_start_rank() {
        assert_eq!(true, on_start_rank(B7));
        assert_eq!(true, on_start_rank(A7));
        assert_eq!(false, on_start_rank(D4));
        assert_eq!(false, on_start_rank(H8));
    }

    #[test]
    fn test_control() {
        let bp = BlackPawn;
        assert_eq!(D2 | F2, bp.control(E3, A1 | B6, D8 | D4));
        assert_eq!(F7 | D7, bp.control(E8, A1 | B6, D8 | D4));
        assert_eq!(B2.lift(), bp.control(A3, A4 | C5, F4 | H8));
        assert_eq!(G2.lift(), bp.control(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moves() {
        let (bp, zero) = (BlackPawn, BitBoard::EMPTY);
        assert_eq!(D1.lift(), bp.moves(D2, E2 | D5, G1 | F7));
        assert_eq!(G6 | G5, bp.moves(G7, A1 | F5, H6 | B3));
        assert_eq!(zero, bp.moves(G7, A1 | G6, H6 | B3));
        assert_eq!(G6.lift(), bp.moves(G7, A1 | G8, G5 | B3));
    }
}
