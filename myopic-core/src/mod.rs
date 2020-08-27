use crate::bitboard::BitBoard;
use crate::direction::Dir;
use crate::direction::N;
use crate::direction::S;
use std::collections::BTreeSet;

pub mod bitboard;
pub mod castlezone;
pub mod direction;
pub mod hash;
pub mod square;

/// Represents the two different teams in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Side {
    White,
    Black,
}

impl Side {
    /// Get the vertical direction in which a pawn on this side moves
    /// (north or south).
    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::White => N,
            Side::Black => S,
        }
    }

    /// Get the rank on which a pawn on this side starts the game.
    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[1],
            Side::Black => BitBoard::RANKS[6],
        }
    }

    /// Get the rank to which a pawn on this side moves to following
    /// it's special two rank first move.
    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[3],
            Side::Black => BitBoard::RANKS[4],
        }
    }

    /// Get the rank a pawn on this side must be on for it to be able
    /// to promote on it's next move.
    pub fn pawn_promoting_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[6],
            Side::Black => BitBoard::RANKS[1],
        }
    }


    /// The rank a pawn on this
    pub fn pawn_last_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[7],
            Side::Black => BitBoard::RANKS[0],
        }
    }
}
