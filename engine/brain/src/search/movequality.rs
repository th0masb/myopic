use myopic_board::{Move, Move::*, Reflectable, Side, Square};

use crate::{BitBoard, Class, EvalChessBoard, Piece};

/// A function which approximately evaluates the quality
/// of a move within the context of the given position.
/// It can be used to decide the search order of legal
/// moves for a position.
pub trait BestMoveHeuristic<B: EvalChessBoard> {
    /// Assign a heuristic score to the given move in the
    /// context of the given position. The score is agnostic
    /// of the side to move, i.e. high magnitude positive
    /// score is always better and high magnitude negative
    /// score is always worse.
    fn estimate(&self, board: &B, mv: &Move) -> i32;
}

/// Simplest estimator which simply evaluates all moves
/// as equal.
pub struct AllMovesEqualHeuristic;

impl<B: EvalChessBoard> BestMoveHeuristic<B> for AllMovesEqualHeuristic {
    fn estimate(&self, _board: &B, _mv: &Move) -> i32 {
        0
    }
}

/// Main imp of the heuristic move estimator trait,
/// it categorises moves into one of four subcategories from
/// best (good exchanges) to worst (bad exchanges) and then
/// also orders within those subcategories.
pub struct MaterialAndPositioningHeuristic;

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
impl<B: EvalChessBoard> BestMoveHeuristic<B> for MaterialAndPositioningHeuristic {
    fn estimate(&self, board: &B, mv: &Move) -> i32 {
        match get_category(board, mv) {
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

fn get_category<B: EvalChessBoard>(board: &B, mv: &Move) -> MoveCategory {
    match mv {
        Enpassant { .. } | Castle { .. } | Promotion { .. } => MoveCategory::Special,
        &Standard { moving, from, dest, .. } => {
            if board.side(moving.0.reflect()).contains(dest) {
                let exchange_value =
                    crate::see::exchange_value(board, from, dest, board.piece_values());
                if exchange_value > 0 {
                    MoveCategory::GoodExchange(exchange_value)
                } else {
                    MoveCategory::BadExchange(exchange_value)
                }
            } else {
                get_lower_value_delta(board, moving, dest)
                    .map(|n| MoveCategory::BadExchange(n))
                    .unwrap_or_else(|| {
                        MoveCategory::Positional(
                            parity(moving.0)
                                * (board.positional_eval(moving, dest)
                                    - board.positional_eval(moving, from)),
                        )
                    })
            }
        }
    }
}

fn get_lower_value_delta<B: EvalChessBoard>(board: &B, piece: Piece, dst: Square) -> Option<i32> {
    let piece_values = board.piece_values().clone();
    let moving_value = piece_values[piece.1 as usize];
    get_lower_value_pieces(piece.1)
        .into_iter()
        .map(|&class| Piece(piece.0.reflect(), class))
        .filter(|p| compute_control(board, *p).contains(dst))
        .map(|p| piece_values[p.1 as usize] - moving_value)
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

fn compute_control<B: EvalChessBoard>(board: &B, piece: Piece) -> BitBoard {
    let (white, black) = board.sides();
    board
        .locs(&[piece])
        .iter()
        .map(|s| piece.control(s, white | black))
        .fold(BitBoard::EMPTY, |l, r| l | r)
}

fn parity(side: Side) -> i32 {
    match side {
        Side::W => 1,
        Side::B => -1,
    }
}
