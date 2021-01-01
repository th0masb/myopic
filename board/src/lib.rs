#[macro_use]
extern crate lazy_static;

pub use crate::implementation::Board;
use anyhow::Result;
pub use mv::Move;
pub use parse::uci::UciMove;
pub use myopic_core::*;

mod implementation;
mod mv;
mod parse;

/// The start position of a chess game encoded in FEN format
pub const STARTPOS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Return the start position of a standard game
pub fn start() -> Board {
    STARTPOS_FEN.parse().unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
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
pub enum Termination {
    Draw,
    Loss,
}

/// Represents the individual components which make up a board position
/// encoded as a FEN string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum FenComponent {
    Board,
    Active,
    CastlingRights,
    Enpassant,
    HalfMoveCount,
    MoveCount,
}

/// Trait representing a mutable state of play of a chess game
/// which can be evolved/devolved via (applicable) Move instances,
/// compute the set of legal moves and queried for a variety of
/// properties.
pub trait ChessBoard: Clone + Send {
    /// Evolves the position by making the given move. If the source hash
    /// of the move does not match the hash of this position (prior to making
    /// the move) then an error will be returned. If the hash matches but
    /// the move is illegal in this position (e.g if you manually start
    /// creating moves) then the results are undefined.
    fn make(&mut self, action: Move) -> Result<()>;

    /// Reverses and returns the move which was made last. If no move has
    /// been made yet then an error is returned.
    fn unmake(&mut self) -> Result<Move>;

    /// Compute a vector of all the legal moves in this position for the
    /// given computation type. Note there is no particular ordering to the
    /// move vector.
    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move>;

    /// Compute the termination state of this node. If it is not terminal
    /// nothing is returned, if it is then the manner of termination is
    /// returned wrapped inside an Option. The termination can be only a
    /// draw or a loss since a side only loses when it runs out of moves,
    /// i.e. you don't play a winning move, you just fail to have a legal
    /// move.
    fn termination_status(&mut self) -> Option<Termination>;

    /// Determines whether the active side is in a state of check.
    fn in_check(&mut self) -> bool;

    /// Return the locations of all pieces on the given side.
    fn side(&self, side: Side) -> BitBoard;

    /// Return the locations of all white and black pieces.
    fn sides(&self) -> (BitBoard, BitBoard);

    /// Returns the Zobrist hash of this position.
    fn hash(&self) -> u64;

    /// Return the active side in this position, i.e. the one whose turn it is.
    fn active(&self) -> Side;

    /// Return the enpassant target square in this position.
    fn enpassant(&self) -> Option<Square>;

    /// Return the castling status of the given side.
    fn castle_status(&self, side: Side) -> Option<CastleZone>;

    /// Return the locations of the given pieces.
    fn locs(&self, pieces: &[Piece]) -> BitBoard;

    /// Return the location of the king for the given side.
    fn king(&self, side: Side) -> Square;

    /// Return the piece occupying the given location.
    fn piece(&self, location: Square) -> Option<Piece>;

    /// Return the half move clock value at this position.
    fn half_move_clock(&self) -> usize;

    /// Return the number of previous positions for this board.
    fn position_count(&self) -> usize;

    /// Return the previous moves played to reach this current
    /// position.
    fn previous_moves(&self) -> Vec<Move>;

    /// Return the remaining castling rights from this position.
    fn remaining_rights(&self) -> CastleZoneSet;

    /// Parse the given string as a sequence of pgn encoded moves
    /// starting from the current position. The moves are then
    /// made one by one. The sequence of moves which were made
    /// are returned in a Vec.
    fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>>;

    /// Parse the given string as a sequence of uci encoded moves
    /// starting from the current position. The moves are then
    /// made one by one.The sequence of moves which were made
    /// are returned in a Vec.
    fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>>;

    /// Given a uci encoded move this method will attempt to match
    /// it to the unique matching legal move in this position if it
    /// exist. An error is returned if no matching move exists in
    /// this position.
    fn parse_uci(&mut self, uci_move: &str) -> Result<Move>;

    /// Return the specified components of the FEN encoding of this position
    /// in the given order with components separated by a space.
    fn to_partial_fen(&self, cmps: &[FenComponent]) -> String;

    /// Return the complete FEN representation of this position.
    fn to_fen(&self) -> String {
        self.to_partial_fen(&[
            FenComponent::Board,
            FenComponent::Active,
            FenComponent::CastlingRights,
            FenComponent::Enpassant,
            FenComponent::HalfMoveCount,
            FenComponent::MoveCount,
        ])
    }

    /// Returns the locations of all pieces on the board.
    fn all_pieces(&self) -> BitBoard {
        let (w, b) = self.sides();
        w | b
    }
}

#[cfg(test)]
mod uci_conversion_test {
    use crate::mv::Move;
    use myopic_core::*;

    #[test]
    fn test_pawn_standard_conversion() {
        assert_eq!(
            "e2e4",
            Move::Standard {
                source: 0u64,
                moving: Piece::WP,
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
                source: 0u64,
                moving: Piece::BR,
                from: Square::H1,
                dest: Square::H7,
                capture: Some(Piece::WQ),
            }
            .uci_format()
        );
    }

    #[test]
    fn test_castling_conversion() {
        assert_eq!(
            "e1g1",
            Move::Castle {
                source: 1u64,
                zone: CastleZone::WK
            }
            .uci_format()
        );
        assert_eq!(
            "e1c1",
            Move::Castle {
                source: 1u64,
                zone: CastleZone::WQ
            }
            .uci_format()
        );
        assert_eq!(
            "e8g8",
            Move::Castle {
                source: 8u64,
                zone: CastleZone::BK
            }
            .uci_format()
        );
        assert_eq!(
            "e8c8",
            Move::Castle {
                source: 8u64,
                zone: CastleZone::BQ
            }
            .uci_format()
        );
    }

    #[test]
    fn test_promotion_conversion() {
        assert_eq!(
            "e7d8q",
            Move::Promotion {
                source: 9u64,
                from: Square::E7,
                dest: Square::D8,
                promoted: Piece::WQ,
                capture: Some(Piece::BB),
            }
            .uci_format()
        )
    }

    #[test]
    fn test_enpassant_conversion() {
        assert_eq!(
            "e5d6",
            Move::Enpassant {
                source: 0u64,
                side: Side::White,
                from: Square::E5,
                dest: Square::D6,
                capture: Square::D5,
            }
            .uci_format()
        )
    }
}
