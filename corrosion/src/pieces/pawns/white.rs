use super::WHITE_CONTROL;
use crate::bitboard::{simple::RANKS, BitBoard};
use crate::pieces::{Piece, WhitePawn};
use crate::square::{constants::*, Square};

impl WhitePawn {
    fn on_start_rank(loc: Square) -> bool {
        !(loc & RANKS[1]).is_empty()
    }
}

impl Piece for WhitePawn {
    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WHITE_CONTROL[loc.i as usize]
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let all = white | black;
        let mut result = ((loc - RANKS[7]) << 8) - all;
        if WhitePawn::on_start_rank(loc) && !result.is_empty() {
            result = result | ((loc.as_set() << 16) - all)
        }
        result | self.attackset(loc, white, black)
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(loc, white, black) & black
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_on_start_rank() {
        assert_eq!(true, WhitePawn::on_start_rank(B2));
        assert_eq!(true, WhitePawn::on_start_rank(A2));
        assert_eq!(false, WhitePawn::on_start_rank(D4));
        assert_eq!(false, WhitePawn::on_start_rank(G7));
    }

    #[test]
    fn test_controlset() {
        assert_eq!(D4 | F4, WhitePawn.controlset(E3, A1 | B6, D8 | D4));
        assert_eq!(BitBoard::EMPTY, WhitePawn.controlset(E8, A1 | B6, D8 | D4));
        assert_eq!(B4.as_set(), WhitePawn.controlset(A3, A4 | C5, F4 | H8));
        assert_eq!(G4.as_set(), WhitePawn.controlset(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moveset() {
        assert_eq!(D3 | D4 | E3, WhitePawn.moveset(D2, E2 | D5, G1 | F7 | E3));
        assert_eq!(D3.as_set(), WhitePawn.moveset(D2, D4 | G6, A2 | D7));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moveset(D2, D3 | A1, B5 | D5));
        assert_eq!(G7.as_set(), WhitePawn.moveset(G6, A1 | F5, H6 | B3));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moveset(G6, A1 | G7, H6 | B3));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moveset(G6, A1 | A5, H6 | G7));
        assert_eq!(BitBoard::EMPTY, WhitePawn.moveset(A8, D3 | H7, F4 | C3));
    }
}
