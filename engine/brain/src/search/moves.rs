use myopic_board::{Board, Move, Move::*, Moves, Reflectable, Square};

use crate::negascout::SearchContext;
use crate::search::pv::PrincipleVariation;
use crate::{BitBoard, Class, Evaluator, Piece, PositionTables};

pub struct MoveGenerator<'a> {
    pv: &'a PrincipleVariation,
    estimator: MaterialAndPositioningHeuristic,
}

impl<'a> From<&'a PrincipleVariation> for MoveGenerator<'a> {
    fn from(value: &'a PrincipleVariation) -> Self {
        MoveGenerator { pv: value, estimator: Default::default() }
    }
}

impl MoveGenerator<'_> {
    pub fn generate(
        &self,
        state: &Evaluator,
        ctx: &SearchContext,
        table: Option<&Move>,
    ) -> impl Iterator<Item = Move> {
        let mut moves = state.board().moves(Moves::All);
        moves.sort_by_cached_key(|m| self.estimator.estimate(state, m));
        table.map(|t| reposition_last(&mut moves, t));
        if let Some(pv) = self.pv.get_next_move(ctx.precursors.as_slice()) {
            reposition_last(&mut moves, &pv);
        }
        moves.into_iter().rev()
    }
}

fn reposition_last(dest: &mut Vec<Move>, new_first: &Move) {
    if let Some(index) = dest.iter().position(|m| m == new_first) {
        let removed = dest.remove(index);
        dest.push(removed);
    }
}

/// Main private of the heuristic move estimator trait,
/// it categorises moves into one of four subcategories from
/// best (good exchanges) to worst (bad exchanges) and then
/// also orders within those subcategories.
#[derive(Default)]
struct MaterialAndPositioningHeuristic {
    tables: PositionTables,
}

impl MaterialAndPositioningHeuristic {
    fn estimate(&self, board: &Evaluator, mv: &Move) -> i32 {
        match self.get_category(board, mv) {
            MoveCategory::GoodExchange(n) => 30_000 + n,
            MoveCategory::Special => 20_000,
            MoveCategory::Positional(n) => 10_000 + n,
            MoveCategory::BadExchange(n) => n,
        }
    }

    fn get_category(&self, eval: &Evaluator, mv: &Move) -> MoveCategory {
        match mv {
            Enpassant { .. } | Castle { .. } | Promotion { .. } => MoveCategory::Special,
            &Standard { moving: Piece(side, class), from, dest, .. } => {
                if eval.board().side(side.reflect()).contains(dest) {
                    let exchange_value = eval.see(from, dest);
                    if exchange_value > 0 {
                        MoveCategory::GoodExchange(exchange_value)
                    } else {
                        MoveCategory::BadExchange(exchange_value)
                    }
                } else {
                    get_lower_value_delta(eval, Piece(side, class), dest)
                        .map(|n| MoveCategory::BadExchange(n))
                        .unwrap_or_else(|| {
                            let moving = Piece(side, class);
                            let from_value = self.tables.midgame(moving, from);
                            let dest_value = self.tables.midgame(moving, dest);
                            MoveCategory::Positional(side.parity() * (dest_value - from_value))
                        })
                }
            }
        }
    }
}

enum MoveCategory {
    // Wraps the see exchange value, > 0
    GoodExchange(i32),
    Special,
    // Wraps the position table value
    Positional(i32),
    // Wraps the see exchange value <= 0
    BadExchange(i32),
}

fn get_lower_value_delta(eval: &Evaluator, piece: Piece, dst: Square) -> Option<i32> {
    let piece_values = eval.piece_values();
    let moving_value = piece_values[piece.1];
    get_lower_value_pieces(piece.1)
        .into_iter()
        .map(|&class| Piece(piece.0.reflect(), class))
        .filter(|p| compute_control(eval.board(), *p).contains(dst))
        .map(|p| piece_values[p.1] - moving_value)
        .min()
}

fn get_lower_value_pieces<'a>(class: Class) -> &'a [Class] {
    match class {
        Class::P => &[],
        Class::N | Class::B => &[Class::P],
        Class::R => &[Class::P, Class::N, Class::B],
        Class::Q => &[Class::P, Class::N, Class::B, Class::R],
        Class::K => &[Class::P, Class::N, Class::B, Class::R, Class::Q],
    }
}

fn compute_control(board: &Board, piece: Piece) -> BitBoard {
    let (white, black) = board.sides();
    board
        .locs(&[piece])
        .iter()
        .map(|s| piece.control(s, white | black))
        .fold(BitBoard::EMPTY, |l, r| l | r)
}
