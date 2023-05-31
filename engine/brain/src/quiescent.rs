use std::cmp;
use MoveComputeType::{Attacks, AttacksChecks};

use myopic_board::anyhow::Result;
use myopic_board::{Move, MoveComputeType, TerminalState};

use crate::eval::EvalChessBoard;
use crate::{eval, see};

const Q_DEPTH_CAP: i32 = -8;
const Q_CHECK_CAP: i32 = -2;

// TODO Do we want the quiescent search to have interruption finishing checks too?

/// Performs a depth limited search looking to evaluate only quiet positions,
/// i.e. those with no attack moves.
pub fn search<B: EvalChessBoard>(
    state: &mut B,
    mut alpha: i32,
    beta: i32,
    depth: i32,
) -> Result<i32> {
    if depth == Q_DEPTH_CAP || state.terminal_state().is_some() {
        return Ok(match state.terminal_state() {
            Some(TerminalState::Loss) => eval::LOSS_VALUE,
            Some(TerminalState::Draw) => eval::DRAW_VALUE,
            None => state.relative_eval(),
        });
    }
    // If we aren't in check then we can use the static eval as the initial
    // result under the sound assumption that there exists a move
    // (which might not be considered here) we can make in the position
    // which will improve our score. We cannot make this assumption if we
    // are in check because we will consider all the moves and so we
    // assume lost until proven otherwise.
    let mut result = if state.in_check() {
        -eval::INFTY
    } else {
        state.relative_eval()
    };

    // Break immediately if the stand pat is greater than beta.
    if result >= beta {
        return Ok(beta);
    }
    if alpha < result {
        alpha = result;
    }

    for evolve in compute_quiescent_moves(state, depth) {
        state.make(evolve)?;
        let next_result = -search(state, -beta, -alpha, depth - 1)?;
        state.unmake()?;
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return Ok(beta);
        }
    }
    return Ok(result);
}

fn compute_quiescent_moves<B: EvalChessBoard>(state: &mut B, depth: i32) -> Vec<Move> {
    let moves_type = if depth < Q_CHECK_CAP { Attacks } else { AttacksChecks };
    // If in check don't filter out any attacks, we must check all available moves.
    let good_attack_threshold = if state.in_check() { -eval::INFTY } else { 0 };

    let mut moves = state.compute_moves(moves_type)
        .into_iter()
        .map(|mv| (score(state, &mv), mv))
        .filter(|(s, _)| *s > good_attack_threshold)
        .collect::<Vec<_>>();

    moves.sort_unstable_by_key(|(score, _)| -*score);
    moves.into_iter().map(|(_, m)| m).collect()
}

fn score<B: EvalChessBoard>(state: &mut B, mv: &Move) -> i32 {
    if !is_attack(mv) {
        30000
    } else {
        match mv {
            &Move::Enpassant { .. } => 10000,
            &Move::Promotion { .. } => 20000,
            &Move::Standard { from, dest, .. } => {
                see::exchange_value(state, from, dest, state.piece_values())
            }
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
