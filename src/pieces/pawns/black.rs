use crate::pieces::{Piece, BlackPawn};
use crate::square::{Square, constants::*};
use crate::bitboard::{BitBoard, simple::RANKS};
use super::BLACK_CONTROL;

impl BlackPawn {
    fn on_start_rank(loc: Square) -> bool {
        !(loc & RANKS[6]).is_empty()
    }
}

impl Piece for BlackPawn {
    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        BLACK_CONTROL[loc.i as usize]
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let all = white | black;
        let mut result = ((loc - RANKS[0]) >> 8) - all;
        if BlackPawn::on_start_rank(loc) && !result.is_empty() {
            result = result | ((loc.lift() >> 16) - all)
        }
        result | self.attackset(loc, white, black)
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(loc, white, black) & white
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_on_start_rank() {
        assert_eq!(true, BlackPawn::on_start_rank(B7));
        assert_eq!(true, BlackPawn::on_start_rank(A7));
        assert_eq!(false, BlackPawn::on_start_rank(D4));
        assert_eq!(false, BlackPawn::on_start_rank(H8));
    }

    #[test]
    fn test_controlset() {
        let bp = BlackPawn;
        assert_eq!(D2 | F2, bp.controlset(E3, A1 | B6, D8 | D4));
        assert_eq!(F7 | D7, bp.controlset(E8, A1 | B6, D8 | D4));
        assert_eq!(B2.lift(), bp.controlset(A3, A4 | C5, F4 | H8));
        assert_eq!(G2.lift(), bp.controlset(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moveset() {
        let (bp, zero) = (BlackPawn, BitBoard::EMPTY);
        assert_eq!(D1.lift(), bp.moveset(D2, E2 | D5, G1 | F7));
        assert_eq!(G6 | G5, bp.moveset(G7, A1 | F5, H6 | B3));
        assert_eq!(zero, bp.moveset(G7, A1 | G6, H6 | B3));
        assert_eq!(G6.lift(), bp.moveset(G7, A1 | G8, G5 | B3));
    }
}
