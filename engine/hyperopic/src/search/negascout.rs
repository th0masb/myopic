use anyhow::{anyhow, Result};
use core::cmp;
use std::time::Instant;

use TreeNode::{All, Cut, Pv};

use crate::board::board_moves;
use crate::constants::{class, create_piece, in_board};
use crate::moves::Move;
use crate::node;
use crate::node::SearchNode;
use crate::position::{TerminalState, CASTLING_DETAILS};
use crate::search::end::SearchEnd;
use crate::search::moves::{MoveGenerator, SearchMove};
use crate::search::pv::PrincipleVariation;
use crate::search::quiescent;
use crate::search::transpositions::{Transpositions, TreeNode};

/// Provides relevant callstack information for the search to
/// use during the traversal of the tree.
pub struct Context {
    pub start: Instant,
    pub alpha: i32,
    pub beta: i32,
    pub depth: u8,
    pub precursors: Vec<Move>,
    pub known_pv_node: bool,
}

impl Context {
    fn next_level(&self, next_alpha: i32, next_beta: i32, mv: &Move, r: u8) -> Context {
        let mut next_precursors = self.precursors.clone();
        next_precursors.push(mv.clone());
        Context {
            start: self.start,
            alpha: next_alpha,
            beta: next_beta,
            depth: self.depth - cmp::min(r, self.depth),
            precursors: next_precursors,
            known_pv_node: false,
        }
    }
}

#[derive(Default)]
pub struct SearchResponse {
    /// The evaluation of the position negamax was called for
    pub eval: i32,
    /// The path of optimal play which led to the eval
    pub path: Vec<Move>,
}

impl std::ops::Neg for SearchResponse {
    type Output = SearchResponse;
    fn neg(self) -> Self::Output {
        SearchResponse { eval: -self.eval, path: self.path }
    }
}

pub struct Scout<'a, E: SearchEnd, T: Transpositions> {
    pub end: &'a E,
    pub transpositions: &'a mut T,
    pub moves: MoveGenerator,
    pub pv: &'a PrincipleVariation,
}

fn reposition_first(dest: &mut Vec<SearchMove>, new_first: &Move) {
    if let Some(index) = dest.iter().position(|m| &m.m == new_first) {
        let removed = dest.remove(index);
        dest.insert(0, removed);
    }
}

