use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::pieces::Piece;

/// API method for retrieving the evaluation for a piece at a given location
/// in the midgame.
pub fn midgame(piece: Piece, location: Square) -> i32 {
    let (table_index, parity) = compute_index_and_parity(piece, location);
    parity * MIDGAME[(piece as usize) % 6][table_index]
}

/// API method for retrieving the evaluation for a piece at a given location
/// in the endgame.
pub fn endgame(piece: Piece, location: Square) -> i32 {
    let (table_index, parity) = compute_index_and_parity(piece, location);
    parity * ENDGAME[(piece as usize) % 6][table_index]
}

/// Computes the table index alongside the parity multiplier according to the
/// piece side.
fn compute_index_and_parity(piece: Piece, location: Square) -> (usize, i32) {
    match piece.side() {
        Side::White => (63 - (location as usize), 1),
        Side::Black => (63 - (location.reflect() as usize), -1),
    }
}

#[cfg(test)]
mod test {
    use crate::base::Reflectable;
    use crate::base::square::Square::*;
    use crate::pieces::Piece;

    use super::{endgame, midgame};

    #[test]
    fn test_reflect() {
        assert_eq!(A8, A1.reflect());
        assert_eq!(H1, H8.reflect());
        assert_eq!(D3, D6.reflect());
        assert_eq!(D5, D4.reflect());
    }

    #[test]
    fn test_midgame() {
        assert_eq!(30, midgame(Piece::WP, C6));
        assert_eq!(-30, midgame(Piece::BP, C3));

        assert_eq!(10, midgame(Piece::WN, D3));
        assert_eq!(-10, midgame(Piece::BN, D6));

        assert_eq!(25, midgame(Piece::WB, D4));
        assert_eq!(-25, midgame(Piece::BB, D5));

        assert_eq!(5, midgame(Piece::WR, D2));
        assert_eq!(-5, midgame(Piece::BR, D7));

        assert_eq!(5, midgame(Piece::WQ, B3));
        assert_eq!(-5, midgame(Piece::BQ, B6));

        assert_eq!(50, midgame(Piece::WK, B1));
        assert_eq!(-50, midgame(Piece::BK, B8));
    }

    #[test]
    fn test_endgame() {
        assert_eq!(80, endgame(Piece::WP, C6));
        assert_eq!(-80, endgame(Piece::BP, C3));

        assert_eq!(-40, endgame(Piece::WN, E1));
        assert_eq!(40, endgame(Piece::BN, E8));

        assert_eq!(25, endgame(Piece::WB, D4));
        assert_eq!(-25, endgame(Piece::BB, D5));

        assert_eq!(10, endgame(Piece::WR, D3));
        assert_eq!(-10, endgame(Piece::BR, D6));

        assert_eq!(-30, endgame(Piece::WQ, A4));
        assert_eq!(30, endgame(Piece::BQ, A5));

        assert_eq!(10, endgame(Piece::WK, D7));
        assert_eq!(-10, endgame(Piece::BK, D2));
    }
}

const MIDGAME: [[i32; 64]; 6] = [
    PAWN_MIDGAME,
    KNIGHT_MIDGAME,
    BISHOP_MIDGAME,
    ROOK_MIDGAME,
    QUEEN_MIDGAME,
    KING_MIDGAME,
];

const ENDGAME: [[i32; 64]; 6] = [
    PAWN_ENDGAME,
    KNIGHT_ENDGAME,
    BISHOP_ENDGAME,
    ROOK_ENDGAME,
    QUEEN_ENDGAME,
    KING_ENDGAME,
];

const PAWN_MIDGAME: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 60, 60, 60, 60, 60, 60, 60, 60, 5, 25, 30, 50, 50, 30, 25, 5, 5, 20,
    30, 40, 40, 30, 20, 5, 5, -5, -5, 40, 40, -5, -5, 5, 10, -5, 0, -10, -10, 0, -5, 10, 0, 0, 0,
    -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PAWN_ENDGAME: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 100, 100, 100, 100, 100, 100, 100, 100, 80, 80, 80, 80, 80, 80, 80, 80,
    60, 60, 60, 60, 60, 60, 60, 60, 20, 20, 20, 20, 20, 20, 20, 20, -10, -10, -10, -10, -10, -10,
    -10, -10, -50, -50, -50, -50, -50, -50, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0,
];

const KNIGHT_MIDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40, -40, 10, 15, 15, 15, 15, 10, -40, -40, 10, 25, 25, 25,
    25, 10, -40, -40, 10, 35, 35, 35, 35, 10, -40, -40, 10, 20, 25, 25, 20, 10, -40, -40, 10, 10,
    10, 10, 10, 10, -40, -40, -30, 0, 0, 0, 0, -30, -40, -40, -40, -40, -40, -40, -40, -40, -40,
];

