use std::cell::RefCell;
use std::fmt::Debug;

use enum_map::EnumMap;

pub use moves::Move;
use myopic_core::anyhow::Result;
use myopic_core::enum_map::Enum;
pub use myopic_core::*;
pub use parse::uci::UciMove;

use crate::enumset::EnumSet;
use crate::private::cache::CalculationCache;
use crate::private::history::History;
use crate::private::positions::Positions;
use crate::private::rights::Rights;

mod moves;
mod parse;
mod private;

/// The start position of a chess game encoded in FEN format
pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Return the start position of a standard game
pub fn start() -> Board {
    START_FEN.parse().unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Enum)]
pub enum MoveComputeType {
    All,
    Attacks,
    /// If a promoting move causes check then all promoting moves for
    /// the four different target pieces will be included for that pawn.
    AttacksChecks,
}

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

/// Represents the individual components which make up a board position
/// encoded as a FEN string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum FenPart {
    Board,
    Active,
    CastlingRights,
    Enpassant,
    HalfMoveCount,
    MoveCount,
}

/// Chessboard representation on which a game can be played
#[derive(Debug, Clone)]
pub struct Board {
    history: History,
    pieces: Positions,
    rights: Rights,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    cache: RefCell<CalculationCache>,
}

impl Board {
    /// Evolves the position by making the given move. If the source hash
    /// of the move does not match the hash of this position (prior to making
    /// the move) then an error will be returned. If the hash matches but
    /// the move is illegal in this position (e.g if you manually start
    /// creating moves) then the results are undefined.
    pub fn make(&mut self, mv: Move) -> Result<()> {
        self.make_impl(mv)
    }

    /// Reverses and returns the move which was made last. If no move has
    /// been made yet then an error is returned.
    pub fn unmake(&mut self) -> Result<Move> {
        self.unmake_impl()
    }

