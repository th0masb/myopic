use crate::board::{control, iter, union_boards};
use crate::constants::{
    class, create_piece, in_board, piece_class, piece_side, reflect_side, side_parity,
};
use crate::moves::Move::{Castle, Enpassant, Normal, Null, Promote};
use crate::moves::{Move, Moves};
use crate::node::SearchNode;
use crate::position::Position;
use crate::search::negascout::Context;
use crate::search::pv::PrincipleVariation;
use crate::tables::PositionTables;
use crate::{Board, Class, Piece, Square};

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
        state: &SearchNode,
        ctx: &Context,
        table: Option<&Move>,
    ) -> impl Iterator<Item = Move> {
        let mut moves = state.position().moves(&Moves::All);
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
    fn estimate(&self, board: &SearchNode, mv: &Move) -> i32 {
        match self.get_category(board, mv) {
            MoveCategory::GoodExchange(n) => 30_000 + n,
            MoveCategory::Special => 20_000,
            MoveCategory::Positional(n) => 10_000 + n,
            MoveCategory::BadExchange(n) => n,
        }
    }

    fn get_category(&self, eval: &SearchNode, mv: &Move) -> MoveCategory {
        match mv {
            Null | Enpassant { .. } | Castle { .. } | Promote { .. } => MoveCategory::Special,
            &Normal { moving, from, dest, capture } => {
                if capture.is_some() {
                    let exchange_value = eval.see(from, dest);
                    if exchange_value > 0 {
                        MoveCategory::GoodExchange(exchange_value)
                    } else {
                        MoveCategory::BadExchange(exchange_value)
                    }
                } else {
                    get_lower_value_delta(eval, moving, dest)
                        .map(|n| MoveCategory::BadExchange(n))
                        .unwrap_or_else(|| {
                            let side = piece_side(moving);
                            let from_value = self.tables.midgame(moving, from);
                            let dest_value = self.tables.midgame(moving, dest);
                            MoveCategory::Positional(side_parity(side) * (dest_value - from_value))
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

fn get_lower_value_delta(eval: &SearchNode, piece: Piece, dst: Square) -> Option<i32> {
    let piece_values = eval.piece_values();
    let p_class = piece_class(piece);
    let moving_value = piece_values[p_class];
    get_lower_value_pieces(p_class)
        .into_iter()
        .map(|&class| create_piece(reflect_side(piece_side(piece)), class))
        .filter(|p| in_board(compute_control(eval.position(), *p), dst))
        .map(|p| piece_values[piece_class(p)] - moving_value)
        .min()
}

fn get_lower_value_pieces<'a>(class: Class) -> &'a [Class] {
    match class {
        class::P => &[],
        class::N | class::B => &[class::P],
        class::R => &[class::P, class::N, class::B],
        class::Q => &[class::P, class::N, class::B, class::R],
        class::K => &[class::P, class::N, class::B, class::R, class::Q],
        _ => panic!("{} not a valid piece class", class),
    }
}

fn compute_control(board: &Position, piece: Piece) -> Board {
    let occupied = union_boards(&board.side_boards);
    iter(board.piece_boards[piece]).fold(0u64, |a, n| a | control(piece, n, occupied))
}