const KNIGHT_ENDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40, -40, 10, 15, 15, 15, 15, 10, -40, -40, 20, 35, 35, 35,
    35, 20, -40, -40, 15, 25, 25, 25, 25, 15, -40, -40, 15, 20, 25, 25, 20, 15, -40, -40, 10, 10,
    10, 10, 10, 10, -40, -40, -30, 0, 0, 0, 0, -30, -40, -40, -40, -40, -40, -40, -40, -40, -40,
];

const BISHOP_MIDGAME: [i32; 64] = [
    -30, -20, -10, -10, -10, -10, -20, -30, -20, 10, 15, 15, 15, 15, 10, -20, -10, 15, 20, 25, 25,
    20, 15, -10, -10, 15, 30, 35, 35, 30, 15, -10, -10, 15, 20, 25, 25, 20, 15, -10, -20, 10, 10,
    10, 10, 10, 10, -20, -30, 10, 0, 0, 0, 0, 10, -30, -40, -40, -40, -40, -40, -40, -40, -40,
];

const BISHOP_ENDGAME: [i32; 64] = [
    -30, -20, -10, -10, -10, -10, -20, -30, -20, 10, 15, 15, 15, 15, 10, -20, -10, 20, 35, 35, 35,
    35, 20, -10, -10, 15, 20, 25, 25, 20, 15, -10, -10, 15, 20, 25, 25, 20, 15, -10, -20, 10, 10,
    10, 10, 10, 10, -20, -30, 10, 0, 0, 0, 0, 10, -30, -40, -40, -40, -40, -40, -40, -40, -40,
];

const ROOK_MIDGAME: [i32; 64] = [
    5, 7, 10, 10, 10, 10, 7, 5, 7, 15, 25, 30, 30, 25, 15, 7, -30, -30, -30, -30, -30, -30, -30,
    -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
    -30, -30, -30, -30, -30, -30, -10, -10, 0, 5, 5, 0, -10, -10, 10, 10, 20, 30, 30, 20, 10, 10,
];

const ROOK_ENDGAME: [i32; 64] = [
    5, 7, 10, 10, 10, 10, 7, 5, 25, 35, 40, 40, 40, 40, 35, 25, -5, 5, 20, 20, 20, 20, 5, -5, -5,
    -5, 10, 25, 25, 10, -5, -5, -5, -5, 10, 25, 25, 10, -5, -5, -5, -5, 10, 10, 10, 10, -5, -5,
    -10, -10, 0, 5, 5, 0, -10, -10, 5, 5, 10, 15, 15, 10, 5, 5,
];

const QUEEN_MIDGAME: [i32; 64] = [
    5, 7, 10, 10, 10, 10, 7, 5, 7, 7, 10, 15, 15, 10, 7, 7, -10, 5, 20, 35, 35, 20, 5, -10, -10, 5,
    20, 25, 25, 20, 5, -10, -30, 5, 20, 25, 25, 20, 5, -30, -30, 5, 25, 35, 35, 25, 5, -30, -10,
    -10, 10, 10, 10, 10, -10, -10, -40, -40, -40, -5, -5, -40, -40, -40,
];

const QUEEN_ENDGAME: [i32; 64] = [
    5, 7, 10, 10, 10, 10, 7, 5, 7, 20, 25, 35, 35, 25, 20, 7, -20, 10, 20, 35, 35, 20, 10, -20,
    -30, 5, 20, 25, 25, 20, 5, -30, -30, 5, 20, 25, 25, 20, 5, -30, -30, 5, 10, 15, 15, 10, 5, -30,
    -10, -10, 5, 10, 10, 5, -10, -10, -40, -40, -40, -5, -5, -40, -40, -40,
];

const KING_MIDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
    -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -35, -35, -35, -40, -40, -35, -35, -35, 10,
    50, 30, -40, -40, 10, 50, 10,
];

const KING_ENDGAME: [i32; 64] = [
    -40, -40, -40, -40, -40, -40, -40, -40, -20, 2, 5, 10, 10, 5, 2, -20, -20, 5, 10, 20, 20, 10,
    5, -20, -20, 10, 20, 25, 25, 20, 10, -20, -20, 10, 20, 25, 25, 20, 10, -20, -20, 5, 10, 20, 20,
    10, 5, -20, -40, 2, 5, 10, 10, 5, 2, -40, -10, 0, 0, -20, -20, 0, 0, -10,
];
