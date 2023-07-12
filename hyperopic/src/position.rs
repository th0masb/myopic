use crate::{Board, ClassMap, CornerMap, Piece, PieceMap, Side, SideMap, Square, SquareMap};

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
    castle_rights: CornerMap<bool>,
    enpassant: Option<Square>,
    clock: usize,
    hash: u64,
}

#[derive(Clone, PartialEq)]
pub struct Position {
    piece_boards: PieceMap<Board>,
    piece_locs: SquareMap<Option<Piece>>,
    side_boards: SideMap<Board>,
    castle_rights: CornerMap<bool>,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    history: Vec<Discards>,
}

impl Position {

}
