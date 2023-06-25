use crate::eval::{EvalFacet, Evaluation};
use crate::{Class, Piece, Reflectable, Square};
use enum_map::{enum_map, EnumMap};
use myopic_board::{Board, Move};

pub type PieceValues = EnumMap<Class, i32>;

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
            mid_values: enum_map! {
                Class::P => 200,
                Class::N => 782,
                Class::B => 830,
                Class::R => 1289,
                Class::Q => 2529,
                Class::K => 100_000
            },
            end_values: enum_map! {
                Class::P => 293,
                Class::N => 865,
                Class::B => 918,
                Class::R => 1378,
                Class::Q => 2687,
                Class::K => 100_000
            },
        }
    }
}

impl<'a> From<&'a Board> for MaterialFacet {
    fn from(value: &Board) -> Self {
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

    pub fn compute_midgame_eval(&self, board: &Board) -> i32 {
        Square::iter()
            .flat_map(|square| board.piece(square))
            .map(|Piece(side, class)| side.parity() * self.mid_values[class])
            .sum()
    }

    pub fn compute_endgame_eval(&self, board: &Board) -> i32 {
        Square::iter()
            .flat_map(|square| board.piece(square))
            .map(|Piece(side, class)| side.parity() * self.end_values[class])
            .sum()
    }

    fn remove(&mut self, Piece(side, class): Piece) {
        let parity = side.parity();
        self.mid_eval -= parity * self.mid_values[class];
        self.end_eval -= parity * self.end_values[class];
    }

    fn add(&mut self, Piece(side, class): Piece) {
        let parity = side.parity();
        self.mid_eval += parity * self.mid_values[class];
        self.end_eval += parity * self.end_values[class];
    }

    fn make_impl(&mut self, mv: &Move, add: UpdateFn, remove: UpdateFn) {
        match mv {
            Move::Castle { .. } | Move::Null => {}
            Move::Standard { capture, .. } => {
                if let Some(piece) = capture {
                    remove(self, *piece);
                }
            }
            Move::Enpassant { side, .. } => {
                remove(self, Piece(side.reflect(), Class::P));
            }
            Move::Promotion { promoted: Piece(side, class), capture, .. } => {
                remove(self, Piece(*side, Class::P));
                add(self, Piece(*side, *class));
                if let Some(p) = capture {
                    remove(self, *p)
                }
            }
        }
    }
}

impl EvalFacet for MaterialFacet {
    fn static_eval(&self, _: &Board) -> Evaluation {
        Evaluation::Phased { mid: self.mid_eval, end: self.end_eval }
    }

    fn make(&mut self, mv: &Move, _: &Board) {
        self.make_impl(mv, MaterialFacet::add, MaterialFacet::remove)
    }

    fn unmake(&mut self, mv: &Move) {
        self.make_impl(mv, MaterialFacet::remove, MaterialFacet::add)
    }
}
