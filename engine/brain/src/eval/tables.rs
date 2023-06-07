use serde_derive::{Deserialize, Serialize};

use crate::eval::{EvalFacet, Evaluation};
use crate::{Class, Corner, Line};
use myopic_board::{Board, Move, Piece, Reflectable, Side, Square};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PieceSquareTablesFacet {
    tables: PositionTables,
    mid_eval: i32,
    end_eval: i32,
}

impl Default for PieceSquareTablesFacet {
    fn default() -> Self {
        PieceSquareTablesFacet { tables: PositionTables::default(), mid_eval: 0, end_eval: 0 }
    }
}

impl<'a> From<&'a Board> for PieceSquareTablesFacet {
    fn from(value: &Board) -> Self {
        let mut facet = PieceSquareTablesFacet::default();
        facet.mid_eval = facet.compute_midgame_eval(value);
        facet.end_eval = facet.compute_endgame_eval(value);
        facet
    }
}

type UpdateFn = fn(&mut PieceSquareTablesFacet, Piece, Square) -> ();

impl PieceSquareTablesFacet {
    pub fn compute_midgame_eval(&self, board: &Board) -> i32 {
        Square::iter()
            .flat_map(|square| board.piece(square).map(|p| (p, square)))
            .map(|(piece, square)| self.tables.midgame(piece, square))
            .sum()
    }

    pub fn compute_endgame_eval(&self, board: &Board) -> i32 {
        Square::iter()
            .flat_map(|square| board.piece(square).map(|p| (p, square)))
            .map(|(piece, square)| self.tables.endgame(piece, square))
            .sum()
    }

    fn add(&mut self, piece: Piece, square: Square) {
        self.mid_eval += self.tables.midgame(piece, square);
        self.end_eval += self.tables.endgame(piece, square);
    }

    fn remove(&mut self, piece: Piece, square: Square) {
        self.mid_eval -= self.tables.midgame(piece, square);
        self.end_eval -= self.tables.endgame(piece, square);
    }

    fn make_impl(&mut self, mv: &Move, add: UpdateFn, remove: UpdateFn) {
        match mv {
            &Move::Castle { corner: Corner(side, flank) } => {
                let Line(rook_start, rook_end) = Line::rook_castling(Corner(side, flank));
                remove(self, Piece(side, Class::R), rook_start);
                add(self, Piece(side, Class::R), rook_end);
                let Line(king_start, king_end) = Line::king_castling(Corner(side, flank));
                remove(self, Piece(side, Class::K), king_start);
                add(self, Piece(side, Class::K), king_end);
            }
            &Move::Standard { moving, from, dest, capture } => {
                remove(self, moving, from);
                add(self, moving, dest);
                if let Some(piece) = capture {
                    remove(self, piece, from);
                }
            }
            &Move::Enpassant { side, from, dest, capture } => {
                remove(self, Piece(side, Class::P), from);
                add(self, Piece(side, Class::P), dest);
                remove(self, Piece(side.reflect(), Class::P), capture);
            }
            &Move::Promotion { from, dest, promoted: Piece(side, class), capture } => {
                remove(self, Piece(side, Class::P), from);
                add(self, Piece(side, class), dest);
                if let Some(captured) = capture {
                    remove(self, captured, from);
                }
            }
        }
    }
}

impl EvalFacet for PieceSquareTablesFacet {
    fn static_eval(&self, _: &Board) -> Evaluation {
        Evaluation::Phased { mid: self.mid_eval, end: self.end_eval }
    }

    fn make(&mut self, mv: &Move, _: &Board) {
        self.make_impl(mv, PieceSquareTablesFacet::add, PieceSquareTablesFacet::remove);
    }

    fn unmake(&mut self, mv: &Move) {
        self.make_impl(mv, PieceSquareTablesFacet::remove, PieceSquareTablesFacet::add);
    }
}

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
    match piece.0 {
        Side::W => 1,
        Side::B => -1,
    }
}

fn table_index(piece: Piece, adjusted_location: Square) -> usize {
    if piece.1 == Class::P {
        (adjusted_location as usize) % 32
    } else {
        let file_index = adjusted_location.file_index();
        let column_index = if file_index < 4 { file_index } else { 7 - file_index };
        adjusted_location.rank_index() * 4 + column_index
    }
}

