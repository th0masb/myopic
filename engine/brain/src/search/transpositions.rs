use myopic_board::Board;
use crate::Move;

pub trait Transpositions {
    fn get(&self, pos: &Board) -> Option<&TreeNode>;
    fn put(&mut self, pos: &Board, n: TreeNode);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TreeNode {
    Pv { hash: u64, depth: u8, eval: i32, optimal_path: Vec<Move> },
    Cut { hash: u64, depth: u8, beta: i32, cutoff_move: Move },
    All { hash: u64, depth: u8, eval: i32, best_move: Move },
}

pub struct TranspositionsImpl {
    inner: Vec<Option<TreeNode>>,
}

impl Transpositions for TranspositionsImpl {
    fn get(&self, pos: &Board) -> Option<&TreeNode> {
        let hash = pos.hash();
        let index = self.index(hash);
        self.inner[index].as_ref().filter(|&m| m.matches(hash))
    }

    fn put(&mut self, pos: &Board, n: TreeNode) {
        let hash = pos.hash();
        let index = self.index(hash);
        self.inner[index] = Some(n);
    }
}

impl TranspositionsImpl {
    pub fn new(n_entries: usize) -> TranspositionsImpl {
        TranspositionsImpl { inner: vec![None; n_entries] }
    }

    fn index(&self, k: u64) -> usize {
        (k % self.inner.len() as u64) as usize
    }
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
