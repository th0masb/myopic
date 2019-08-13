use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::pieces::Piece;

/// API method for retrieving the evaluation for a piece at a given location
/// in the midgame.
pub fn midgame(piece: Piece, location: Square) -> i32 {
    if piece.is_pawn() {
        let (table_index, parity) = compute_pawn_index_and_parity(piece, location);
        parity * PAWN[table_index].0
    } else {
        let (table_index, parity) = compute_non_pawn_index_and_parity(piece, location);
        parity * NON_PAWN_TABLES[((piece as usize) % 6) - 1][table_index].0
    }
}

/// API method for retrieving the evaluation for a piece at a given location
/// in the endgame.
pub fn endgame(piece: Piece, location: Square) -> i32 {
    if piece.is_pawn() {
        let (table_index, parity) = compute_pawn_index_and_parity(piece, location);
        parity * PAWN[table_index].1
    } else {
        let (table_index, parity) = compute_non_pawn_index_and_parity(piece, location);
        parity * NON_PAWN_TABLES[((piece as usize) % 6) - 1][table_index].1
    }
}

fn compute_pawn_index_and_parity(pawn: Piece, location: Square) -> (usize, i32) {
    match pawn.side() {
        Side::White => (location as usize, 1),
        Side::Black => (location.reflect() as usize, -1),
    }
}

fn compute_non_pawn_index_and_parity(piece: Piece, location: Square) -> (usize, i32) {
    match piece.side() {
        Side::White => (compute_table_index_non_pawn(location), 1),
        Side::Black => (compute_table_index_non_pawn(location.reflect()), -1),
    }
}

fn compute_table_index_non_pawn(location: Square) -> usize {
    let file_index = location.file_index();
    let column_index = if file_index < 4 {
        file_index
    } else {
        7 - file_index
    };
    location.rank_index() * 4 + column_index
}

/// Tables lifted from stockfish here: https://github.com/official-stockfish/Stockfish/blob/master/src/psqt.cpp
/// They are (mid, end) values for white side on files H - E
// Knight
const KNIGHT: [(i32, i32); 32] = [
    // Rank 1
    (-169, -105),
    (-96, -74),
    (-80, -46),
    (-79, -18),
    // Rank 2
    (-79, -70),
    (-39, -56),
    (-24, -15),
    (-9, 6),
    (-64, -38),
    (-20, -33),
    (4, -5),
    (19, 27),
    (-28, -36),
    (5, 0),
    (41, 13),
    (47, 34),
    (-29, -41),
    (13, -20),
    (42, 4),
    (52, 35),
    (-11, -51),
    (28, -38),
    (63, -17),
    (55, 19),
    (-67, -64),
    (-21, -45),
    (6, -37),
    (37, 16),
    (-200, -98),
    (-80, -89),
    (-53, -53),
    (-32, -16),
];

const BISHOP: [(i32, i32); 32] = [
    // Rank 1
    (-44, -63),
    (-4, -30),
    (-11, -35),
    (-28, -8),
    (-18, -38),
    (7, -13),
    (14, -14),
    (3, 0),
    (-8, -18),
    (24, 0),
    (-3, -7),
    (15, 13),
    (1, -26),
    (8, -3),
    (26, 1),
    (37, 16),
    (-7, -24),
    (30, -6),
    (23, -10),
    (28, 17),
    (-17, -26),
    (4, 2),
    (-1, 1),
    (8, 16),
    (-21, -34),
    (-19, -18),
    (10, -7),
    (-6, 9),
    (-48, -51),
    (-3, -40),
    (-12, -39),
    (-25, -20),
];

const ROOK: [(i32, i32); 32] = [
    // Rank 1
    (-24, -2),
    (-13, -6),
    (-7, -3),
    (2, -2),
    (-18, -10),
    (-10, -7),
    (-5, 1),
    (9, 0),
    (-21, 10),
    (-7, -4),
    (3, 2),
    (-1, -2),
    (-13, -5),
    (-5, 2),
    (-4, -8),
    (-6, 8),
    (-24, -8),
    (-12, 5),
    (-1, 4),
    (6, -9),
    (-24, 3),
    (-4, -2),
    (4, -10),
    (10, 7),
    (-8, 1),
    (6, 2),
    (10, 17),
    (12, -8),
    (-22, 12),
    (-24, -6),
    (-6, 13),
    (4, 7),
];

