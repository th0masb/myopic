use std::cmp;

use crate::itertools;
use crate::base::Reflectable;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::EvalBoard;
use crate::base::bitboard::BitBoard;
use crate::board::Board;

const Q_DEPTH_CAP: i32 = -20;
const Q_CHECK_CAP: i32 = -7;

fn negamax<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 || state.termination_status().is_some() {
        match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            None => -quiescent(state, -beta, -alpha, -1),
        }
    } else {
        let mut best_result = -eval::INFTY;
        for evolve in state.compute_moves(MoveComputeType::All) {
            let discards = state.evolve(&evolve);
            let next_result = -negamax(state, -beta, -alpha, depth - 1);
            state.devolve(&evolve, discards);
            best_result = cmp::max(best_result, next_result);
            alpha = cmp::max(alpha, best_result);
            if alpha > beta {
                return beta;
            }
        }
        return best_result;
    }
}

fn quiescent<B: EvalBoard>(state: &mut B, mut alpha: i32, mut beta: i32, depth: i32) -> i32 {
    if depth == Q_DEPTH_CAP || state.termination_status().is_some() {
        return match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            None => state.static_eval(),
        };
    }
    let moves = compute_quiescent_moves(state, depth);
    if moves.is_empty() {
        return state.static_eval();
    } else {
        let mut best_result = -eval::INFTY;
        for evolve in moves {
            let discards = state.evolve(&evolve);
            let next_result = -quiescent(state, -beta, -alpha, depth - 1);
            state.devolve(&evolve, discards);
            best_result = cmp::max(best_result, next_result);
            alpha = cmp::max(alpha, best_result);
            if alpha > beta {
                return beta;
            }
        }
        return best_result;
    }
}

fn compute_quiescent_moves<B: Board>(state: &mut B, depth: i32) -> Vec<Move> {
    let mut moves = if depth > Q_CHECK_CAP {
        state.compute_moves(MoveComputeType::AttacksChecks)
    } else {
        state.compute_moves(MoveComputeType::Attacks)
    };
    let enemies = state.side(state.active().reflect());
    let attack_filter = |mv: &Move| is_attack(mv, enemies);
    let split_index = itertools::partition(&mut moves, attack_filter);
    let mut attacks: Vec<_> = moves.iter().take(split_index).cloned().collect();
    let mut others:  Vec<_> = moves.iter().skip(split_index).cloned().collect();
    // Then score attacks using see and filter bad exchanges before sorting and
    // recombining.


    unimplemented!()
}

fn score_attack<B: Board>(state: &mut B, attack: &Move) -> i32 {
    unimplemented!()
}

fn is_attack(query: &Move, enemies: BitBoard) -> bool {
    match query {
        &Move::Enpassant(src) => true,
        &Move::Castle(zone) => false,
        &Move::Promotion(_, target, _) => enemies.contains(target),
        &Move::Standard(_, _, target) => enemies.contains(target),
    }
}
