use core::cmp;
use std::time::Instant;

use myopic_board::{Move, MoveComputeType, TerminalState};
use myopic_board::anyhow::{anyhow, Result};

use crate::Evaluator;
use crate::search::{eval, quiescent};
use crate::search::movequality::{BestMoveHeuristic, MaterialAndPositioningHeuristic};
use crate::search::pv::PrincipleVariation;
use crate::search::terminator::SearchTerminator;
use crate::search::transpositions::{Transpositions, TreeNode};

/// Performs a negascout search without any iterative deepening,
/// we simply provide a depth to search to. The depth should be
/// kept low otherwise ID is always preferable. In particular
/// this function will support a depth 0 search which performs
/// a quiescent search on the provided root.
pub fn search(root: &mut Evaluator, depth: usize) -> Result<SearchResponse> {
    Scout {
        terminator: &depth,
        pv: &PrincipleVariation::default(),
        transpositions: &mut Transpositions::new(1),
        move_quality_estimator: MaterialAndPositioningHeuristic::default(),
    }
    .search(
        root,
        SearchContext {
            start_time: Instant::now(),
            alpha: -eval::INFTY,
            beta: eval::INFTY,
            depth_remaining: depth,
            precursors: vec![],
        },
    )
}

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

///
pub struct SearchResponse {
    /// The evaluation of the position negamax was called for
    pub eval: i32,
    /// The path of optimal play which led to the eval if the
    /// depth was greater than zero.
    pub path: Vec<Move>,
}

impl std::ops::Neg for SearchResponse {
    type Output = SearchResponse;

    fn neg(self) -> Self::Output {
        SearchResponse { eval: -self.eval, path: self.path }
    }
}

impl Default for SearchResponse {
    fn default() -> Self {
        SearchResponse { eval: 0, path: vec![] }
    }
}

pub struct Scout<'a, T, M>
where
    T: SearchTerminator,
    M: BestMoveHeuristic,
{
    /// The terminator is responsible for deciding when the
    /// search is complete
    pub terminator: &'a T,
    /// Reference to the current principle search variation
    pub pv: &'a PrincipleVariation,
    /// Cache of search information for all nodes in
    /// the tree which is shared across searches
    /// during an iterative deepening run. It can be
    /// thought of as transient information to give
    /// further hints for ordering and to skip searches
    /// if we already have sufficient information for
    /// that part of the tree.
    pub transpositions: &'a mut Transpositions,
    /// Used for performing an initial sort on the moves
    /// generated in each position for optimising the search
    pub move_quality_estimator: M,
}

enum TableSuggestion {
    Pv(u8, Move),
    Cut(Move),
    All(Move),
}

impl TableSuggestion {
    pub fn mv(&self) -> &Move {
        match self {
            TableSuggestion::Pv(_, mv) => mv,
            TableSuggestion::Cut(mv) => mv,
            TableSuggestion::All(mv) => mv,
        }
    }
}

impl<T, M> Scout<'_, T, M>
where
    T: SearchTerminator,
    M: BestMoveHeuristic,
{
    pub fn search(
        &mut self,
        root: &mut Evaluator,
        mut ctx: SearchContext,
    ) -> Result<SearchResponse> {
        if self.terminator.should_terminate(&ctx) {
            return Err(anyhow!("Terminated at depth {}", ctx.depth_remaining))
        } else if ctx.depth_remaining == 0 || root.board().terminal_state().is_some() {
            return match root.board().terminal_state() {
                Some(TerminalState::Loss) => Ok(eval::LOSS_VALUE),
                Some(TerminalState::Draw) => Ok(eval::DRAW_VALUE),
                None => quiescent::search(root, ctx.alpha, ctx.beta),
            }.map(|eval| SearchResponse { eval, path: vec![] })
        }

        let (hash, mut table_suggestion) = (root.board().hash(), None);
        match self.transpositions.get(hash) {
            None => {}
            Some(TreeNode::Pv { depth, eval, optimal_path, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining {
                    // We already searched this position fully at a sufficient depth
                    return Ok(SearchResponse { eval: *eval, path: optimal_path.clone() });
                } else {
                    // The depth wasn't sufficient and so we only have a suggestion
                    // for the best move
                    table_suggestion =
                        optimal_path.last().map(|m| TableSuggestion::Pv(*depth, m.clone()))
                }
            }
            Some(TreeNode::Cut { depth, beta, cutoff_move, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining && ctx.beta <= *beta {
                    return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
                } else {
                    table_suggestion = Some(TableSuggestion::Cut(cutoff_move.clone()));
                }
            }
            Some(TreeNode::All { depth, eval, best_move, .. }) => {
                if (*depth as usize) >= ctx.depth_remaining && *eval <= ctx.alpha {
                    return Ok(SearchResponse { eval: *eval, path: vec![] });
                } else {
                    table_suggestion = Some(TableSuggestion::All(best_move.clone()));
                }
            }
        };

        let (start_alpha, mut result, mut best_path) = (ctx.alpha, -eval::INFTY, vec![]);
        for (i, evolve) in
        self.compute_moves(root, &ctx.precursors, table_suggestion).into_iter().rev().enumerate()
        {
            root.make(evolve.clone())?;
            #[allow(unused_assignments)]
                let mut response = SearchResponse::default();
            if i == 0 {
                // Perform a full search immediately on the first move which
                // we expect to be the best
                response =
                    -self.search(root, ctx.next_level(-ctx.beta, -ctx.alpha, &evolve))?;
            } else {
                // Search with null window under the assumption that the
                // previous moves are better than this
                response =
                    -self.search(root, ctx.next_level(-ctx.alpha - 1, -ctx.alpha, &evolve))?;
                // If there is some move which can raise alpha
                if ctx.alpha < response.eval && response.eval < ctx.beta {
                    // Then this was actually a better move and so we must
                    // perform a full search
                    response =
                        -self.search(root, ctx.next_level(-ctx.beta, -ctx.alpha, &evolve))?;
                }
            }
            root.unmake()?;

            if response.eval > result {
                result = response.eval;
                best_path = response.path;
                best_path.insert(0, evolve.clone());
            }

            ctx.alpha = cmp::max(ctx.alpha, result);
            if ctx.alpha >= ctx.beta {
                // We are a cut node
                self.transpositions.insert(
                    hash,
                    TreeNode::Cut {
                        depth: ctx.depth_remaining as u8,
                        beta: ctx.beta,
                        cutoff_move: evolve,
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

    fn compute_moves(
        &self,
        node: &mut Evaluator,
        precursors: &Vec<Move>,
        table_suggestion: Option<TableSuggestion>,
    ) -> Vec<Move> {
        let mut moves = node.board().compute_moves(MoveComputeType::All);
        moves.sort_by_cached_key(|m| self.move_quality_estimator.estimate(node, m));
        table_suggestion.map(|t| reposition_last(&mut moves, t.mv()));
        if let Some(pv) = self.pv.get_next_move(precursors.as_slice()) {
            reposition_last(&mut moves, &pv);
        }
        moves
    }
}

fn reposition_last(dest: &mut Vec<Move>, new_first: &Move) {
    if let Some(index) = dest.iter().position(|m| m == new_first) {
        let removed = dest.remove(index);
        dest.push(removed);
    }
}
