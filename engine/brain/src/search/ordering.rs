use myopic_board::{Move, Move::*, Reflectable, Side, Square};

use crate::{BitBoard, EvalChessBoard, Piece, Piece::*};

/// A function which approximately evaluates the quality
/// of a move within the context of the given position.
/// It can be used to decide the search order of legal
/// moves for a position.
pub trait MoveQualityEstimator<B: EvalChessBoard> {
    /// Assign a heuristic score to the given move in the
    /// context of the given position. The score is agnostic
    /// of the side to move, i.e. high magnitude positive
    /// score is always better and high magnitude negative
    /// score is always worse.
    fn estimate(&self, board: &mut B, mv: &Move) -> i32;
}

/// Simplest estimator which simply evaluates all moves
/// as equal.
pub struct ConstantEstimator;
impl<B: EvalChessBoard> MoveQualityEstimator<B> for ConstantEstimator {
    fn estimate(&self, _board: &mut B, _mv: &Move) -> i32 {
        0
    }
}

/// Main imp of the heuristic move estimator trait,
/// it categorises moves into one of four subcategories from
/// best (good exchanges) to worst (bad exchanges) and then
/// also orders within those subcategories.
pub struct EstimatorImpl;
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
impl<B: EvalChessBoard> MoveQualityEstimator<B> for EstimatorImpl {
    fn estimate(&self, board: &mut B, mv: &Move) -> i32 {
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

fn get_category<B: EvalChessBoard>(board: &mut B, mv: &Move) -> MoveCategory {
    match mv {
        Enpassant { .. } | Castle { .. } | Promotion { .. } => MoveCategory::Special,
        &Standard {
            moving, from, dest, ..
        } => {
            if board.side(moving.side().reflect()).contains(dest) {
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
                            parity(moving.side())
                                * (board.positional_eval(moving, dest)
                                    - board.positional_eval(moving, from)),
                        )
                    })
            }
        }
    }
}

fn get_lower_value_delta<B: EvalChessBoard>(
    board: &mut B,
    piece: Piece,
    dst: Square,
) -> Option<i32> {
    let piece_values = board.piece_values().clone();
    let moving_value = piece_values[piece as usize % 6];
    get_lower_value_pieces(piece)
        .into_iter()
        .filter(|&p| compute_control(board, *p).contains(dst))
        .map(|&p| piece_values[p as usize % 6] - moving_value)
        .min()
}

fn get_lower_value_pieces<'a>(piece: Piece) -> &'a [Piece] {
    match piece {
        WP | BP => &[],

        WN | WB => &[BP],
        WR => &[BP, BN, BB],
        WQ => &[BP, BN, BB, BR],
        WK => &[BP, BN, BB, BR, BQ],

        BN | BB => &[WP],
        BR => &[WP, WN, WB],
        BQ => &[WP, WN, WB, WR],
        BK => &[WP, WN, WB, WR, WQ],
    }
}

fn compute_control<B: EvalChessBoard>(board: &mut B, piece: Piece) -> BitBoard {
    let (white, black) = board.sides();
    board
        .locs(&[piece])
        .iter()
        .map(|s| piece.control(s, white, black))
        .fold(BitBoard::EMPTY, |l, r| l | r)
}

fn parity(side: Side) -> i32 {
    match side {
        Side::White => 1,
        Side::Black => -1,
    }
}
