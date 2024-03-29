use anyhow::{anyhow, Result};
use std::cmp::{max, min};
use std::time::Instant;
use NodeType::{All, Cut, Pv};

use crate::board::board_moves;
use crate::constants::{class, create_piece, in_board};
use crate::moves::Move;
use crate::node;
use crate::node::TreeNode;
use crate::position::{TerminalState, CASTLING_DETAILS};
use crate::search::end::SearchEnd;
use crate::search::moves::{MoveGenerator, SearchMove};
use crate::search::pv::PrincipleVariation;
use crate::search::quiescent;
use crate::search::table::{NodeType, Transpositions};

/// Provides relevant callstack information for the search to
/// use during the traversal of the tree.
pub struct Context {
    pub start: Instant,
    pub root_index: u16,
    pub alpha: i32,
    pub beta: i32,
    pub depth: u8,
    pub precursors: Vec<Move>,
    pub known_raise_alpha: Option<Move>,
}

impl Context {
    fn next(&self, alpha: i32, beta: i32, m: &Move, r: u8) -> Context {
        let mut next_precursors = self.precursors.clone();
        next_precursors.push(m.clone());
        Context {
            start: self.start,
            alpha,
            beta,
            depth: self.depth - min(r, self.depth),
            root_index: self.root_index,
            precursors: next_precursors,
            known_raise_alpha: None,
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

pub struct TreeSearcher<'a, E: SearchEnd, T: Transpositions> {
    pub end: &'a E,
    pub table: &'a mut T,
    pub moves: MoveGenerator,
    pub pv: &'a PrincipleVariation,
}

fn reposition_first(dest: &mut Vec<SearchMove>, new_first: &Move) {
    if let Some(index) = dest.iter().position(|m| &m.m == new_first) {
        let removed = dest.remove(index);
        dest.insert(0, removed);
    }
}

enum TableLookup {
    Miss,
    Suggestion(NodeType),
    Hit(SearchResponse),
}

impl<E: SearchEnd, T: Transpositions> TreeSearcher<'_, E, T> {
    pub fn search(&mut self, node: &mut TreeNode, mut ctx: Context) -> Result<SearchResponse> {
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

        let table_entry = match self.do_table_lookup(node, &ctx) {
            TableLookup::Miss => None,
            TableLookup::Suggestion(n) => Some(n),
            TableLookup::Hit(response) => return Ok(response),
        };

        let in_pvs = self.pv.in_pv(ctx.precursors.as_slice());

        if !in_pvs && should_try_null_move_pruning(node, &ctx) {
            node.make(Move::Null)?;
            let score = -self.search(node, ctx.next(-ctx.beta, -ctx.alpha, &Move::Null, 3))?;
            node.unmake()?;
            if score.eval > ctx.beta {
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }
        }

        let mvs = self.generate_moves(node, &ctx, &table_entry);
        let start_alpha = ctx.alpha;
        let in_check = node.position().in_check();
        let is_pv_node = in_pvs
            || ctx.known_raise_alpha.is_some()
            || matches!(table_entry, Some(NodeType::Pv(_)));

        let mut i = 0;
        let mut research = false;
        let mut best_path = vec![];
        let mut raised_alpha = false;
        let mut score = -node::INFTY;

        while i < mvs.len() {
            let sm = &mvs[i];
            let m = &sm.m;

            // The depth reduction we will search the move with
            let mut r = 1;
            if !research && ctx.depth > 2 && !in_check && !sm.is_tactical() {
                if is_pv_node {
                    if i > 6 {
                        r += 1
                    }
                } else {
                    match i {
                        0 => {}
                        1..=6 => r += 1,
                        _ => r += ctx.depth / 3,
                    }
                }
            }

            node.make(m.clone())?;
            let response = if !raised_alpha {
                -self.search(node, ctx.next(-ctx.beta, -ctx.alpha, &m, r))?
            } else {
                // Search with null window under the assumption that the
                // previous moves are better than this
                let null = -self.search(node, ctx.next(-ctx.alpha - 1, -ctx.alpha, &m, r))?;
                // If there is some move which can raise alpha
                if score < null.eval {
                    // Then this was actually a better move and so we must
                    // perform a full search
                    -self.search(node, ctx.next(-ctx.beta, -ctx.alpha, &m, r))?
                } else {
                    null
                }
            };
            node.unmake()?;

            if score < response.eval {
                // If we found a better score at reduced depth research move at full depth
                if r > 1 {
                    research = true;
                    continue;
                }
                score = response.eval;
                best_path = response.path;
                best_path.insert(0, m.clone());
                if ctx.alpha < score {
                    ctx.alpha = score;
                    raised_alpha = true;
                }
            }

            if ctx.alpha >= ctx.beta {
                self.table.put(
                    node.position(),
                    ctx.root_index,
                    ctx.depth,
                    ctx.beta,
                    Cut(m.clone()),
                );
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }

            i += 1;
            research = false;
            // If this is the case we are in a PV node and so need to research everything at full
            // depth, so don't continue this search any longer
            if !is_pv_node && raised_alpha {
                break;
            }
        }

        // In this case we thought we weren't in a PV node but we actually were, do a full research
        // of the node. We know which moved raised alpha so we can speed things up by starting with
        // that move in the recursive call
        if !is_pv_node && raised_alpha {
            debug_assert!(best_path.len() > 0);
            ctx.alpha = start_alpha;
            ctx.known_raise_alpha = best_path.first().cloned();
            return self.search(node, ctx);
        }

        // Populate the table with the information from this node.
        debug_assert!(best_path.len() > 0);
        self.table.put(
            node.position(),
            ctx.root_index,
            ctx.depth,
            score,
            if raised_alpha {
                Pv(best_path.clone())
            } else {
                All(best_path.first().unwrap().clone())
            },
        );

        Ok(SearchResponse { eval: ctx.alpha, path: best_path })
    }

    fn do_table_lookup(&self, node: &TreeNode, ctx: &Context) -> TableLookup {
        // If we are in a repeated position then do not break early using table lookup as we can
        // enter a repeated cycle.
        if let Some(existing) = self.table.get(node.position()) {
            let is_repeated_position = has_repetition(node);
            match &existing.node_type {
                n @ Pv(path) => {
                    if !is_repeated_position
                        && existing.depth >= ctx.depth
                        && path.len() > 0
                        && is_pseudo_legal(node, path.first().unwrap())
                    {
                        let adjusted_eval = min(ctx.beta, max(ctx.alpha, existing.eval));
                        TableLookup::Hit(SearchResponse { eval: adjusted_eval, path: path.clone() })
                    } else {
                        TableLookup::Suggestion(n.clone())
                    }
                }
                n @ Cut(m) => {
                    if !is_repeated_position
                        && existing.depth >= ctx.depth
                        && ctx.beta <= existing.eval
                        && is_pseudo_legal(node, m)
                    {
                        TableLookup::Hit(SearchResponse { eval: ctx.beta, path: vec![] })
                    } else {
                        TableLookup::Suggestion(n.clone())
                    }
                }
                n @ All(m) => {
                    if !is_repeated_position
                        && existing.depth >= ctx.depth
                        && existing.eval <= ctx.alpha
                        && is_pseudo_legal(node, m)
                    {
                        // Since we have a fail hard framework don't return the exact eval, but the
                        // current alpha value
                        TableLookup::Hit(SearchResponse { eval: ctx.alpha, path: vec![] })
                    } else {
                        TableLookup::Suggestion(n.clone())
                    }
                }
            }
        } else {
            TableLookup::Miss
        }
    }

    fn generate_moves(
        &self,
        node: &TreeNode,
        ctx: &Context,
        table_entry: &Option<NodeType>,
    ) -> Vec<SearchMove> {
        let mut mvs = self.moves.generate(node);
        table_entry.as_ref().map(|n| {
            reposition_first(
                &mut mvs,
                match n {
                    Pv(path) => path.first().unwrap(),
                    Cut(m) => m,
                    All(m) => m,
                },
            )
        });
        ctx.known_raise_alpha.as_ref().map(|m| reposition_first(&mut mvs, m));
        self.pv.get_next_move(ctx.precursors.as_slice()).map(|m| reposition_first(&mut mvs, &m));
        mvs
    }
}

fn has_repetition(node: &TreeNode) -> bool {
    node.position()
        .history
        .iter()
        .rev()
        .take_while(|(_, m)| m.is_repeatable())
        .any(|(d, _)| d.key == node.position().key)
}

fn is_pseudo_legal(node: &TreeNode, m: &Move) -> bool {
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

fn should_try_null_move_pruning(node: &TreeNode, ctx: &Context) -> bool {
    let position = node.position();
    ctx.depth < 5 && ctx.beta < 1000 && !position.in_check() && {
        let active = position.active;
        let pawns = position.piece_boards[create_piece(active, class::P)];
        let others = position.side_boards[active] & !pawns;
        pawns.count_ones() > 2 && others.count_ones() > 1 || others.count_ones() > 2
    }
}
