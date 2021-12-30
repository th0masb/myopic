use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use myopic_board::{Move, MoveComputeType};

use crate::EvalChessBoard;
use crate::search::negascout::SearchResponse;

const SHALLOW_EVAL_BRANCHING: usize = 5;

/// Precomputed suggested moves to aid in move ordering
/// for the search
pub struct OrderingHints<B: EvalChessBoard> {
    /// The root position from which all move sequences
    /// start from
    root: B,
    /// Principal variation moves which are the highest
    /// priority moves to try
    pvs: HashMap<Vec<Move>, Vec<PVMove>>,
    /// Shallow evaluation collections must contain
    /// all the legal moves in the position and allow
    /// a more accurate complete ordering compared to
    /// the heuristic approach
    evs: HashMap<Vec<Move>, Vec<SEMove>>,
}

impl<B: EvalChessBoard> OrderingHints<B> {
    pub fn new(root: B) -> OrderingHints<B> {
        OrderingHints {
            root,
            pvs: HashMap::default(),
            evs: HashMap::default(),
        }
    }

    pub fn populate_shallow_eval(&mut self, depth: usize) {
        self.populate_shallow_eval_impl(&mut self.root.clone(), depth, vec![])
    }

    fn populate_shallow_eval_impl(&mut self, board: &mut B, depth: usize, precursors: Vec<Move>) {
        let curr_level = OrderingHints::compute_shallow_eval(board);
        let next_paths = curr_level
            .iter()
            .map(|m| m.mv.clone())
            .take(SHALLOW_EVAL_BRANCHING)
            .collect_vec();

        self.set_evs(precursors.clone(), curr_level);

        if depth > 0 {
            for mv in next_paths {
                let mut next_precursors = precursors.clone();
                next_precursors.push(mv.clone());
                board.make(mv).unwrap();
                self.populate_shallow_eval_impl(board, depth - 1, next_precursors);
                board.unmake().unwrap();
            }
        }
    }

    fn compute_shallow_eval(root: &mut B) -> Vec<SEMove> {
        let mut dest = vec![];
        for mv in root.compute_moves(MoveComputeType::All) {
            root.make(mv).unwrap();
            let SearchResponse { eval, .. } = -super::negascout::search(root, 0).unwrap();
            let mv_made = root.unmake().unwrap();
            dest.push(SEMove { mv: mv_made, eval });
        }
        dest.sort();
        dest.reverse();
        dest
    }

    pub fn get_pvs(&self, mvs: &Vec<Move>) -> Option<&Vec<PVMove>> {
        self.pvs.get(mvs)
    }

    pub fn add_pv(&mut self, depth: usize, pv: &Vec<Move>) {
        for (i, mv) in pv.iter().enumerate() {
            let precursors = pv.iter().cloned().take(i).collect_vec();
            self.add_pv_impl(
                &precursors,
                PVMove {
                    mv: mv.clone(),
                    depth,
                },
            );
        }
    }

    fn add_pv_impl(&mut self, mvs: &Vec<Move>, pv: PVMove) {
        match self.pvs.get_mut(mvs) {
            None => {
                self.pvs.insert(mvs.clone(), vec![pv]);
            }
            Some(pvs) => {
                pvs.push(pv);
                pvs.sort();
                pvs.reverse();
            }
        }
    }

    pub fn get_evs(&self, mvs: &Vec<Move>) -> Option<&Vec<SEMove>> {
        self.evs.get(mvs)
    }

    pub fn set_evs(&mut self, mvs: Vec<Move>, mut evs: Vec<SEMove>) {
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

// Principal variation move
#[derive(Clone, PartialEq, Eq)]
pub struct PVMove {
    pub mv: Move,
    pub depth: usize,
}

impl PartialOrd for PVMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.depth.partial_cmp(&other.depth)
    }
}

impl Ord for PVMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth)
    }
}