    /// Parse the given string as a sequence of pgn encoded moves
    /// starting from the current position. The moves are then
    /// made one by one. The sequence of moves which were made
    /// are returned in a Vec.
    pub fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in parse::pgn::moves(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    /// Parse the given string as a sequence of uci encoded moves
    /// starting from the current position. The moves are then
    /// made one by one.The sequence of moves which were made
    /// are returned in a Vec.
    pub fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in parse::uci::move_sequence(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    /// Compute a vector of all the legal moves in this position for the
    /// given computation type. Note there is no particular ordering to the
    /// move vector. If we are in check then the type is ignored and all
    /// legal moves are returned.
    pub fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    /// Compute the termination state of this node. If it is not terminal
    /// nothing is returned, if it is then the manner of termination is
    /// returned wrapped inside an Option. The termination can be only a
    /// draw or a loss since a side only loses when it runs out of moves,
    /// i.e. you don't play a winning move, you just fail to have a legal
    /// move.
    pub fn terminal_state(&self) -> Option<TerminalState> {
        self.terminal_state_impl()
    }

    /// Determines whether the active side is in a state of check.
    pub fn in_check(&self) -> bool {
        self.passive_control().contains(self.king(self.active))
    }

    /// Return the locations of all pieces on the given side.
    pub fn side(&self, side: Side) -> BitBoard {
        match side {
            Side::W => self.pieces.whites(),
            Side::B => self.pieces.blacks(),
        }
    }

    /// Return the locations of all white and black pieces.
    pub fn sides(&self) -> (BitBoard, BitBoard) {
        (self.pieces.side_locations(Side::W), self.pieces.side_locations(Side::B))
    }

    /// Returns the Zobrist hash of this position.
    pub fn hash(&self) -> u64 {
        private::hash(&self.pieces, &self.rights, self.active, self.enpassant)
    }

    /// Return the active side in this position, i.e. the one whose turn it is.
    pub fn active(&self) -> Side {
        self.active
    }

    /// Return the enpassant target square in this position.
    pub fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    /// Return the locations of the given pieces.
    pub fn locs(&self, pieces: &[Piece]) -> BitBoard {
        pieces.into_iter().map(|&p| self.pieces.locs(p)).fold(BitBoard::EMPTY, |l, r| l | r)
    }

    /// Return the location of the king for the given side.
    pub fn king(&self, side: Side) -> Square {
        self.pieces.locs(Piece(side, Class::K)).into_iter().next().unwrap()
    }

    /// Return the piece occupying the given location.
    pub fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    /// Return the half move clock value at this position.
    pub fn half_move_clock(&self) -> usize {
        self.clock
    }

    /// Return the number of previous positions for this board.
    pub fn position_count(&self) -> usize {
        self.history.position_count()
    }

    /// Return the remaining castling rights from this position.
    pub fn remaining_rights(&self) -> EnumMap<Side, EnumSet<Flank>> {
        self.rights.0.clone()
    }

    /// Given a uci encoded move this method will attempt to match
    /// it to the unique matching legal move in this position if it
    /// exist. An error is returned if no matching move exists in
    /// this position.
    pub fn parse_uci(&self, uci_move: &str) -> Result<Move> {
        parse::uci::single_move(self, uci_move)
    }

    /// Return the specified components of the FEN encoding of this position
    /// in the given order with components separated by a space.
    pub fn to_fen_parts(&self, parts: &[FenPart]) -> String {
        private::fen::to_fen_impl(self, parts)
    }

    /// Return the complete FEN representation of this position.
    pub fn to_fen(&self) -> String {
        self.to_fen_parts(&[
            FenPart::Board,
            FenPart::Active,
            FenPart::CastlingRights,
            FenPart::Enpassant,
            FenPart::HalfMoveCount,
            FenPart::MoveCount,
        ])
    }

    /// Returns the locations of all pieces on the board.
    pub fn all_pieces(&self) -> BitBoard {
        let (w, b) = self.sides();
        w | b
    }
}

impl Default for Board {
    fn default() -> Self {
        START_FEN.parse().unwrap()
    }
}

impl PartialEq<Board> for Board {
    fn eq(&self, other: &Board) -> bool {
        self.pieces == other.pieces
            && self.rights == other.rights
            && self.enpassant == other.enpassant
            && self.active == other.active
            && self.half_move_clock() == other.half_move_clock()
    }
}

#[cfg(test)]
mod uci_conversion_test {
    use myopic_core::*;

    use crate::moves::Move;

    #[test]
    fn test_pawn_standard_conversion() {
        assert_eq!(
            "e2e4",
            Move::Standard {
                moving: Piece(Side::W, Class::P),
                from: Square::E2,
                dest: Square::E4,
                capture: None,
            }
            .uci_format()
        );
    }

    #[test]
    fn test_rook_standard_conversion() {
        assert_eq!(
            "h1h7",
            Move::Standard {
                moving: Piece(Side::B, Class::R),
                from: Square::H1,
                dest: Square::H7,
                capture: Some(Piece(Side::W, Class::Q)),
            }
            .uci_format()
        );
    }

    #[test]
    fn test_castling_conversion() {
        assert_eq!("e1g1", Move::Castle { corner: Corner(Side::W, Flank::K) }.uci_format());
        assert_eq!("e1c1", Move::Castle { corner: Corner(Side::W, Flank::Q) }.uci_format());
        assert_eq!("e8g8", Move::Castle { corner: Corner(Side::B, Flank::K) }.uci_format());
        assert_eq!("e8c8", Move::Castle { corner: Corner(Side::B, Flank::Q) }.uci_format());
    }

    #[test]
    fn test_promotion_conversion() {
        assert_eq!(
            "e7d8q",
            Move::Promotion {
                from: Square::E7,
                dest: Square::D8,
                promoted: Piece(Side::W, Class::Q),
                capture: Some(Piece(Side::B, Class::B)),
            }
            .uci_format()
        )
    }

    #[test]
    fn test_enpassant_conversion() {
        assert_eq!(
            "e5d6",
            Move::Enpassant {
                side: Side::W,
                from: Square::E5,
                dest: Square::D6,
                capture: Square::D5,
            }
            .uci_format()
        )
    }
}
