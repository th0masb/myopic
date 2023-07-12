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

#[derive(Debug, Clone, PartialEq)]
pub struct Discards {
    pub castling_rights: CornerMap<bool>,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub piece_boards: PieceMap<Board>,
    pub piece_locs: SquareMap<Option<Piece>>,
    pub side_boards: SideMap<Board>,
    pub castling_rights: CornerMap<bool>,
    pub active: Side,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
    pub history: Vec<Discards>,
}

impl Default for Position {
    fn default() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse().unwrap()
    }
}

impl Position {
    pub fn make(&mut self, m: Move) -> Result<()> {
        todo!()
    }

    pub fn unmake(&mut self) -> Result<Move> {
        todo!()
    }

    pub fn moves(&self, moves: Moves) -> Vec<Move> {
        todo!()
    }

    pub fn create_discards(&self) -> Discards {
        Discards {
            castling_rights: self.castling_rights.clone(),
            enpassant: self.enpassant,
            clock: self.clock,
            key: self.key,
        }
    }
}
