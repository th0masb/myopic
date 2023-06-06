use crate::eval::EvalFacet;
use crate::{ChessBoard, Line, Move, Piece, Square};
use crate::{PieceValues, PositionTables, Reflectable};
use myopic_board::Class;

const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];
const TOTAL_PHASE: i32 = 16 * PHASE_VALUES[0]
    + 4 * (PHASE_VALUES[1] + PHASE_VALUES[2] + PHASE_VALUES[3])
    + 2 * PHASE_VALUES[4];

#[derive(Clone, PartialEq, Eq)]
pub struct MaterialFacet {
    piece_values: PieceValues,
    table_values: PositionTables,
    mid_eval: i32,
    end_eval: i32,
    phase: i32,
}

impl Reflectable for MaterialFacet {
    fn reflect(&self) -> Self {
        MaterialFacet {
            mid_eval: -self.mid_eval,
            end_eval: -self.end_eval,
            phase: self.phase,
            piece_values: self.piece_values.clone(),
            table_values: self.table_values.clone(),
        }
    }
}

impl<B: ChessBoard> EvalFacet<B> for MaterialFacet {
    fn static_eval(&self, _: &B) -> i32 {
        let phase: i32 = ((self.phase * 256 + TOTAL_PHASE / 2) / TOTAL_PHASE) as i32;
        let (mid, end) = (self.mid_eval, self.end_eval);
        ((mid * (256 - phase)) + end * phase) / 256
    }

    fn make(&mut self, mv: &Move, _: &B) {
        match mv {
            &Move::Standard { moving, from, dest, capture, .. } => {
                self.remove(moving, from);
                self.add(moving, dest);
                capture.map(|taken| self.remove(taken, dest));
            }
            &Move::Promotion { from, dest, promoted, capture, .. } => {
                let pawn = Piece(promoted.0, Class::P);
                self.remove(pawn, from);
                self.add(promoted, dest);
                capture.map(|taken| self.remove(taken, dest));
            }
            &Move::Enpassant { side, from, dest, capture, .. } => {
                let active_pawn = Piece(side, Class::P);
                self.remove(active_pawn, from);
                self.add(active_pawn, dest);
                self.remove(active_pawn.reflect(), capture);
            }
            &Move::Castle { corner, .. } => {
                let Line(r_src, r_target) = Line::rook_castling(corner);
                let Line(k_src, k_target) = Line::king_castling(corner);
                let rook = Piece(corner.0, Class::R);
                let king = Piece(corner.0, Class::K);
                self.remove(rook, r_src);
                self.add(rook, r_target);
                self.remove(king, k_src);
                self.add(king, k_target);
            }
        };
    }

    fn unmake(&mut self, mv: &Move) {
        match mv {
            &Move::Standard { moving, from, dest, capture, .. } => {
                self.remove(moving, dest);
                self.add(moving, from);
                capture.map(|taken| self.add(taken, dest));
            }
            &Move::Promotion { from, dest, promoted, capture, .. } => {
                let pawn = Piece(promoted.0, Class::P);
                self.add(pawn, from);
                self.remove(promoted, dest);
                capture.map(|taken| self.add(taken, dest));
            }
            &Move::Enpassant { side, from, dest, capture, .. } => {
                let active_pawn = Piece(side, Class::P);
                let passive_pawn = active_pawn.reflect();
                self.remove(active_pawn, dest);
                self.add(active_pawn, from);
                self.add(passive_pawn, capture);
            }
            &Move::Castle { corner, .. } => {
                let Line(r_src, r_target) = Line::rook_castling(corner);
                let Line(k_src, k_target) = Line::king_castling(corner);
                let rook = Piece(corner.0, Class::R);
                let king = Piece(corner.0, Class::K);
                self.add(rook, r_src);
                self.remove(rook, r_target);
                self.add(king, k_src);
                self.remove(king, k_target);
            }
        };
    }
}

impl MaterialFacet {
    pub fn new<B: ChessBoard>(
        board: &B,
        values: PieceValues,
        tables: PositionTables,
    ) -> MaterialFacet {
        MaterialFacet {
            mid_eval: compute_midgame(board, &values, &tables),
            end_eval: compute_endgame(board, &values, &tables),
            phase: compute_phase(board),
            piece_values: values,
            table_values: tables,
        }
    }

    pub fn tables(&self) -> &PositionTables {
        &self.table_values
    }

    pub fn values(&self) -> &PieceValues {
        &self.piece_values
    }

    #[cfg(test)]
    pub fn mid_eval(&self) -> i32 {
        self.mid_eval
    }

    #[cfg(test)]
    pub fn end_eval(&self) -> i32 {
        self.end_eval
    }

    fn remove(&mut self, piece: Piece, location: Square) {
        let (tables, values) = (&self.table_values, &self.piece_values);
        self.mid_eval -= tables.midgame(piece, location) + values.midgame(piece);
        self.end_eval -= tables.endgame(piece, location) + values.endgame(piece);
        self.phase += PHASE_VALUES[piece.1 as usize];
    }

    fn add(&mut self, piece: Piece, location: Square) {
        let (tables, values) = (&self.table_values, &self.piece_values);
        self.mid_eval += tables.midgame(piece, location) + values.midgame(piece);
        self.end_eval += tables.endgame(piece, location) + values.endgame(piece);
        self.phase -= PHASE_VALUES[piece.1 as usize];
    }
}

pub fn compute_phase<B: ChessBoard>(board: &B) -> i32 {
    let phase_sub: i32 = Piece::all()
        .filter(|p| p.1 != Class::K)
        .map(|p| board.locs(&[p]).size() as i32 * PHASE_VALUES[p.1 as usize])
        .sum();
    TOTAL_PHASE - phase_sub
}

pub fn compute_midgame<B: ChessBoard>(
    board: &B,
    values: &PieceValues,
    tables: &PositionTables,
) -> i32 {
    Piece::all()
        .flat_map(|p| board.locs(&[p]).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables.midgame(p, loc) + values.midgame(p))
        .sum()
}

pub fn compute_endgame<B: ChessBoard>(
    board: &B,
    values: &PieceValues,
    tables: &PositionTables,
) -> i32 {
    Piece::all()
        .flat_map(|p| board.locs(&[p]).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables.endgame(p, loc) + values.endgame(p))
        .sum()
}
