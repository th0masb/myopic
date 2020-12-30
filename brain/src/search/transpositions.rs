use crate::Move;
use anyhow::{anyhow, Result};

// Let's estimate 24 bytes per table entry
const MAX_ENTRIES: usize = 30_000_000;

pub struct TranspositionTable {
    inner: Vec<Option<TreeNode>>,
}

impl TranspositionTable {
    pub fn new(n_entries: usize) -> Result<TranspositionTable> {
        if n_entries == 0 || n_entries > MAX_ENTRIES {
            Err(anyhow!("Cannot create table with {} entries", n_entries))
        } else {
            let mut inner = Vec::with_capacity(n_entries);
            for _ in 0..n_entries {
                inner.push(None)
            }
            Ok(TranspositionTable { inner })
        }
    }

    pub fn get(&self, k: u64) -> Option<&TreeNode> {
        let index = self.index(k);
        self.inner[index].as_ref().filter(|&m| m.matches(k))
    }

    pub fn insert(&mut self, k: u64, v: TreeNode) {
        let index = self.index(k);
        self.inner[index] = Some(v);
    }

    fn index(&self, k: u64) -> usize {
        (k % self.inner.len() as u64) as usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TreeNode {
    Pv {
        depth: u8,
        eval: i32,
        optimal_path: Vec<Move>,
    },
    Cut {
        depth: u8,
        beta: i32,
        cutoff_move: Move,
    },
    All {
        depth: u8,
        eval: i32,
        best_move: Move,
    },
}
impl TreeNode {
    fn matches(&self, hash: u64) -> bool {
        match self {
            TreeNode::Cut { cutoff_move, .. } => cutoff_move.source() == hash,
            TreeNode::All { best_move, .. } => best_move.source() == hash,
            TreeNode::Pv { optimal_path, .. } => optimal_path
                .first()
                .map(|m| m.source() == hash)
                .unwrap_or(false),
        }
    }

    pub fn depth(&self) -> usize {
        (match self {
            &TreeNode::Pv { depth, .. } => depth,
            &TreeNode::Cut { depth, .. } => depth,
            &TreeNode::All { depth, .. } => depth,
        }) as usize
    }
}
