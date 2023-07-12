use std::str::FromStr;
use crate::{Board, ClassMap, CornerMap, Piece, PieceMap, Side, SideMap, Square, SquareMap};
use crate::moves::{Move, Moves};
use anyhow::Result;

/// Represents the possible ways a game can be terminated, we only
/// consider a game to be terminated when a side has no legal moves
/// to make or if a special draw condition is met like position
/// repetition. If a side has no legal moves and is currently in check
/// then the game is lost, if it is not in check then the game is
/// drawn.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TerminalState {
    Draw,
    Loss,
}

#[derive(Clone, PartialEq)]
pub struct Discards {
    castling_rights: CornerMap<bool>,
    enpassant: Option<Square>,
    clock: usize,
    hash: u64,
}

#[derive(Clone, PartialEq)]
pub struct Position {
    pub piece_boards: PieceMap<Board>,
    pub piece_locs: SquareMap<Option<Piece>>,
    pub side_boards: SideMap<Board>,
    pub castling_rights: CornerMap<bool>,
    pub active: Side,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
    pub prior_positions: usize,
    pub history: Vec<Discards>,
}

impl Default for Position {
    fn default() -> Self {
        todo!()
    }
}

impl Position {
    pub fn make(&mut self, m: Move) -> Result<()> {
        todo!()
    }

    pub fn unmake(&mut self, m: Move) -> Result<Move> {
        todo!()
    }

    pub fn moves(&self, moves: Moves) -> Vec<Move> {
        todo!()
    }
}