fn adjust_location(side: Side, location: Square) -> Square {
    match side {
        Side::W => location,
        Side::B => location.reflect(),
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
        let adjusted_location = adjust_location(piece.0, location);
        let table_index = table_index(piece, adjusted_location);
        parity(piece)
            * match piece.1 {
                Class::N => f(self.knight[table_index]),
                Class::B => f(self.bishop[table_index]),
                Class::R => f(self.rook[table_index]),
                Class::Q => f(self.queen[table_index]),
                Class::K => f(self.king[table_index]),
                Class::P => {
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
    (-79, -70), (-39, -56), (-24, -15), (-9, 6),
    (-64, -38), (-20, -33), (4, -5), (19, 27),
    (-28, -36), (5, 0), (41, 13), (47, 34),
    (-29, -41), (13, -20), (42, 4), (52, 35),
    (-11, -51), (28, -38), (63, -17), (55, 19),
    (-67, -64), (-21, -45), (6, -37), (37, 16),
    (-200, -98), (-80, -89), (-53, -53), (-32, -16),
    // Rank 8
];

#[rustfmt::skip]
const BISHOP: [(i32, i32); 32] = [
    // Rank 1
    (-44, -63), (-4, -30), (-11, -35), (-28, -8),
    (-18, -38), (7, -13), (14, -14), (3, 0),
    (-8, -18), (24, 0), (-3, -7), (15, 13),
    (1, -26), (8, -3), (26, 1), (37, 16),
    (-7, -24), (30, -6), (23, -10), (28, 17),
    (-17, -26), (4, 2), (-1, 1), (8, 16),
    (-21, -34), (-19, -18), (10, -7), (-6, 9),
    (-48, -51), (-3, -40), (-12, -39), (-25, -20),
];

#[rustfmt::skip]
const ROOK: [(i32, i32); 32] = [
    // Rank 1
    (-24, -2), (-13, -6), (-7, -3), (2, -2),
    (-18, -10), (-10, -7), (-5, 1), (9, 0),
    (-21, 10), (-7, -4), (3, 2), (-1, -2),
    (-13, -5), (-5, 2), (-4, -8), (-6, 8),
    (-24, -8), (-12, 5), (-1, 4), (6, -9),
    (-24, 3), (-4, -2), (4, -10), (10, 7),
    (-8, 1), (6, 2), (10, 17), (12, -8),
    (-22, 12), (-24, -6), (-6, 13), (4, 7),
];

#[rustfmt::skip]
const QUEEN: [(i32, i32); 32] = [
    // Rank 1
    (3, -69), (-5, -57), (-5, -47), (4, -26),
    (-3, -55), (5, -31), (8, -22), (12, -4),
    (-3, -39), (6, -18), (13, -9), (7, 3),
    (4, -23), (5, -3), (9, 13), (8, 24),
    (0, -29), (14, -6), (12, 9), (5, 21),
    (-4, -38), (10, -18), (6, -12), (8, 1),
    (-5, -50), (6, -27), (10, -24), (8, -8),
    (-2, -75), (-2, -52), (1, -43), (-2, -36),
];

#[rustfmt::skip]
const KING: [(i32, i32); 32] = [
    // Rank 1
    (272, 0), (325, 41), (273, 80), (190, 93),
    (277, 57), (305, 98), (241, 138), (183, 131),
    (198, 86), (253, 138), (168, 165), (120, 173),
    (169, 103), (191, 152), (136, 168), (108, 169),
    (145, 98), (176, 166), (112, 197), (69, 194),
    (122, 87), (159, 164), (85, 174), (36, 189),
    (87, 40), (120, 99), (64, 128), (25, 141),
    (64, 5), (87, 60), (49, 75), (0, 75),
];

#[rustfmt::skip]
const PAWN_1_4: [(i32, i32); 32] = [
    // Pawn (asymmetric distribution) (note H file is on the left here
    // Rank 1
    (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
    (-5, -19), (7, -5), (19, 7), (-20, -20), (-20, -20), (10, 10), (3, -6), (3, -10),
    (-22, -4), (5, -6), (22, 3), (32, 4), (15, 4), (11, -10), (-15, -10), (-9, -10),
    (-12, -9), (4, -10), (17, -12), (40, -13), (20, -4), (6, -8), (-23, -2), (-8, 6),
];

#[rustfmt::skip]
const PAWN_5_8: [(i32, i32); 32] = [
    // Pawn (asymmetric distribution) (note H file is on the left here
    // Rank 5
    (5, 8), (-13, 13), (-2, -6), (11, -12), (1, -12), (-13, 3), (0, 4), (13, 9),
    (-18, 13), (-15, 6), (-5, 7), (-8, 30), (22, 28), (-7, 21), (-12, 20), (-5, 28),
    (-8, 7), (10, 4), (-16, 19), (5, 25), (-13, 21), (-3, 12), (7, -11), (-7, 0),
    (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
];

#[cfg(test)]
mod test {
    use myopic_board::{Piece, Reflectable, Square::*};

    use crate::{Class, PositionTables, Side};

    // Fully connected pawn table
    #[rustfmt::skip]
    const PAWN: [(i32, i32); 64] = [
        // Pawn (asymmetric distribution) (note H file is on the left here
        // Rank 1
        (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
        (-5, -19), (7, -5), (19, 7), (-20, -20), (-20, -20), (10, 10), (3, -6), (3, -10),
        (-22, -4), (5, -6), (22, 3), (32, 4), (15, 4), (11, -10), (-15, -10), (-9, -10),
        (-12, -9), (4, -10), (17, -12), (40, -13), (20, -4), (6, -8), (-23, -2), (-8, 6),
        (5, 8), (-13, 13), (-2, -6), (11, -12), (1, -12), (-13, 3), (0, 4), (13, 9),
        (-18, 13), (-15, 6), (-5, 7), (-8, 30), (22, 28), (-7, 21), (-12, 20), (-5, 28),
        (-8, 7), (10, 4), (-16, 19), (5, 25), (-13, 21), (-3, 12), (7, -11), (-7, 0),
        (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0),
    ];

    #[test]
    fn test_pawn_table_conjunction() {
        let tables = PositionTables::default();
        for i in 0..64 {
            assert_eq!(PAWN[i as usize].0, tables.midgame(Piece(Side::W, Class::P), i.into()))
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
        assert_eq!(-7, tables.midgame(Piece(Side::W, Class::P), C6));
        assert_eq!(7, tables.midgame(Piece(Side::B, Class::P), C3));

        assert_eq!(19, tables.midgame(Piece(Side::W, Class::N), D3));
        assert_eq!(-19, tables.midgame(Piece(Side::B, Class::N), D6));

        assert_eq!(26, tables.midgame(Piece(Side::W, Class::B), C4));
        assert_eq!(-26, tables.midgame(Piece(Side::B, Class::B), C5));

        assert_eq!(-5, tables.midgame(Piece(Side::W, Class::R), F2));
        assert_eq!(5, tables.midgame(Piece(Side::B, Class::R), F7));

        assert_eq!(6, tables.midgame(Piece(Side::W, Class::Q), B3));
        assert_eq!(-6, tables.midgame(Piece(Side::B, Class::Q), B6));

        assert_eq!(325, tables.midgame(Piece(Side::W, Class::K), B1));
        assert_eq!(-325, tables.midgame(Piece(Side::B, Class::K), B8));
    }

    #[test]
    fn test_endgame() {
        let tables = PositionTables::default();
        assert_eq!(21, tables.endgame(Piece(Side::W, Class::P), C6));
        assert_eq!(-21, tables.endgame(Piece(Side::B, Class::P), C3));

        assert_eq!(-18, tables.endgame(Piece(Side::W, Class::N), E1));
        assert_eq!(18, tables.endgame(Piece(Side::B, Class::N), E8));

        assert_eq!(16, tables.endgame(Piece(Side::W, Class::B), D4));
        assert_eq!(-16, tables.endgame(Piece(Side::B, Class::B), D5));

        assert_eq!(-2, tables.endgame(Piece(Side::W, Class::R), D3));
        assert_eq!(2, tables.endgame(Piece(Side::B, Class::R), D6));

        assert_eq!(-23, tables.endgame(Piece(Side::W, Class::Q), A4));
        assert_eq!(23, tables.endgame(Piece(Side::B, Class::Q), A5));

        assert_eq!(141, tables.endgame(Piece(Side::W, Class::K), D7));
        assert_eq!(-141, tables.endgame(Piece(Side::B, Class::K), D2));
    }
}
