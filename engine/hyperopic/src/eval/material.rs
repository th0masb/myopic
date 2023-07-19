use crate::constants::{class, create_piece, piece_class, piece_side, side_parity};
use crate::{ClassMap, Piece};

use crate::moves::Move;
use crate::node::{EvalFacet, Evaluation};
use crate::position::Position;

pub type PieceValues = ClassMap<i32>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaterialFacet {
    mid_values: PieceValues,
    end_values: PieceValues,
    mid_eval: i32,
    end_eval: i32,
}

impl Default for MaterialFacet {
    fn default() -> Self {
        MaterialFacet {
            mid_eval: 0,
            end_eval: 0,
            mid_values: [230, 782, 830, 1289, 2529, 100_000],
            end_values: [300, 865, 918, 1378, 2687, 100_000],
        }
    }
}

impl<'a> From<&'a Position> for MaterialFacet {
    fn from(value: &Position) -> Self {
        let mut facet = MaterialFacet::default();
        facet.mid_eval = facet.compute_midgame_eval(value);
        facet.end_eval = facet.compute_endgame_eval(value);
        facet
    }
}

type UpdateFn = fn(&mut MaterialFacet, Piece) -> ();

impl MaterialFacet {
    pub fn mid_values(&self) -> &PieceValues {
        &self.mid_values
    }

    pub fn compute_midgame_eval(&self, board: &Position) -> i32 {
        (0..64)
            .flat_map(|square| board.piece_locs[square])
            .map(|p| side_parity(piece_side(p)) * self.mid_values[piece_class(p)])
            .sum()
    }

    pub fn compute_endgame_eval(&self, board: &Position) -> i32 {
        (0..64)
            .flat_map(|square| board.piece_locs[square])
            .map(|p| side_parity(piece_side(p)) * self.end_values[piece_class(p)])
            .sum()
    }

    fn remove(&mut self, piece: Piece) {
        let class = piece_class(piece);
        let parity = side_parity(piece_side(piece));
        self.mid_eval -= parity * self.mid_values[class];
        self.end_eval -= parity * self.end_values[class];
    }

    fn add(&mut self, piece: Piece) {
        let class = piece_class(piece);
        let parity = side_parity(piece_side(piece));
        self.mid_eval += parity * self.mid_values[class];
        self.end_eval += parity * self.end_values[class];
    }

    fn make_impl(&mut self, mv: &Move, add: UpdateFn, remove: UpdateFn) {
        match mv {
            Move::Castle { .. } | Move::Null => {}
            Move::Normal { capture, .. } => {
                if let Some(piece) = capture {
                    remove(self, *piece);
                }
            }
            Move::Enpassant { side, .. } => {
                remove(self, create_piece(*side, class::P));
            }
            Move::Promote { promoted, capture, .. } => {
                let side = piece_side(*promoted);
                remove(self, create_piece(side, class::P));
                add(self, *promoted);
                if let Some(p) = capture {
                    remove(self, *p)
                }
            }
        }
    }
}

impl EvalFacet for MaterialFacet {
    fn static_eval(&self, _: &Position) -> Evaluation {
        Evaluation::Phased { mid: self.mid_eval, end: self.end_eval }
    }

    fn make(&mut self, mv: &Move, _: &Position) {
        self.make_impl(mv, MaterialFacet::add, MaterialFacet::remove)
    }

    fn unmake(&mut self, mv: &Move) {
        self.make_impl(mv, MaterialFacet::remove, MaterialFacet::add)
    }
}
