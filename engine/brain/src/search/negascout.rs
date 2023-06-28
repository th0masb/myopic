use core::cmp;
use std::time::Instant;

use myopic_board::anyhow::{anyhow, Result};
use myopic_board::{Move, TerminalState};

use crate::search::moves::MoveGenerator;
use crate::search::terminator::SearchTerminator;
use crate::search::transpositions::{Transpositions, TreeNode};
use crate::search::{eval, quiescent};
use crate::Evaluator;

/// Provides relevant callstack information for the search to
/// use during the traversal of the tree.
pub struct SearchContext {
    pub start_time: Instant,
    pub alpha: i32,
    pub beta: i32,
    pub depth_remaining: usize,
    pub precursors: Vec<Move>,
}

impl SearchContext {
    fn next_level(&self, next_alpha: i32, next_beta: i32, mv: &Move) -> SearchContext {
        let mut next_precursors = self.precursors.clone();
        next_precursors.push(mv.clone());
        SearchContext {
            start_time: self.start_time,
            alpha: next_alpha,
            beta: next_beta,
            depth_remaining: self.depth_remaining - 1,
            precursors: next_precursors,
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

pub struct Scout<'a, T: SearchTerminator> {
    /// The terminator is responsible for deciding when the search is complete
    pub terminator: &'a T,
    /// Transposition table containing previously computed information about nodes in the tree.
    pub transpositions: &'a mut Transpositions,
    /// Move generator for nodes in the tree
    pub moves: MoveGenerator<'a>,
}

impl<T: SearchTerminator> Scout<'_, T> {
    pub fn search(
        &mut self,
        root: &mut Evaluator,
        mut ctx: SearchContext,
    ) -> Result<SearchResponse> {
        if self.terminator.should_terminate(&ctx) {
            return Err(anyhow!("Terminated at depth {}", ctx.depth_remaining));
        } else if ctx.depth_remaining == 0 || root.board().terminal_state().is_some() {
            return match root.board().terminal_state() {
                Some(TerminalState::Loss) => Ok(eval::LOSS_VALUE),
                Some(TerminalState::Draw) => Ok(eval::DRAW_VALUE),
                None => quiescent::search(root, ctx.alpha, ctx.beta),
            }
            .map(|eval| SearchResponse { eval, path: vec![] });
        }

        let (hash, mut table_move) = (root.board().hash(), None);
        match self.transpositions.get(hash) {
            None => {}
            Some(TreeNode::Pv { depth, eval, optimal_path, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining {
                    // We already searched this position fully at a sufficient depth
                    return Ok(SearchResponse { eval: *eval, path: optimal_path.clone() });
                } else {
                    // The depth wasn't sufficient and so we only have a suggestion
                    // for the best move
                    table_move = optimal_path.last()
                }
            }
            Some(TreeNode::Cut { depth, beta, cutoff_move, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining && ctx.beta <= *beta {
                    return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
                } else {
                    table_move = Some(cutoff_move);
                }
            }
            Some(TreeNode::All { depth, eval, best_move, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining && *eval <= ctx.alpha {
                    return Ok(SearchResponse { eval: *eval, path: vec![] });
                } else {
                    table_move = Some(best_move);
                }
            }
        };

        let (start_alpha, mut result, mut best_path) = (ctx.alpha, -eval::INFTY, vec![]);
        for (i, m) in self.moves.generate(root, &ctx, table_move).enumerate() {
            root.make(m.clone())?;
            #[allow(unused_assignments)]
            let mut response = SearchResponse::default();
            if i == 0 {
                // Perform a full search immediately on the first move which
                // we expect to be the best
                response = -self.search(root, ctx.next_level(-ctx.beta, -ctx.alpha, &m))?;
            } else {
                // Search with null window under the assumption that the
                // previous moves are better than this
                response = -self.search(root, ctx.next_level(-ctx.alpha - 1, -ctx.alpha, &m))?;
                // If there is some move which can raise alpha
                if ctx.alpha < response.eval && response.eval < ctx.beta {
                    // Then this was actually a better move and so we must
                    // perform a full search
                    response = -self.search(root, ctx.next_level(-ctx.beta, -ctx.alpha, &m))?;
                }
            }
            root.unmake()?;

            if response.eval > result {
                result = response.eval;
                best_path = response.path;
                best_path.insert(0, m.clone());
            }

            ctx.alpha = cmp::max(ctx.alpha, result);
            if ctx.alpha >= ctx.beta {
                // We are a cut node
                self.transpositions.insert(
                    hash,
                    TreeNode::Cut {
                        depth: ctx.depth_remaining as u8,
                        beta: ctx.beta,
                        cutoff_move: m,
                        hash,
                    },
                );
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }
        }

        // Populate the table with the information from this node.
        if ctx.alpha == start_alpha {
            // We are an all node
            if let Some(m) = best_path.last() {
                self.transpositions.insert(
                    hash,
                    TreeNode::All {
                        depth: ctx.depth_remaining as u8,
                        eval: result,
                        best_move: m.clone(),
                        hash,
                    },
                )
            }
        } else {
            // We are a pv node
            self.transpositions.insert(
                hash,
                TreeNode::Pv {
                    depth: ctx.depth_remaining as u8,
                    eval: result,
                    optimal_path: best_path.clone(),
                    hash,
                },
            )
        }

        Ok(SearchResponse { eval: result, path: best_path })
    }
}
