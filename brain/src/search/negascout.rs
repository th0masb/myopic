use crate::search::eval;
use crate::search::ordering::{EstimatorImpl, MoveQualityEstimator};
use crate::search::ordering_hints::OrderingHints;
use crate::search::terminator::SearchTerminator;
use crate::{quiescent, EvalBoard};
use anyhow::{anyhow, Result};
use core::cmp;
use itertools::Itertools;
use myopic_board::{Move, MoveComputeType, Termination};
use serde::export::PhantomData;
use std::time::Instant;

/// Performs a negascout search without any iterative deepening,
/// we simply provide a depth to search to. The depth should be
/// kept low otherwise ID is always preferable. In particular
/// this function will support a depth 0 search which performs
/// a quiescent search on the provided root.
pub fn search<B>(root: &mut B, depth: usize) -> Result<SearchResponse>
where
    B: EvalBoard,
{
    Scout {
        terminator: &depth,
        ordering_hints: &OrderingHints::new(root.clone()),
        move_quality_estimator: EstimatorImpl,
        board_type: PhantomData,
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
        SearchResponse {
            eval: -self.eval,
            path: self.path,
        }
    }
}

impl Default for SearchResponse {
    fn default() -> Self {
        SearchResponse {
            eval: 0,
            path: vec![],
        }
    }
}

pub struct Scout<'a, T, B, M>
where
    T: SearchTerminator,
    B: EvalBoard,
    M: MoveQualityEstimator<B>,
{
    /// The terminator is responsible for deciding when the
    /// search is complete
    pub terminator: &'a T,
    /// Precomputed hints for helping to order moves
    /// generated for positions in the search tree
    pub ordering_hints: &'a OrderingHints<B>,
    /// Used for performing an initial sort on the moves
    /// generated in each position for optimising the search
    pub move_quality_estimator: M,
    /// Placeholder to satisfy the compiler because of the 'unused'
    /// type parameter for the board
    pub board_type: std::marker::PhantomData<B>,
}

impl<T, B, M> Scout<'_, T, B, M>
where
    T: SearchTerminator,
    B: EvalBoard,
    M: MoveQualityEstimator<B>,
{
    ///
    pub fn search(&self, root: &mut B, mut ctx: SearchContext) -> Result<SearchResponse> {
        if self.terminator.should_terminate(&ctx) {
            Err(anyhow!("Terminated at depth {}", ctx.depth_remaining))
        } else if ctx.depth_remaining == 0 || root.termination_status().is_some() {
            match root.termination_status() {
                Some(Termination::Loss) => Ok(eval::LOSS_VALUE),
                Some(Termination::Draw) => Ok(eval::DRAW_VALUE),
                None => quiescent::search(root, -eval::INFTY, eval::INFTY, -1),
            }
            .map(|eval| SearchResponse { eval, path: vec![] })
        } else {
            let (mut result, mut best_path) = (-eval::INFTY, vec![]);
            for (i, evolve) in self
                .compute_moves(root, &ctx.precursors)
                .into_iter()
                .enumerate()
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
                        response = -self
                            .search(root, ctx.next_level(-ctx.beta, -response.eval, &evolve))?;
                    }
                }
                root.unmake()?;

                if response.eval > result {
                    result = response.eval;
                    best_path = response.path;
                    best_path.push(evolve);
                }

                ctx.alpha = cmp::max(ctx.alpha, result);
                if ctx.alpha >= ctx.beta {
                    return Ok(SearchResponse {
                        eval: ctx.beta,
                        path: vec![],
                    });
                }
            }
            Ok(SearchResponse {
                eval: result,
                path: best_path,
            })
        }
    }

    fn compute_heuristically_ordered_moves(&self, board: &mut B) -> Vec<Move> {
        let mut moves = board.compute_moves(MoveComputeType::All);
        moves.sort_by_cached_key(|m| -self.move_quality_estimator.estimate(board, m));
        moves
    }

    fn compute_moves(&self, board: &mut B, precursors: &Vec<Move>) -> Vec<Move> {
        let sm = self.ordering_hints;
        match (sm.get_pvs(precursors), sm.get_evs(precursors)) {
            (None, None) => self.compute_heuristically_ordered_moves(board),
            (Some(pvs), None) => pvs
                .into_iter()
                .map(|pv| pv.mv.clone())
                .chain(self.compute_heuristically_ordered_moves(board))
                .dedup()
                .collect(),
            (None, Some(evs)) => evs.into_iter().map(|ev| ev.mv.clone()).collect(),
            (Some(pvs), Some(evs)) => pvs
                .into_iter()
                .map(|pv| pv.mv.clone())
                .chain(evs.into_iter().map(|ev| ev.mv.clone()))
                .dedup()
                .collect(),
        }
    }
}
