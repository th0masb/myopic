use myopic_board::{Piece, Piece::*, Reflectable, Side, Square};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq)]
pub struct PositionTables {
    // Asymmetric, split across 2 arrays
    pub pawn_1_4: [(i32, i32); 32],
    pub pawn_5_8: [(i32, i32); 32],
    // Symmetric
    pub knight: [(i32, i32); 32],
    pub bishop: [(i32, i32); 32],
    pub rook: [(i32, i32); 32],
    pub queen: [(i32, i32); 32],
    pub king: [(i32, i32); 32],
}

fn parity(piece: Piece) -> i32 {
    match piece.side() {
        Side::White => 1,
        Side::Black => -1,
    }
}

fn table_index(piece: Piece, adjusted_location: Square) -> usize {
    if piece.is_pawn() {
        (adjusted_location as usize) % 32
    } else {
        let file_index = adjusted_location.file_index();
        let column_index = if file_index < 4 { file_index } else { 7 - file_index };
        adjusted_location.rank_index() * 4 + column_index
    }
}

fn adjust_location(side: Side, location: Square) -> Square {
    match side {
        Side::White => location,
        Side::Black => location.reflect(),
    }
}

impl Default for PositionTables {
    fn default() -> Self {
        PositionTables {
            pawn_1_4: PAWN_1_4,
            pawn_5_8: PAWN_5_8,
            knight: KNIGHT,
            bishop: BISHOP,
            rook: ROOK,
            queen: QUEEN,
            king: KING,
        }
    }
}

impl PositionTables {
    /// API method for retrieving the evaluation for a piece at a given location
    /// in the midgame.
    pub fn midgame(&self, piece: Piece, location: Square) -> i32 {
        self.access(piece, location, |(mid, _end)| mid)
    }

    /// API method for retrieving the evaluation for a piece at a given location
    /// in the midgame.
    pub fn endgame(&self, piece: Piece, location: Square) -> i32 {
        self.access(piece, location, |(_mid, end)| end)
    }

    fn access(&self, piece: Piece, location: Square, f: fn((i32, i32)) -> i32) -> i32 {
        let adjusted_location = adjust_location(piece.side(), location);
        let table_index = table_index(piece, adjusted_location);
        parity(piece)
            * match piece {
                WN | BN => f(self.knight[table_index]),
                WB | BB => f(self.bishop[table_index]),
                WR | BR => f(self.rook[table_index]),
                WQ | BQ => f(self.queen[table_index]),
                WK | BK => f(self.king[table_index]),
                WP | BP => {
                    if (adjusted_location as usize) < 32 {
                        f(self.pawn_1_4[table_index])
                    } else {
                        f(self.pawn_5_8[table_index])
                    }
                }
            }
    }
}

/// Tables lifted from stockfish here: https://github.com/official-stockfish/Stockfish/blob/master/src/psqt.cpp
/// They are (mid, end) values for white side on files H - E
// Knight
#[rustfmt::skip]
const KNIGHT: [(i32, i32); 32] = [
    // Rank 1
    (-169, -105), (-96, -74), (-80, -46), (-79, -18),
    ( -79,  -70), (-39, -56), (-24, -15), ( -9,   6),
    ( -64,  -38), (-20, -33), (  4,  -5), ( 19,  27),
    ( -28,  -36), (  5,   0), ( 41,  13), ( 47,  34),
    ( -29,  -41), ( 13, -20), ( 42,   4), ( 52,  35),
    ( -11,  -51), ( 28, -38), ( 63, -17), ( 55,  19),
    ( -67,  -64), (-21, -45), (  6, -37), ( 37,  16),
    (-200,  -98), (-80, -89), (-53, -53), (-32, -16),
    // Rank 8
];

