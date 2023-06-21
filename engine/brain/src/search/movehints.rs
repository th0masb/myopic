use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::Evaluator;
use myopic_board::{Move, MoveComputeType};

use crate::search::negascout::SearchResponse;

const SHALLOW_EVAL_BRANCHING: usize = 5;

/// Precomputed suggested moves to aid in move ordering
/// for the search
#[derive(Default)]
pub struct MoveOrderingHints {
    /// Shallow evaluation collections must contain
    /// all the legal moves in the position and allow
    /// a more accurate complete ordering compared to
    /// the heuristic approach
    evs: HashMap<Vec<Move>, Vec<SEMove>>,
}

impl MoveOrderingHints {
    pub fn populate(&mut self, root: &mut Evaluator, depth: usize) {
        self.populate_impl(root, depth, vec![])
    }

    fn populate_impl(&mut self, root: &mut Evaluator, depth: usize, precursors: Vec<Move>) {
        let curr_level = self.compute_shallow_eval(root);
        let next_paths =
            curr_level.iter().map(|m| m.mv.clone()).take(SHALLOW_EVAL_BRANCHING).collect_vec();

        self.set_evs(precursors.clone(), curr_level);

        if depth > 0 {
            for mv in next_paths {
                let mut next_precursors = precursors.clone();
                next_precursors.push(mv.clone());
                root.make(mv).unwrap();
                self.populate_impl(root, depth - 1, next_precursors);
                root.unmake().unwrap();
            }
        }
    }

    fn compute_shallow_eval(&mut self, root: &mut Evaluator) -> Vec<SEMove> {
        let mut dest = vec![];
        for mv in root.board().compute_moves(MoveComputeType::All) {
            root.make(mv).unwrap();
            let SearchResponse { eval, .. } = -super::negascout::search(root, 0).unwrap();
            let mv_made = root.unmake().unwrap();
            dest.push(SEMove { mv: mv_made, eval });
        }
        dest.sort();
        dest.reverse();
        dest
    }

    pub fn get_evs(&self, mvs: &Vec<Move>) -> Option<&Vec<SEMove>> {
        self.evs.get(mvs)
    }

    fn set_evs(&mut self, mvs: Vec<Move>, mut evs: Vec<SEMove>) {
        evs.sort();
        evs.reverse();
        self.evs.insert(mvs, evs);
    }
}

// Shallow eval move
#[derive(Clone, PartialEq, Eq)]
pub struct SEMove {
    pub mv: Move,
    pub eval: i32,
}

impl PartialOrd for SEMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.eval.partial_cmp(&other.eval)
    }
}

impl Ord for SEMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.eval.cmp(&other.eval)
    }
}