const QUEEN: [(i32, i32); 32] = [
    // Rank 1
    (3, -69),
    (-5, -57),
    (-5, -47),
    (4, -26),
    (-3, -55),
    (5, -31),
    (8, -22),
    (12, -4),
    (-3, -39),
    (6, -18),
    (13, -9),
    (7, 3),
    (4, -23),
    (5, -3),
    (9, 13),
    (8, 24),
    (0, -29),
    (14, -6),
    (12, 9),
    (5, 21),
    (-4, -38),
    (10, -18),
    (6, -12),
    (8, 1),
    (-5, -50),
    (6, -27),
    (10, -24),
    (8, -8),
    (-2, -75),
    (-2, -52),
    (1, -43),
    (-2, -36),
];

const KING: [(i32, i32); 32] = [
    // Rank 1
    (272, 0),
    (325, 41),
    (273, 80),
    (190, 93),
    (277, 57),
    (305, 98),
    (241, 138),
    (183, 131),
    (198, 86),
    (253, 138),
    (168, 165),
    (120, 173),
    (169, 103),
    (191, 152),
    (136, 168),
    (108, 169),
    (145, 98),
    (176, 166),
    (112, 197),
    (69, 194),
    (122, 87),
    (159, 164),
    (85, 174),
    (36, 189),
    (87, 40),
    (120, 99),
    (64, 128),
    (25, 141),
    (64, 5),
    (87, 60),
    (49, 75),
    (0, 75),
];

const NON_PAWN_TABLES: [[(i32, i32); 32]; 5] = [KNIGHT, BISHOP, ROOK, QUEEN, KING];

const PAWN: [(i32, i32); 64] = [
    // Pawn (asymmetric distribution) (note H file is on the left here
    // Rank 1
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    // Rank 2
    (-5, -19),
    (7, -5),
    (19, 7),
    (16, 14),
    (19, 0),
    (10, 10),
    (3, -6),
    (3, -10),
    (-22, -4),
    (5, -6),
    (22, 3),
    (32, 4),
    (15, 4),
    (11, -10),
    (-15, -10),
    (-9, -10),
    (-12, -9),
    (4, -10),
    (17, -12),
    (40, -13),
    (20, -4),
    (6, -8),
    (-23, -2),
    (-8, 6),
    (5, 8),
    (-13, 13),
    (-2, -6),
    (11, -12),
    (1, -12),
    (-13, 3),
    (0, 4),
    (13, 9),
    (-18, 13),
    (-15, 6),
    (-5, 7),
    (-8, 30),
    (22, 28),
    (-7, 21),
    (-12, 20),
    (-5, 28),
    (-8, 7),
    (10, 4),
    (-16, 19),
    (5, 25),
    (-13, 21),
    (-3, 12),
    (7, -11),
    (-7, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
];

#[cfg(test)]
mod test {
    use crate::base::square::Square::*;
    use crate::base::Reflectable;
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
        assert_eq!(-7, midgame(Piece::WP, C6));
        assert_eq!(7, midgame(Piece::BP, C3));

        assert_eq!(19, midgame(Piece::WN, D3));
        assert_eq!(-19, midgame(Piece::BN, D6));

        assert_eq!(26, midgame(Piece::WB, C4));
        assert_eq!(-26, midgame(Piece::BB, C5));

        assert_eq!(-5, midgame(Piece::WR, F2));
        assert_eq!(5, midgame(Piece::BR, F7));

        assert_eq!(6, midgame(Piece::WQ, B3));
        assert_eq!(-6, midgame(Piece::BQ, B6));

        assert_eq!(325, midgame(Piece::WK, B1));
        assert_eq!(-325, midgame(Piece::BK, B8));
    }

    #[test]
    fn test_endgame() {
        assert_eq!(21, endgame(Piece::WP, C6));
        assert_eq!(-21, endgame(Piece::BP, C3));

        assert_eq!(-18, endgame(Piece::WN, E1));
        assert_eq!(18, endgame(Piece::BN, E8));

        assert_eq!(16, endgame(Piece::WB, D4));
        assert_eq!(-16, endgame(Piece::BB, D5));

        assert_eq!(-2, endgame(Piece::WR, D3));
        assert_eq!(2, endgame(Piece::BR, D6));

        assert_eq!(-23, endgame(Piece::WQ, A4));
        assert_eq!(23, endgame(Piece::BQ, A5));

        assert_eq!(141, endgame(Piece::WK, D7));
        assert_eq!(-141, endgame(Piece::BK, D2));
    }
}