#[rustfmt::skip]
const BISHOP: [(i32, i32); 32] = [
    // Rank 1
    (-44, -63), ( -4, -30), (-11, -35), (-28,  -8),
    (-18, -38), (  7, -13), ( 14, -14), (  3,   0),
    ( -8, -18), ( 24,   0), ( -3,  -7), ( 15,  13),
    (  1, -26), (  8,  -3), ( 26,   1), ( 37,  16),
    ( -7, -24), ( 30,  -6), ( 23, -10), ( 28,  17),
    (-17, -26), (  4,   2), ( -1,   1), (  8,  16),
    (-21, -34), (-19, -18), ( 10,  -7), ( -6,   9),
    (-48, -51), ( -3, -40), (-12, -39), (-25, -20),
];

#[rustfmt::skip]
const ROOK: [(i32, i32); 32] = [
    // Rank 1
    (-24,  -2), (-13, -6), (-7,  -3), ( 2, -2),
    (-18, -10), (-10, -7), (-5,   1), ( 9,  0),
    (-21,  10), ( -7, -4), ( 3,   2), (-1, -2),
    (-13,  -5), ( -5,  2), (-4,  -8), (-6,  8),
    (-24,  -8), (-12,  5), (-1,   4), (6,  -9),
    (-24,   3), ( -4, -2), ( 4, -10), (10,  7),
    ( -8,   1), (  6,  2), (10,  17), (12, -8),
    (-22,  12), (-24, -6), (-6,  13), ( 4,  7),
];

#[rustfmt::skip]
const QUEEN: [(i32, i32); 32] = [
    // Rank 1
    ( 3, -69), (-5, -57), (-5, -47), ( 4, -26),
    (-3, -55), ( 5, -31), ( 8, -22), (12,  -4),
    (-3, -39), ( 6, -18), (13,  -9), ( 7,   3),
    ( 4, -23), ( 5,  -3), ( 9,  13), ( 8,  24),
    ( 0, -29), (14,  -6), (12,   9), ( 5,  21),
    (-4, -38), (10, -18), ( 6, -12), ( 8,   1),
    (-5, -50), ( 6, -27), (10, -24), ( 8,  -8),
    (-2, -75), (-2, -52), ( 1, -43), (-2, -36),
];

#[rustfmt::skip]
const KING: [(i32, i32); 32] = [
    // Rank 1
    (272,   0), (325,  41), (273,  80), (190,  93),
    (277,  57), (305,  98), (241, 138), (183, 131),
    (198,  86), (253, 138), (168, 165), (120, 173),
    (169, 103), (191, 152), (136, 168), (108, 169),
    (145,  98), (176, 166), (112, 197), ( 69, 194),
    (122,  87), (159, 164), ( 85, 174), ( 36, 189),
    ( 87,  40), (120,  99), ( 64, 128), ( 25, 141),
    ( 64,   5), ( 87,  60), ( 49,  75), (  0,  75),
];

