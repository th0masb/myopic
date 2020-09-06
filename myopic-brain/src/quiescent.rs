use crate::eval::EvalBoard;
use crate::{eval, see};
use myopic_board::{Move, MoveComputeType, MutBoard, Termination};
use myopic_core::bitboard::BitBoard;
use myopic_core::reflectable::Reflectable;
use std::cmp;

const Q_DEPTH_CAP: i32 = -8;
const Q_CHECK_CAP: i32 = -2;

// TODO Do we want the quiescent search to have interruption finishing checks too?

/// Performs a depth limited search looking to evaluate only quiet positions,
/// i.e. those with no attack moves.
pub(super) fn search<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    if depth == Q_DEPTH_CAP || state.termination_status().is_some() {
        return match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            None => state.static_eval(),
        };
    }
    // If we aren't in check then we can use the static eval as the initial
    // result under the sound assumption that there exists a move
    // (which might not be considered here) we can make in the position
    // which will improve our score. We cannot make this assumption if we
    // are in check because we will consider all the moves and so we
    // assume lost until proven otherwise.
    let mut result = if state.in_check() { -eval::INFTY } else { state.static_eval() };

    // Break immediately if the stand pat is greater than beta.
    if result >= beta {
        return beta;
    }
    if alpha < result {
        alpha = result;
    }

    for evolve in compute_quiescent_moves(state, depth) {
        let discards = state.evolve(&evolve);
        let next_result = -search(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return beta;
        }
    }
    return result;
}

fn compute_quiescent_moves<B: MutBoard>(state: &mut B, depth: i32) -> Vec<Move> {
    let mut moves = if depth < Q_CHECK_CAP {
        state.compute_moves(MoveComputeType::Attacks)
    } else {
        state.compute_moves(MoveComputeType::AttacksChecks)
    };
    let enemies = state.side(state.active().reflect());
    let is_attack_filter = |mv: &Move| is_attack(mv, enemies);
    // If in check don't filter out any attacks, we must check all available moves.
    let good_attack_threshold = if state.in_check() { -eval::INFTY } else { 0 };
    let split_index = itertools::partition(&mut moves, is_attack_filter);
    // Score attacks using see and filter bad exchanges before sorting and
    // recombining.
    let mut attacks: Vec<_> = moves
        .iter()
        .take(split_index)
        .map(|mv| (mv, score_attack(state, mv)))
        .filter(|(_, score)| *score > good_attack_threshold)
        .collect();
    attacks.sort_by_key(|(_, score)| -*score);

    moves
        .iter()
        .cloned()
        .skip(split_index)
        .chain(attacks.into_iter().map(|(mv, _)| mv.clone()))
        .collect()
}

fn score_attack<B: MutBoard>(state: &mut B, attack: &Move) -> i32 {
    match attack {
        &Move::Enpassant(_, _) => 10000,
        &Move::Promotion(_, _, _) => 20000,
        &Move::Standard(_, source, target) => see::exchange_value(state, source, target),
        _ => panic!(),
    }
}

fn is_attack(query: &Move, enemies: BitBoard) -> bool {
    match query {
        &Move::Enpassant(_, _) => true,
        &Move::Castle(_) => false,
        &Move::Promotion(_, target, _) => enemies.contains(target),
        &Move::Standard(_, _, target) => enemies.contains(target),
    }
}
