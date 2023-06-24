use std::cmp;
use itertools::Itertools;
use Move::{Castle, Enpassant, Promotion, Standard};
use MoveComputeType::{Attacks, AttacksChecks};

use myopic_board::anyhow::Result;
use myopic_board::{Move, MoveComputeType, TerminalState};

use crate::{eval, Evaluator, Piece};

const Q_CHECK_CAP: i32 = -1;
const DELTA_SKIP_MARGIN: i32 = 200;

pub fn full_search(root: &mut Evaluator) -> Result<i32> {
    search(root, -eval::INFTY, eval::INFTY)
}

pub fn search(root: &mut Evaluator, alpha: i32, beta: i32) -> Result<i32> {
    search_impl(root, alpha, beta, -1)
}

/// Performs a depth limited search looking to evaluate only quiet positions,
/// i.e. those with no attack moves.
fn search_impl(root: &mut Evaluator, mut alpha: i32, beta: i32, depth: i32) -> Result<i32> {
    match root.board().terminal_state() {
        Some(TerminalState::Loss) => return Ok(eval::LOSS_VALUE),
        Some(TerminalState::Draw) => return Ok(eval::DRAW_VALUE),
        _ => {}
    }
    // If we aren't in check then we can use the static eval as the initial
    // result under the sound assumption that there exists a move
    // (which might not be considered here) we can make in the position
    // which will improve our score. We cannot make this assumption if we
    // are in check because we will consider all the moves and so we
    // assume lost until proven otherwise.
    let in_check = root.board().in_check();
    let mut result = if in_check { -eval::INFTY } else { root.relative_eval() };

    // Break immediately if the stand pat is greater than beta.
    if result >= beta {
        return Ok(beta);
    }
    if alpha < result {
        alpha = result;
    }

    for (category, evolve) in compute_quiescent_moves(root, depth) {
        match category {
            MoveCategory::Special => {}
            MoveCategory::Other => {}
            MoveCategory::BadExchange { .. } => {
                if !in_check {
                    continue
                }
            }
            MoveCategory::GoodExchange { optimistic_delta, .. } => {
                if !in_check &&
                    depth < Q_CHECK_CAP &&
                    result + optimistic_delta + DELTA_SKIP_MARGIN < alpha {
                    continue
                }
            }
        };
        root.make(evolve)?;
        let next_result = -search_impl(root, -beta, -alpha, depth - 1)?;
        root.unmake()?;
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return Ok(beta);
        }
    }
    return Ok(result);
}

fn compute_quiescent_moves(state: &mut Evaluator, depth: i32) -> Vec<(MoveCategory, Move)> {
    let mut moves = state
        .board()
        .compute_moves(if depth < Q_CHECK_CAP { Attacks } else { AttacksChecks })
        .into_iter()
        .map(|mv| (categorise(state, &mv), mv))
        .collect_vec();

    moves.sort_unstable_by_key(|(category, _)| -category.score());
    moves
}

fn categorise(state: &mut Evaluator, mv: &Move) -> MoveCategory {
    match mv {
        Enpassant { .. } | Promotion { .. } | Castle { .. } => MoveCategory::Special,
        Standard { from, dest, capture, .. } => {
            match capture {
                None => MoveCategory::Other,
                Some(Piece(_, class)) => {
                    let see = state.see(*from, *dest);
                    if see <= 0 {
                        MoveCategory::BadExchange { see }
                    } else {
                        MoveCategory::GoodExchange {
                            see,
                            optimistic_delta: state.piece_values()[*class]
                        }
                    }
                }
            }
        }
    }
}

enum MoveCategory {
    BadExchange { see: i32 },
    Special,
    Other,
    GoodExchange { see: i32, optimistic_delta: i32 },
}

impl MoveCategory {
    fn score(&self) -> i32 {
        match self {
            MoveCategory::BadExchange { see } => *see,
            MoveCategory::Special => 10000,
            MoveCategory::Other => 5000,
            MoveCategory::GoodExchange { see, .. } => 20000 + see
        }
    }
}
