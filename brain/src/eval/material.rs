use crate::{PieceValues, PositionTables, Reflectable};
use myopic_board::{Move, Piece, Square, ChessBoard};
use crate::eval::EvalComponent;

const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];
const TOTAL_PHASE: i32 = 16 * PHASE_VALUES[0]
    + 4 * (PHASE_VALUES[1] + PHASE_VALUES[2] + PHASE_VALUES[3])
    + 2 * PHASE_VALUES[4];

#[derive(Clone, PartialEq, Eq)]
pub struct Material {
    piece_values: PieceValues,
    table_values: PositionTables,
    mid_eval: i32,
    end_eval: i32,
    phase: i32,
}

impl Reflectable for Material {
    fn reflect(&self) -> Self {
        Material {
            mid_eval: -self.mid_eval,
            end_eval: -self.end_eval,
            phase: self.phase,
            piece_values: self.piece_values.clone(),
            table_values: self.table_values.clone(),
        }
    }
}

impl EvalComponent for Material {
    fn static_eval(&mut self) -> i32 {
        let phase: i32 = ((self.phase * 256 + TOTAL_PHASE / 2) / TOTAL_PHASE) as i32;
        let (mid, end) = (self.mid_eval, self.end_eval);
        ((mid * (256 - phase)) + end * phase) / 256
    }

    fn make(&mut self, mv: &Move) {
        match mv {
            &Move::Standard{moving, from, dest, capture, ..} => {
                self.remove(moving, from);
                self.add(moving, dest);
                capture.map(|taken| self.remove(taken, dest));
            }

            &Move::Promotion { from, dest, promoted, capture, .. } => {
                let pawn = Piece::pawn(promoted.side());
                self.remove(pawn, from);
                self.add(promoted, dest);
                capture.map(|taken| self.remove(taken, dest));
            }

            &Move::Enpassant { side, from, dest, capture, .. } => {
                let active_pawn = Piece::pawn(side);
                self.remove(active_pawn, from);
                self.add(active_pawn, dest);
                self.remove(active_pawn.reflect(), capture);
            }

            &Move::Castle { zone, .. } => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
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
                let pawn = Piece::pawn(promoted.side());
                self.add(pawn, from);
                self.remove(promoted, dest);
                capture.map(|taken| self.add(taken, dest));
            }

            &Move::Enpassant { side, from, dest, capture, .. } => {
                let active_pawn = Piece::pawn(side);
                let passive_pawn = active_pawn.reflect();
                self.remove(active_pawn, dest);
                self.add(active_pawn, from);
                self.add(passive_pawn, capture);
            }
            &Move::Castle { zone, .. } => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
                self.add(rook, r_src);
                self.remove(rook, r_target);
                self.add(king, k_src);
                self.remove(king, k_target);
            }
        };
    }

    fn replicate(&self) -> Box<dyn EvalComponent> {
        Box::new(self.clone())
    }
}

impl Material {
    pub fn new<B: ChessBoard>(board: &B, values: PieceValues, tables: PositionTables) -> Material {
        Material {
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

    pub fn mid_eval(&self) -> i32 {
        self.mid_eval
    }

    pub fn end_eval(&self) -> i32 {
        self.end_eval
    }

    fn remove(&mut self, piece: Piece, location: Square) {
        let (tables, values) = (&self.table_values, &self.piece_values);
        self.mid_eval -= tables.midgame(piece, location) + values.midgame(piece);
        self.end_eval -= tables.endgame(piece, location) + values.endgame(piece);
        self.phase += PHASE_VALUES[(piece as usize) % 6];
    }

    fn add(&mut self, piece: Piece, location: Square) {
        let (tables, values) = (&self.table_values, &self.piece_values);
        self.mid_eval += tables.midgame(piece, location) + values.midgame(piece);
        self.end_eval += tables.endgame(piece, location) + values.endgame(piece);
        self.phase -= PHASE_VALUES[(piece as usize) % 6];
    }
}

pub fn compute_phase<B: ChessBoard>(board: &B) -> i32 {
    let pieces: Vec<_> = Piece::whites()
        .take(5)
        .chain(Piece::blacks().take(5))
        .collect();
    let phase_sub: i32 = pieces
        .into_iter()
        .map(|p| board.locs(&[p]).size() as i32 * PHASE_VALUES[(p as usize) % 6])
        .sum();
    TOTAL_PHASE - phase_sub
}

pub fn compute_midgame<B: ChessBoard>(board: &B, values: &PieceValues, tables: &PositionTables) -> i32 {
    Piece::all()
        .flat_map(|p| board.locs(&[p]).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables.midgame(p, loc) + values.midgame(p))
        .sum()
}

pub fn compute_endgame<B: ChessBoard>(board: &B, values: &PieceValues, tables: &PositionTables) -> i32 {
    Piece::all()
        .flat_map(|p| board.locs(&[p]).iter().map(move |loc| (p, loc)))
        .map(|(p, loc)| tables.endgame(p, loc) + values.endgame(p))
        .sum()
}
