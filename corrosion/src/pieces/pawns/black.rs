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
        let mut result = ((loc - RANKS[0]) << 8) - all;
        if BlackPawn::on_start_rank(loc) && !result.is_empty() {
            result = result | ((loc.as_set() << 16) - all)
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
        assert_eq!(D2 | F2, BlackPawn.controlset(E3, A1 | B6, D8 | D4));
        assert_eq!(F7 | D7, BlackPawn.controlset(E8, A1 | B6, D8 | D4));
        assert_eq!(B2.as_set(), BlackPawn.controlset(A3, A4 | C5, F4 | H8));
        assert_eq!(G2.as_set(), BlackPawn.controlset(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moveset() {
        assert_eq!(D3 | D4, BlackPawn.moveset(D2, E2 | D5, G1 | F7));
        assert_eq!(D3.as_set(), BlackPawn.moveset(D2, D4 | G6, A2 | D7));
        assert_eq!(BitBoard::EMPTY, BlackPawn.moveset(D2, D3 | A1, B5 | D5));
        assert_eq!(G7.as_set(), BlackPawn.moveset(G6, A1 | F5, H6 | B3));
        assert_eq!(BitBoard::EMPTY, BlackPawn.moveset(G6, A1 | G7, H6 | B3));
        assert_eq!(BitBoard::EMPTY, BlackPawn.moveset(G6, A1 | A5, H6 | G7));
        assert_eq!(BitBoard::EMPTY, BlackPawn.moveset(A8, D3 | H7, F4 | C3));
    }
}
