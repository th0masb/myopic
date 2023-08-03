use crate::moves::Move;
use crate::position::Position;
use TreeNode::*;

pub trait Transpositions {
    fn get(&self, pos: &Position) -> Option<&TreeNode>;
    fn put(&mut self, pos: &Position, n: TreeNode);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TableEntry {
    pub root_index: u16,
    pub key: u64,
    pub depth: u8,
    pub eval: i32,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeType {
    Pv(Vec<Move>),
    Cut(Move),
    All(Move),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TreeNode {
    Pv { hash: u64, depth: u8, eval: i32, best_path: Vec<Move> },
    Cut { hash: u64, depth: u8, beta: i32, cutoff_move: Move },
    All { hash: u64, depth: u8, eval: i32, best_move: Move },
}

pub struct TranspositionsImpl {
    inner: Vec<Option<TreeNode>>,
}

impl Transpositions for TranspositionsImpl {
    fn get(&self, pos: &Position) -> Option<&TreeNode> {
        let index = self.index(pos.key);
        self.inner[index].as_ref().filter(|&m| m.matches(pos.key))
    }

    fn put(&mut self, pos: &Position, n: TreeNode) {
        let index = self.index(pos.key);
        let curr = &self.inner[index];
        //if curr.is_some() && curr.as_ref().unwrap().depth() > n.depth() {
        //    return
        //}
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
    pub fn matches(&self, hash: u64) -> bool {
        match self {
            Cut { hash: node_hash, .. } => *node_hash == hash,
            All { hash: node_hash, .. } => *node_hash == hash,
            Pv { hash: node_hash, .. } => *node_hash == hash,
        }
    }

    pub fn depth(&self) -> usize {
        (match self {
            &Pv { depth, .. } => depth,
            &Cut { depth, .. } => depth,
            &All { depth, .. } => depth,
        }) as usize
    }

    pub fn get_move(&self) -> &Move {
        match self {
            Pv { best_path: optimal_path, .. } => optimal_path.first().unwrap(),
            Cut { cutoff_move, .. } => cutoff_move,
            All { best_move, .. } => best_move,
        }
    }
}
