use crate::moves::Move;
use crate::position::Position;
use std::cmp::min;

pub trait Transpositions {
    fn get(&self, pos: &Position) -> Option<&TableEntry>;
    fn put(&mut self, pos: &Position, root_index: u16, depth: u8, eval: i32, node_type: NodeType);
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

pub struct TranspositionsImpl {
    inner: Vec<Option<TableEntry>>,
}

impl Transpositions for TranspositionsImpl {
    fn get(&self, pos: &Position) -> Option<&TableEntry> {
        let index = self.index(pos.key);
        self.inner[index].as_ref().filter(|&m| m.key == pos.key)
    }

    fn put(&mut self, pos: &Position, root_index: u16, depth: u8, eval: i32, node_type: NodeType) {
        let index = self.index(pos.key);
        if let Some(existing) = &self.inner[index] {
            let index_diff = root_index - min(existing.root_index, root_index);
            if existing.depth as u16 > depth as u16 + index_diff {
                return;
            }
        }
        self.inner[index] = Some(TableEntry { root_index, depth, eval, key: pos.key, node_type });
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