impl<E: SearchEnd, T: Transpositions> Scout<'_, E, T> {
    pub fn search(&mut self, node: &mut SearchNode, mut ctx: Context) -> Result<SearchResponse> {
        if self.end.should_end(&ctx) {
            return Err(anyhow!("Terminated at depth {}", ctx.depth));
        }
        let terminal_state = node.position().compute_terminal_state();
        if ctx.depth == 0 || terminal_state.is_some() {
            return match terminal_state {
                Some(TerminalState::Loss) => Ok(node::LOSS_VALUE),
                Some(TerminalState::Draw) => Ok(node::DRAW_VALUE),
                None => quiescent::search(node, ctx.alpha, ctx.beta),
            }
            .map(|eval| SearchResponse { eval, path: vec![] });
        }

        let (key, mut table_move) = (node.position().key, None);
        // If we are in a repeated position then do not break early using table lookup as we can
        // enter a repeated cycle.
        let is_repeated_position = node
            .position()
            .history
            .iter()
            .rev()
            .take_while(|(_, m)| m.is_repeatable())
            .any(|(d, _)| d.key == key);

        let table_entry = self.transpositions.get(node.position()).cloned();
        match &table_entry {
            None => {}
            Some(Pv { depth, eval, best_path: optimal_path, .. }) => {
                table_move = optimal_path.first().cloned();
                if !is_repeated_position
                    && *depth >= ctx.depth
                    && table_move.is_some()
                    && is_pseudo_legal(node, table_move.as_ref().unwrap())
                {
                    return Ok(SearchResponse { eval: *eval, path: optimal_path.clone() });
                }
            }
            Some(Cut { depth, beta, cutoff_move, .. }) => {
                table_move = Some(cutoff_move.clone());
                if !is_repeated_position
                    && *depth >= ctx.depth
                    && ctx.beta <= *beta
                    && is_pseudo_legal(node, &cutoff_move)
                {
                    return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
                }
            }
            Some(All { depth, eval, best_move, .. }) => {
                table_move = Some(best_move.clone());
                if !is_repeated_position
                    && *depth >= ctx.depth
                    && *eval <= ctx.alpha
                    && is_pseudo_legal(node, &best_move)
                {
                    return Ok(SearchResponse { eval: *eval, path: vec![] });
                }
            }
        };

        if should_try_null_move_pruning(node, &ctx) {
            node.make(Move::Null)?;
            let null_search =
                -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &Move::Null, 3))?;
            node.unmake()?;
            if null_search.eval > ctx.beta {
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }
        }

        let mut mvs = self.moves.generate(node);
        table_move.map(|m| reposition_first(&mut mvs, &m));
        let precursors = ctx.precursors.as_slice();
        // Principle variation search
        let in_pvs = self.pv.in_pv(precursors);
        let next_pv = if in_pvs { self.pv.get_next_move(precursors) } else { None };
        next_pv.map(|m| reposition_first(&mut mvs, &m));
        let in_check = node.position().in_check();
        let is_pv_node = ctx.known_pv_node || matches!(table_entry, Some(TreeNode::Pv { .. }));
        let start_alpha = ctx.alpha;

        let mut i = 0;
        let mut non_tactical_i = 0;
        let mut research = false;
        let mut best_path = vec![];
        let mut raised_alpha = false;
        let mut did_reduced_search = false;

        while i < mvs.len() {
            let sm = &mvs[i];
            let m = &sm.m;

            let mut r = 1;
            if !raised_alpha
                && !research
                && ctx.depth > 2
                && !in_check
                && !in_pvs
                && !is_pv_node
                && !sm.is_tactical()
            {
                match non_tactical_i {
                    0 => {}
                    1..=6 => r += 1,
                    _ => r += ctx.depth / 3,
                }
            }

            node.make(m.clone())?;
            #[allow(unused_assignments)]
            let mut response = SearchResponse::default();
            if i == 0 {
                // Perform a full search immediately on the first move which
                // we expect to be the best
                response = -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &m, r))?;
            } else {
                // Search with null window under the assumption that the
                // previous moves are better than this
                response = -self.search(node, ctx.next_level(-ctx.alpha - 1, -ctx.alpha, &m, r))?;
                // If there is some move which can raise alpha
                if ctx.alpha < response.eval && response.eval < ctx.beta {
                    // Then this was actually a better move and so we must
                    // perform a full search
                    response = -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &m, r))?;
                }
            }
            node.unmake()?;

            if ctx.alpha < response.eval {
                // If we found an alpha increase at reduced depth research move at full depth to double check
                if r > 1 {
                    research = true;
                    continue;
                }
                raised_alpha = true;
                ctx.alpha = response.eval;
                best_path = response.path;
                best_path.insert(0, m.clone());
            }

            if ctx.alpha >= ctx.beta {
                self.transpositions.put(
                    node.position(),
                    Cut { depth: ctx.depth, beta: ctx.beta, cutoff_move: m.clone(), hash: key },
                );
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }

            i += 1;
            research = false;
            if !sm.is_tactical() {
                non_tactical_i += 1;
            }
            if r > 1 {
                did_reduced_search = true;
            }
            // If this is the case we are in a PV node and so need to research everything at full
            // depth, so don't continue this search any longer
            if did_reduced_search && raised_alpha {
                break;
            }
        }

        // In this case we thought we weren't in a PV node but we actually were, do a full research
        // of the node
        if did_reduced_search && raised_alpha {
            ctx.alpha = start_alpha;
            ctx.known_pv_node = true;
            return self.search(node, ctx);
        }

        // Populate the table with the information from this node.
        if ctx.alpha == start_alpha {
            if let Some(m) = best_path.first() {
                self.transpositions.put(
                    node.position(),
                    All { depth: ctx.depth, eval: ctx.alpha, best_move: m.clone(), hash: key },
                );
            }
        } else {
            self.transpositions.put(
                node.position(),
                Pv { depth: ctx.depth, eval: ctx.alpha, best_path: best_path.clone(), hash: key },
            );
        }

        Ok(SearchResponse { eval: ctx.alpha, path: best_path })
    }
}

fn is_pseudo_legal(node: &SearchNode, m: &Move) -> bool {
    let position = node.position();
    match m {
        Move::Null => false,
        Move::Enpassant { capture, .. } => position.enpassant == Some(*capture),
        &Move::Castle { corner } => {
            position.castling_rights[corner] && {
                let details = &CASTLING_DETAILS[corner];
                let rook = create_piece(position.active, class::R);
                let king = create_piece(position.active, class::K);
                position.piece_locs[details.rook_line.0] == Some(rook)
                    && position.piece_locs[details.king_line.0] == Some(king)
            }
        }
        &Move::Normal { moving, from, dest, capture } => {
            let (friendly, enemy) = position.friendly_enemy_boards();
            position.piece_locs[from] == Some(moving)
                && position.piece_locs[dest] == capture
                && in_board(board_moves(moving, from, friendly, enemy), dest)
        }
        &Move::Promote { from, dest, capture, .. } => {
            position.piece_locs[from] == Some(create_piece(position.active, class::P))
                && position.piece_locs[dest] == capture
        }
    }
}

fn should_try_null_move_pruning(node: &SearchNode, ctx: &Context) -> bool {
    let position = node.position();
    ctx.depth < 5 && ctx.beta < 1000 && !position.in_check() && {
        let active = position.active;
        let pawns = position.piece_boards[create_piece(active, class::P)];
        let others = position.side_boards[active] & !pawns;
        pawns.count_ones() > 2 && others.count_ones() > 1 || others.count_ones() > 2
    }
}
