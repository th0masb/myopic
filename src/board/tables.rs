use crate::base::Side;
use crate::base::square::constants::SQUARES;
use crate::base::square::Square;
use crate::pieces::Piece;


/// API method for retrieving the evaluation for a piece at a given location
/// in the midgame.
pub fn midgame_eval(piece: &dyn Piece, location: Square) -> i32 {
    let (side, pindex) = (piece.side(), piece.index() % 6);
    let (tableindex, parity) = match side {
        Side::White => (63 - location.i as usize, 1),
        Side::Black => (63 - reflect(location).i as usize, -1)
    };
    parity * MIDGAME[pindex][tableindex]
}

/// API method for retrieving the evaluation for a piece at a given location
/// in the endgame.
pub fn endgame_eval(piece: &dyn Piece, location: Square) -> i32 {
    let (side, pindex) = (piece.side(), piece.index() % 6);
    let (tableindex, parity) = match side {
        Side::White => (63 - location.i as usize, 1),
        Side::Black => (63 - reflect(location).i as usize, -1)
    };
    parity * ENDGAME[pindex][tableindex]
}

/// Reflects a square through the horizontal line bisecting the chess board.
fn reflect(loc: Square) -> Square {
    let (rank, file) = (loc.rank as usize, loc.file as usize);
    SQUARES[(7 - rank) * 8 + file]
}

#[cfg(test)]
mod test {
    use crate::base::square::constants::*;
    use crate::base::square::Square;
    use crate::board::tables::{endgame_eval, midgame_eval};
    use crate::board::tables::PAWN_MIDGAME;
    use crate::pieces::*;

    use super::reflect;

    #[test]
    fn test_reflect() {
        assert_eq!(A8, reflect(A1));
        assert_eq!(H1, reflect(H8));
        assert_eq!(D3, reflect(D6));
        assert_eq!(D5, reflect(D4));
    }

    #[test]
    fn test_midgame() {
        assert_eq!(30, midgame_eval(&WhitePawn, C6));
        assert_eq!(-30, midgame_eval(&BlackPawn, C3));

        assert_eq!(10, midgame_eval(&WhiteKnight, D3));
        assert_eq!(-10, midgame_eval(&BlackKnight, D6));

        assert_eq!(25, midgame_eval(&WhiteBishop, D4));
        assert_eq!(-25, midgame_eval(&BlackBishop, D5));

        assert_eq!(5, midgame_eval(&WhiteRook, D2));
        assert_eq!(-5, midgame_eval(&BlackRook, D7));

        assert_eq!(5, midgame_eval(&WhiteQueen, B3));
        assert_eq!(-5, midgame_eval(&BlackQueen, B6));

        assert_eq!(50, midgame_eval(&WhiteKing, B1));
        assert_eq!(-50, midgame_eval(&BlackKing, B8));
    }

    #[test]
    fn test_endgame() {
        assert_eq!(80, endgame_eval(&WhitePawn, C6));
        assert_eq!(-80, endgame_eval(&BlackPawn, C3));

        assert_eq!(-40, endgame_eval(&WhiteKnight, E1));
        assert_eq!(40, endgame_eval(&BlackKnight, E8));

        assert_eq!(25, endgame_eval(&WhiteBishop, D4));
        assert_eq!(-25, endgame_eval(&BlackBishop, D5));

        assert_eq!(10, endgame_eval(&WhiteRook, D3));
        assert_eq!(-10, endgame_eval(&BlackRook, D6));

        assert_eq!(-30, endgame_eval(&WhiteQueen, A4));
        assert_eq!(30, endgame_eval(&BlackQueen, A5));

        assert_eq!(10, endgame_eval(&WhiteKing, D7));
        assert_eq!(-10, endgame_eval(&BlackKing, D2));
    }
}

const MIDGAME: [[i32; 64]; 6] = [
    PAWN_MIDGAME, KNIGHT_MIDGAME, BISHOP_MIDGAME,
    ROOK_MIDGAME, QUEEN_MIDGAME, KING_MIDGAME
];

const ENDGAME: [[i32; 64]; 6] = [
    PAWN_ENDGAME, KNIGHT_ENDGAME, BISHOP_ENDGAME,
    ROOK_ENDGAME, QUEEN_ENDGAME, KING_ENDGAME
];

const PAWN_MIDGAME: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     60,  60,  60,  60,  60,  60,  60,  60,
      5,  25,  30,  50,  50,  30,  25,   5,
      5,  20,  30,  40,  40,  30,  20,   5,
      5,  -5,  -5,  40,  40,  -5,  -5,   5,
     10,  -5,   0, -10, -10,   0,  -5,  10,
      0,   0,   0, -20, -20,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const PAWN_ENDGAME: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
    100, 100, 100, 100, 100, 100, 100, 100,
     80,  80,  80,  80,  80,  80,  80,  80,
     60,  60,  60,  60,  60,  60,  60,  60,
     20,  20,  20,  20,  20,  20,  20,  20,
    -10, -10, -10, -10, -10, -10, -10, -10,
    -50, -50, -50, -50, -50, -50, -50, -50,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const KNIGHT_MIDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40,  10,  15,  15,  15,  15,  10, -40,
    -40,  10,  25,  25,  25,  25,  10, -40,
    -40,  10,  35,  35,  35,  35,  10, -40,
    -40,  10,  20,  25,  25,  20,  10, -40,
    -40,  10,  10,  10,  10,  10,  10, -40,
    -40, -30,   0,   0,   0,   0, -30, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
];