#[rustfmt::skip]
const PAWN_1_4: [(i32, i32); 32] = [
    // Pawn (asymmetric distribution) (note H file is on the left here
    // Rank 1
    (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
    ( -5, -19), (  7,  -5), ( 19,   7), (-20, -20), (-20, -20), ( 10,  10), (  3,  -6), ( 3, -10),
    (-22,  -4), (  5,  -6), ( 22,   3), ( 32,   4), ( 15,   4), ( 11, -10), (-15, -10), (-9, -10),
    (-12,  -9), (  4, -10), ( 17, -12), ( 40, -13), ( 20,  -4), (  6,  -8), (-23,  -2), (-8,   6),
];

#[rustfmt::skip]
const PAWN_5_8: [(i32, i32); 32] = [
    // Pawn (asymmetric distribution) (note H file is on the left here
    // Rank 5
    (  5,   8), (-13,  13), ( -2,  -6), (11, -12), (  1, -12), (-13,   3), (  0,   4), (13,   9),
    (-18,  13), (-15,   6), ( -5,   7), (-8,  30), ( 22,  28), ( -7,  21), (-12,  20), (-5,  28),
    ( -8,   7), ( 10,   4), (-16,  19), ( 5,  25), (-13,  21), ( -3,  12), (  7, -11), (-7,   0),
    (  0,   0), (  0,   0), (  0,   0), ( 0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
];

#[cfg(test)]
mod test {
    use crate::tables::PositionTables;

    use myopic_board::{Piece, Reflectable, Square, Square::*};

    // Fully connected pawn table
    #[rustfmt::skip]
    const PAWN: [(i32, i32); 64] = [
        // Pawn (asymmetric distribution) (note H file is on the left here
        // Rank 1
        (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
        ( -5, -19), (  7,  -5), ( 19,   7), (-20, -20), (-20, -20), ( 10,  10), (  3,  -6), ( 3, -10),
        (-22,  -4), (  5,  -6), ( 22,   3), ( 32,   4), ( 15,   4), ( 11, -10), (-15, -10), (-9, -10),
        (-12,  -9), (  4, -10), ( 17, -12), ( 40, -13), ( 20,  -4), (  6,  -8), (-23,  -2), (-8,   6),
        (  5,   8), (-13,  13), ( -2,  -6), ( 11, -12), (  1, -12), (-13,   3), (  0,   4), (13,   9),
        (-18,  13), (-15,   6), ( -5,   7), ( -8,  30), ( 22,  28), ( -7,  21), (-12,  20), (-5,  28),
        ( -8,   7), ( 10,   4), (-16,  19), (  5,  25), (-13,  21), ( -3,  12), (  7, -11), (-7,   0),
        (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
    ];

    #[test]
    fn test_pawn_table_conjunction() {
        let tables = PositionTables::default();
        for i in 0..64 {
            assert_eq!(PAWN[i as usize].0, tables.midgame(Piece::WP, Square::from_index(i)))
        }
    }

    #[test]
    fn test_reflect() {
        assert_eq!(A8, A1.reflect());
        assert_eq!(H1, H8.reflect());
        assert_eq!(D3, D6.reflect());
        assert_eq!(D5, D4.reflect());
    }

    #[test]
    fn test_midgame() {
        let tables = PositionTables::default();
        assert_eq!(-7, tables.midgame(Piece::WP, C6));
        assert_eq!(7, tables.midgame(Piece::BP, C3));

        assert_eq!(19, tables.midgame(Piece::WN, D3));
        assert_eq!(-19, tables.midgame(Piece::BN, D6));

        assert_eq!(26, tables.midgame(Piece::WB, C4));
        assert_eq!(-26, tables.midgame(Piece::BB, C5));

        assert_eq!(-5, tables.midgame(Piece::WR, F2));
        assert_eq!(5, tables.midgame(Piece::BR, F7));

        assert_eq!(6, tables.midgame(Piece::WQ, B3));
        assert_eq!(-6, tables.midgame(Piece::BQ, B6));

        assert_eq!(325, tables.midgame(Piece::WK, B1));
        assert_eq!(-325, tables.midgame(Piece::BK, B8));
    }

    #[test]
    fn test_endgame() {
        let tables = PositionTables::default();
        assert_eq!(21, tables.endgame(Piece::WP, C6));
        assert_eq!(-21, tables.endgame(Piece::BP, C3));

        assert_eq!(-18, tables.endgame(Piece::WN, E1));
        assert_eq!(18, tables.endgame(Piece::BN, E8));

        assert_eq!(16, tables.endgame(Piece::WB, D4));
        assert_eq!(-16, tables.endgame(Piece::BB, D5));

        assert_eq!(-2, tables.endgame(Piece::WR, D3));
        assert_eq!(2, tables.endgame(Piece::BR, D6));

        assert_eq!(-23, tables.endgame(Piece::WQ, A4));
        assert_eq!(23, tables.endgame(Piece::BQ, A5));

        assert_eq!(141, tables.endgame(Piece::WK, D7));
        assert_eq!(-141, tables.endgame(Piece::BK, D2));
    }
}
