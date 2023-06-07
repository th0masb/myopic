use enum_map::{enum_map, EnumMap};

use crate::eval::{EvalFacet, Evaluation};
use crate::{Class, Corner, Line};
use myopic_board::{Board, Move, Piece, Reflectable, Square};

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
            .map(|(Piece(side, class), square)| side.parity() * self.tables.midgame(class, square))
            .sum()
    }

    pub fn compute_endgame_eval(&self, board: &Board) -> i32 {
        Square::iter()
            .flat_map(|square| board.piece(square).map(|p| (p, square)))
            .map(|(Piece(side, class), square)| side.parity() * self.tables.endgame(class, square))
            .sum()
    }

    fn add(&mut self, Piece(side, class): Piece, square: Square) {
        let parity = side.parity();
        self.mid_eval += parity * self.tables.midgame(class, square);
        self.end_eval += parity * self.tables.endgame(class, square);
    }

    fn remove(&mut self, Piece(side, class): Piece, square: Square) {
        let parity = side.parity();
        self.mid_eval -= parity * self.tables.midgame(class, square);
        self.end_eval -= parity * self.tables.endgame(class, square);
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct PositionTables {
    tables: EnumMap<Class, EnumMap<Square, (i32, i32)>>,
}

impl PositionTables {
    /// API method for retrieving the evaluation for a piece at a given location
    /// in the midgame.
    pub fn midgame(&self, class: Class, location: Square) -> i32 {
        self.tables[class][location].0
    }

    /// API method for retrieving the evaluation for a piece at a given location
    /// in the midgame.
    pub fn endgame(&self, class: Class, location: Square) -> i32 {
        self.tables[class][location].1
    }
}

impl Default for PositionTables {
    fn default() -> Self {
        PositionTables {
            tables: enum_map! {
                Class::P => parse_full(PAWN),
                Class::N => parse_symmetric(KNIGHT),
                Class::B => parse_symmetric(BISHOP),
                Class::R => parse_symmetric(ROOK),
                Class::Q => parse_symmetric(QUEEN),
                Class::K => parse_symmetric(KING),
            },
        }
    }
}

type SymmetricTable = [(i32, i32); 32];
type CompleteTable = [(i32, i32); 64];

fn parse_symmetric(raw: SymmetricTable) -> EnumMap<Square, (i32, i32)> {
    let mut table = enum_map! { _ => (0, 0) };
    Square::iter().for_each(|square| {
        let (rank, file) = (square.rank_index(), square.file_index());
        let column = if file < 4 { file } else { 7 - file };
        table[square] = raw[4 * rank + column];
    });
    table
}

fn parse_full(raw: CompleteTable) -> EnumMap<Square, (i32, i32)> {
    let mut table = enum_map! { _ => (0, 0) };
    Square::iter().for_each(|square| {
        table[square] = raw[8 * square.rank_index() + square.file_index()];
    });
    table
}

/// Tables lifted from stockfish here: https://github.com/official-stockfish/Stockfish/blob/master/src/psqt.cpp
/// They are (mid, end) values for white side on files H - E
#[rustfmt::skip]
const KNIGHT: SymmetricTable = [
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
const BISHOP: SymmetricTable = [
    // Rank 1
    (-44, -63), ( -4, -30), (-11, -35), (-28,  -8),
    (-18, -38), (  7, -13), ( 14, -14), (  3,   0),
    ( -8, -18), ( 24,   0), ( -3,  -7), ( 15,  13),
    (  1, -26), (  8,  -3), ( 26,   1), ( 37,  16),
    ( -7, -24), ( 30,  -6), ( 23, -10), ( 28,  17),
    (-17, -26), (  4,   2), ( -1,   1), (  8,  16),
    (-21, -34), (-19, -18), ( 10,  -7), ( -6,   9),
    (-48, -51), ( -3, -40), (-12, -39), (-25, -20),
    // Rank 8
];

#[rustfmt::skip]
const ROOK: SymmetricTable = [
    // Rank 1
    (-24,  -2), (-13, -6), (-7,  -3), ( 2, -2),
    (-18, -10), (-10, -7), (-5,   1), ( 9,  0),
    (-21,  10), ( -7, -4), ( 3,   2), (-1, -2),
    (-13,  -5), ( -5,  2), (-4,  -8), (-6,  8),
    (-24,  -8), (-12,  5), (-1,   4), ( 6, -9),
    (-24,   3), ( -4, -2), ( 4, -10), (10,  7),
    ( -8,   1), (  6,  2), (10,  17), (12, -8),
    (-22,  12), (-24, -6), (-6,  13), ( 4,  7),
    // Rank 8
];

#[rustfmt::skip]
const QUEEN: SymmetricTable = [
    // Rank 1
    ( 3, -69), (-5, -57), (-5, -47), ( 4, -26),
    (-3, -55), ( 5, -31), ( 8, -22), (12,  -4),
    (-3, -39), ( 6, -18), (13,  -9), ( 7,   3),
    ( 4, -23), ( 5,  -3), ( 9,  13), ( 8,  24),
    ( 0, -29), (14,  -6), (12,   9), ( 5,  21),
    (-4, -38), (10, -18), ( 6, -12), ( 8,   1),
    (-5, -50), ( 6, -27), (10, -24), ( 8,  -8),
    (-2, -75), (-2, -52), ( 1, -43), (-2, -36),
    // Rank 8
];

#[rustfmt::skip]
const KING: SymmetricTable = [
    // Rank 1
    (272,   0), (325,  41), (273,  80), (190,  93),
    (277,  57), (305,  98), (241, 138), (183, 131),
    (198,  86), (253, 138), (168, 165), (120, 173),
    (169, 103), (191, 152), (136, 168), (108, 169),
    (145,  98), (176, 166), (112, 197), ( 69, 194),
    (122,  87), (159, 164), ( 85, 174), ( 36, 189),
    ( 87,  40), (120,  99), ( 64, 128), ( 25, 141),
    ( 64,   5), ( 87,  60), ( 49,  75), (  0,  75),
    // Rank 8
];

#[rustfmt::skip]
const PAWN: CompleteTable = [
    // Rank 1
    (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
    ( -5, -19), (  7,  -5), ( 19,   7), (-20, -20), (-20, -20), ( 10,  10), (  3,  -6), ( 3, -10),
    (-22,  -4), (  5,  -6), ( 22,   3), ( 32,   4), ( 15,   4), ( 11, -10), (-15, -10), (-9, -10),
    (-12,  -9), (  4, -10), ( 17, -12), ( 40, -13), ( 20,  -4), (  6,  -8), (-23,  -2), (-8,   6),
    (  5,   8), (-13,  13), ( -2,  -6), ( 11, -12), (  1, -12), (-13,   3), (  0,   4), (13,   9),
    (-18,  13), (-15,   6), ( -5,   7), ( -8,  30), ( 22,  28), ( -7,  21), (-12,  20), (-5,  28),
    ( -8,   7), ( 10,   4), (-16,  19), (  5,  25), (-13,  21), ( -3,  12), (  7, -11), (-7,   0),
    (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), (  0,   0), ( 0,   0),
    // Rank 8
];

#[cfg(test)]
mod test {
    use myopic_board::Square::*;

    use crate::{Class, PositionTables};

    #[test]
    fn test_midgame() {
        let tables = PositionTables::default();
        assert_eq!(-7, tables.midgame(Class::P, C6));
        assert_eq!(19, tables.midgame(Class::N, D3));
        assert_eq!(26, tables.midgame(Class::B, C4));
        assert_eq!(-5, tables.midgame(Class::R, F2));
        assert_eq!(6, tables.midgame(Class::Q, B3));
        assert_eq!(325, tables.midgame(Class::K, B1));
    }

    #[test]
    fn test_endgame() {
        let tables = PositionTables::default();
        assert_eq!(21, tables.endgame(Class::P, C6));
        assert_eq!(-18, tables.endgame(Class::N, E1));
        assert_eq!(16, tables.endgame(Class::B, D4));
        assert_eq!(-2, tables.endgame(Class::R, D3));
        assert_eq!(-23, tables.endgame(Class::Q, A4));
        assert_eq!(141, tables.endgame(Class::K, D7));
    }
}
