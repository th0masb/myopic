use crate::Move;

pub struct TranspositionTable {
    inner: Vec<Option<TreeNode>>,
}

impl TranspositionTable {
    pub fn new(n_entries: usize) -> TranspositionTable {
        TranspositionTable { inner: vec![None; n_entries] }
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
    Pv { hash: u64, depth: u8, eval: i32, optimal_path: Vec<Move> },
    Cut { hash: u64, depth: u8, beta: i32, cutoff_move: Move },
    All { hash: u64, depth: u8, eval: i32, best_move: Move },
}

impl TreeNode {
    fn matches(&self, hash: u64) -> bool {
        match self {
            TreeNode::Cut { hash: node_hash, .. } => *node_hash == hash,
            TreeNode::All { hash: node_hash, .. } => *node_hash == hash,
            TreeNode::Pv { hash: node_hash, .. } => *node_hash == hash,
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
