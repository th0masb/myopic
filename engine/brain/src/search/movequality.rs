use myopic_board::{Board, Move, Move::*, Reflectable, Square};

use crate::{BitBoard, Class, Evaluator, Piece, PositionTables};

/// A function which approximately evaluates the quality
/// of a move within the context of the given position.
/// It can be used to decide the search order of legal
/// moves for a position.
pub trait BestMoveHeuristic {
    /// Assign a heuristic score to the given move in the
    /// context of the given position. The score is agnostic
    /// of the side to move, i.e. high magnitude positive
    /// score is always better and high magnitude negative
    /// score is always worse.
    fn estimate(&self, board: &Evaluator, mv: &Move) -> i32;
}

/// Simplest estimator which simply evaluates all moves
/// as equal.
pub struct AllMovesEqualHeuristic;

impl BestMoveHeuristic for AllMovesEqualHeuristic {
    fn estimate(&self, _board: &Evaluator, _mv: &Move) -> i32 {
        0
    }
}

/// Main private of the heuristic move estimator trait,
/// it categorises moves into one of four subcategories from
/// best (good exchanges) to worst (bad exchanges) and then
/// also orders within those subcategories.
pub struct MaterialAndPositioningHeuristic {
    tables: PositionTables,
}

impl Default for MaterialAndPositioningHeuristic {
    fn default() -> Self {
        MaterialAndPositioningHeuristic { tables: PositionTables::default() }
    }
}

impl MaterialAndPositioningHeuristic {
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
                            MoveCategory::Positional(
                                self.tables.midgame(class, dest) - self.tables.midgame(class, from),
                            )
                        })
                }
            }
        }
    }
}

// Idea is we split the moves into different categories which are ordered
// so that if category A has more value than category B then all moves in
// A are assigned a greater value than those in B. There is then further
// ordering within categories so certain moves in A can be better than others.
// The categories are (in order best to worst):
//
// 1 Good exchanges
// 2 Special moves
// 3 Positional moves
// 4 Move to an area of control of lower value piece | bad exchanges
//
// The positional moves are those left over when other categories are computed
// and their sub-ordering is according to the delta in position value according
// to the tables.
//
// Special moves (castling, enpassant, promotions) don't really need sub-ordering
//
// Exchanges are ordering according to the resulting material delta as
// computed by the SEE. Moving to an area of control for a lower value piece
// is scored according to the delta between the piece values. For now ignore
// potential pins.
impl BestMoveHeuristic for MaterialAndPositioningHeuristic {
    fn estimate(&self, board: &Evaluator, mv: &Move) -> i32 {
        match self.get_category(board, mv) {
            MoveCategory::GoodExchange(n) => 30_000 + n,
            MoveCategory::Special => 20_000,
            MoveCategory::Positional(n) => 10_000 + n,
            MoveCategory::BadExchange(n) => n,
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