const KNIGHT_ENDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40,  10,  15,  15,  15,  15,  10, -40,
    -40,  20,  35,  35,  35,  35,  20, -40,
    -40,  15,  25,  25,  25,  25,  15, -40,
    -40,  15,  20,  25,  25,  20,  15, -40,
    -40,  10,  10,  10,  10,  10,  10, -40,
    -40, -30,   0,   0,   0,   0, -30, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
];

const BISHOP_MIDGAME: [i32; 64] = [
    -30, -20, -10, -10, -10, -10, -20, -30,
    -20,  10,  15,  15,  15,  15,  10, -20,
    -10,  15,  20,  25,  25,  20,  15, -10,
    -10,  15,  30,  35,  35,  30,  15, -10,
    -10,  15,  20,  25,  25,  20,  15, -10,
    -20,  10,  10,  10,  10,  10,  10, -20,
    -30,  10,   0,   0,   0,   0,  10, -30,
    -40, -40, -40, -40, -40, -40, -40, -40,
];

const BISHOP_ENDGAME: [i32; 64] = [
    -30, -20, -10, -10, -10, -10, -20, -30,
    -20,  10,  15,  15,  15,  15,  10, -20,
    -10,  20,  35,  35,  35,  35,  20, -10,
    -10,  15,  20,  25,  25,  20,  15, -10,
    -10,  15,  20,  25,  25,  20,  15, -10,
    -20,  10,  10,  10,  10,  10,  10, -20,
    -30,  10,   0,   0,   0,   0,  10, -30,
    -40, -40, -40, -40, -40, -40, -40, -40,
];

const ROOK_MIDGAME: [i32; 64] = [
      5,   7,  10,  10,  10,  10,   7,   5,
      7,  15,  25,  30,  30,  25,  15,   7,
    -30, -30, -30, -30, -30, -30, -30, -30,
    -30, -30, -30, -30, -30, -30, -30, -30,
    -30, -30, -30, -30, -30, -30, -30, -30,
    -30, -30, -30, -30, -30, -30, -30, -30,
    -10, -10,   0,   5,   5,   0, -10, -10,
     10,  10,  20,  30,  30,  20,  10,  10,
];

const ROOK_ENDGAME: [i32; 64] = [
      5,   7,  10,  10,  10,  10,   7,   5,
     25,  35,  40,  40,  40,  40,  35,  25,
     -5,   5,  20,  20,  20,  20,   5,  -5,
     -5,  -5,  10,  25,  25,  10,  -5,  -5,
     -5,  -5,  10,  25,  25,  10,  -5,  -5,
     -5,  -5,  10,  10,  10,  10,  -5,  -5,
    -10, -10,   0,   5,   5,   0, -10, -10,
      5,   5,  10,  15,  15,  10,   5,   5,
];

const QUEEN_MIDGAME: [i32; 64] = [
      5,   7,  10,  10,  10,  10,   7,   5,
      7,   7,  10,  15,  15,  10,   7,   7,
    -10,   5,  20,  35,  35,  20,   5, -10,
    -10,   5,  20,  25,  25,  20,   5, -10,
    -30,   5,  20,  25,  25,  20,   5, -30,
    -30,   5,  25,  35,  35,  25,   5, -30,
    -10, -10,  10,  10,  10,  10, -10, -10,
    -40, -40, -40,  -5,  -5, -40, -40, -40,
];

const QUEEN_ENDGAME: [i32; 64] = [
      5,   7,  10,  10,  10,  10,   7,   5,
      7,  20,  25,  35,  35,  25,  20,   7,
    -20,  10,  20,  35,  35,  20,  10, -20,
    -30,   5,  20,  25,  25,  20,   5, -30,
    -30,   5,  20,  25,  25,  20,   5, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -10, -10,   5,  10,  10,   5, -10, -10,
    -40, -40, -40,  -5,  -5, -40, -40, -40,
];

const KING_MIDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40,
    -35, -35, -35, -40, -40, -35, -35, -35,
     10,  50,  30, -40, -40,  10,  50,  10,
];

const KING_ENDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40,
    -20,   2,   5,  10,  10,   5,   2, -20,
    -20,   5,  10,  20,  20,  10,   5, -20,
    -20,  10,  20,  25,  25,  20,  10, -20,
    -20,  10,  20,  25,  25,  20,  10, -20,
    -20,   5,  10,  20,  20,  10,   5, -20,
    -40,   2,   5,  10,  10,   5,   2, -40,
    -10,   0,   0, -20, -20,   0,   0, -10,
];
