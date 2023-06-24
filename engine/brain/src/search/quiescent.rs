use std::cmp;
use MoveComputeType::{Attacks, AttacksChecks};

use myopic_board::anyhow::Result;
use myopic_board::{Move, MoveComputeType, TerminalState};

use crate::{eval, Evaluator};

const Q_CHECK_CAP: i32 = -1;

pub fn full_search(root: &mut Evaluator) -> Result<i32> {
    search(root, -eval::INFTY, eval::INFTY)
}

pub fn search(root: &mut Evaluator, alpha: i32, beta: i32) -> Result<i32> {
    search_impl(root, alpha, beta, -1)
}

/// Performs a depth limited search looking to evaluate only quiet positions,
/// i.e. those with no attack moves.
fn search_impl(root: &mut Evaluator, mut alpha: i32, beta: i32, depth: i32) -> Result<i32> {
    if root.board().terminal_state().is_some() {
        return Ok(match root.board().terminal_state() {
            Some(TerminalState::Loss) => eval::LOSS_VALUE,
            Some(TerminalState::Draw) => eval::DRAW_VALUE,
            None => root.relative_eval(),
        });
    }
    // If we aren't in check then we can use the static eval as the initial
    // result under the sound assumption that there exists a move
    // (which might not be considered here) we can make in the position
    // which will improve our score. We cannot make this assumption if we
    // are in check because we will consider all the moves and so we
    // assume lost until proven otherwise.
    let mut result = if root.board().in_check() { -eval::INFTY } else { root.relative_eval() };

    // Break immediately if the stand pat is greater than beta.
    if result >= beta {
        return Ok(beta);
    }
    if alpha < result {
        alpha = result;
    }

    for evolve in compute_quiescent_moves(root, depth) {
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

fn compute_quiescent_moves(state: &mut Evaluator, depth: i32) -> Vec<Move> {
    let moves_type = if depth < Q_CHECK_CAP { Attacks } else { AttacksChecks };
    // If in check don't filter out any attacks, we must check all available moves.
    let good_attack_threshold = if state.board().in_check() { -eval::INFTY } else { 0 };

    let mut moves = state
        .board()
        .compute_moves(moves_type)
        .into_iter()
        .map(|mv| (score(state, &mv), mv))
        .filter(|(s, _)| *s > good_attack_threshold)
        .collect::<Vec<_>>();

    moves.sort_unstable_by_key(|(score, _)| -*score);
    moves.into_iter().map(|(_, m)| m).collect()
}

fn score(state: &mut Evaluator, mv: &Move) -> i32 {
    if !is_attack(mv) {
        30000
    } else {
        match mv {
            &Move::Enpassant { .. } => 10000,
            &Move::Promotion { .. } => 20000,
            &Move::Standard { from, dest, .. } => state.see(from, dest),
            // Should never get here
            _ => 0,
        }
    }
}

fn is_attack(query: &Move) -> bool {
    match query {
        &Move::Enpassant { .. } => true,
        &Move::Castle { .. } => false,
        &Move::Promotion { capture, .. } => capture.is_some(),
        &Move::Standard { capture, .. } => capture.is_some(),
    }
}
