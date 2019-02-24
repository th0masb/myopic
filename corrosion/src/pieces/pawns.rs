use crate::bitboard::{simple::*, BitBoard};
use crate::dir::{Dir, N, S};
use crate::pieces::Piece;
use crate::side::{Side, Side::Black, Side::White};
use crate::square::constants::{A1, A6, H3, H8, SQUARES};
use crate::square::Square;

fn compute_all_empty_board_moves(side: Side) -> Vec<BitBoard> {
    SQUARES
        .iter()
        .map(|&sq| compute_empty_board_moves(side, sq))
        .collect()
}

fn compute_empty_board_moves(side: Side, loc: Square) -> BitBoard {
    let dir = if side == White { N } else { S };
    let ntake = if on_first_rank(side, loc) { 2 } else { 1 };
    loc.search_vec(dir).into_iter().take(ntake).collect()
}

fn on_first_rank(side: Side, loc: Square) -> bool {
    match side {
        White => !(loc & RANKS[1]).is_empty(),
        Black => !(loc & RANKS[6]).is_empty(),
    }
}

fn compute_all_empty_board_control(side: Side) -> Vec<BitBoard> {
    SQUARES
        .iter()
        .map(|&sq| compute_empty_board_control(side, sq))
        .collect()
}

fn compute_empty_board_control(side: Side, loc: Square) -> BitBoard {
    let (x, left, right) = (loc.as_set() - RANKS[0], FILES[7], FILES[0]);
    match side {
        White => ((x - left) << 9u8) | ((x - right) << 7u8),
        Black => ((x - left) >> 7u8) | ((x - right) >> 9u8),
    }
}

lazy_static! {
    static ref WHITE_MOVES: Vec<BitBoard> = compute_all_empty_board_moves(White);
    static ref BLACK_MOVES: Vec<BitBoard> = compute_all_empty_board_moves(Black);
    static ref WHITE_CONTROL: Vec<BitBoard> = compute_all_empty_board_control(White);
    static ref BLACK_CONTROL: Vec<BitBoard> = compute_all_empty_board_control(Black);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::side::Side;
    use crate::square::constants::*;

    #[test]
    fn test_compute_empty_board_control() {
        let compute_control = |side, loc| compute_empty_board_control(side, loc);
        assert_eq!(D4 | F4, compute_control(White, E3));
        assert_eq!(B6.as_set(), compute_control(White, A5));
        assert_eq!(G5.as_set(), compute_control(White, H4));
        assert_eq!(D4 | F4, compute_control(Black, E5));
        assert_eq!(B6.as_set(), compute_control(Black, A7));
        assert_eq!(G5.as_set(), compute_control(Black, H6));
    }

    #[test]
    fn test_on_first_rank() {
        assert_eq!(true, on_first_rank(White, D2));
        assert_eq!(true, on_first_rank(White, A2));
        assert_eq!(false, on_first_rank(White, D3));
        assert_eq!(true, on_first_rank(Black, H7));
        assert_eq!(false, on_first_rank(Black, H6));
    }
    #[test]
    fn test_compute_empty_board_moves() {
        assert_eq!(D4.as_set(), compute_empty_board_moves(White, D3));
        assert_eq!(G3 | G4, compute_empty_board_moves(White, G2));
        assert_eq!(A5.as_set(), compute_empty_board_moves(Black, A6));
        assert_eq!(D5 | D6, compute_empty_board_moves(Black, D7));
    }
}

pub struct WhitePawn;
impl Piece for WhitePawn {
    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        WHITE_CONTROL[loc.i as usize]
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let all = white | black;
        //let next = ()
        if on_first_rank(White, loc) {

        } else {

        }
        unimplemented!()
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.controlset(loc, white, black) & black
    }
}

pub struct BlackPawn;
