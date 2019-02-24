use crate::bitboard::{BitBoard, simple::*};
use crate::dir::{Dir, N, S};
use crate::pieces::Piece;
use crate::side::{Side, Side::Black, Side::White};
use crate::square::constants::{SQUARES, A1, A6, H3, H8};
use crate::square::Square;

fn compute_all_empty_board_moves(side: Side) -> Vec<BitBoard> {
    SQUARES.iter().map(|&sq| compute_empty_board_moves(side, sq)).collect()
}

fn compute_empty_board_moves(side: Side, loc: Square) -> BitBoard {
    let dir = match side {White => N, _ => S,};
    let ntake = if on_first_rank(side, loc) {2} else {1};
    println!("{}", loc.search(dir));
    loc.search_one(&vec![dir])
}

fn on_first_rank(side: Side, loc: Square) -> bool {
    let (lb, ub) = match side { White => (A1, H3), _ => (A6, H8), };
    lb < loc && loc < ub
}

lazy_static! {
    static ref WHITE_MOVES: Vec<BitBoard> = compute_all_empty_board_moves(White);
    static ref BLACK_MOVES: Vec<BitBoard> = compute_all_empty_board_moves(Black);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::constants::*;
    use crate::side::Side;

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
        //assert_eq!(D5 | D6, compute_empty_board_moves(Black, D7));
    }
}

pub struct WhitePawn;
impl Piece for WhitePawn {

    fn controlset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        let (x, left, right) = (loc.as_set() - RANKS[0], FILES[7], FILES[0]);
        ((x - left) << 9u8) | ((x - right) << 7u8)
    }

    fn moveset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        unimplemented!()
    }

    fn attackset(self, loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        unimplemented!()
    }
}

pub struct BlackPawn;
