#[macro_use]
extern crate lazy_static;

pub use myopic_core::*;
pub use crate::implementation::MutBoardImpl;

mod implementation;
pub mod parse;

/// The start position of a chess game encoded in FEN format
pub const STARTPOS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, PartialEq)]
pub struct Discards {
    pub rights: CastleZoneSet,
    pub piece: Option<Piece>,
    pub enpassant: Option<Square>,
    pub hash: u64,
    pub half_move_clock: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Move {
    Standard(Piece, Square, Square),
    Enpassant(Square, Square),
    Promotion(Square, Square, Piece),
    Castle(CastleZone),
}

impl Move {
    /// Convert this move into a human readable uci long format string.
    pub fn uci_format(&self) -> String {
        match self {
            &Move::Standard(_, src, dest) => format!("{}{}", src, dest),
            &Move::Enpassant(src, dest) => format!("{}{}", src, dest),
            &Move::Promotion(src, dest, piece) => format!(
                "{}{}{}",
                src,
                dest,
                match piece {
                    Piece::WQ | Piece::BQ => "q",
                    Piece::WR | Piece::BR => "r",
                    Piece::WB | Piece::BB => "b",
                    Piece::WN | Piece::BN => "n",
                    _ => "",
                }
            ),
            &Move::Castle(zone) => {
                let (_, src, dest) = zone.king_data();
                format!("{}{}", src, dest)
            }
        }
        .to_lowercase()
        .to_owned()
    }
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
pub trait MutBoard: Clone + Send + Reflectable {
    /// Evolves this board in place according to the given move reference.
    /// The move must be one that is legal in this position otherwise the
    /// results are undefined. The data which is lost during this evolution
    /// is returned at the end of the procedure allowing for devolution to
    /// take place.
    fn evolve(&mut self, action: &Move) -> Discards;

    /// Reverses the given move, i.e. it devolves the board. It can only be
    /// called after the same move has been used to evolve the board. The
    /// discarded information produced by the evolve call must be provided
    /// here. If any of these conditions are not met the results of this
    /// procedure are undefined.
    fn devolve(&mut self, action: &Move, discards: Discards);

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

    /// Return the locations of the given piece.
    fn locs(&self, piece: Piece) -> BitBoard;

    /// Return the location of the king for the given side.
    fn king(&self, side: Side) -> Square;

    /// Return the piece occupying the given location.
    fn piece(&self, location: Square) -> Option<Piece>;

    /// Return the half move clock value at this position.
    fn half_move_clock(&self) -> usize;

    /// Return the total number of half moves played to reach this position.
    fn history_count(&self) -> usize;

    /// Return the remaining castling rights from this position.
    fn remaining_rights(&self) -> CastleZoneSet;

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

    /// Returns the locations of a set of pieces as a single bitboard.
    fn multi_locs(&self, pieces: &[Piece]) -> BitBoard {
        pieces.into_iter().map(|&p| self.locs(p)).collect()
    }

    /// Returns the locations of all pieces on the board.
    fn all_pieces(&self) -> BitBoard {
        let (w, b) = self.sides();
        w | b
    }
}

/// Create a mutable board state from a fen string if it is valid.
pub fn fen_position(fen: &str) -> Result<MutBoardImpl, String> {
    MutBoardImpl::from_fen(String::from(fen))
}

/// Create a mutable board state representing the start of a standard
/// chess game.
pub fn start_position() -> MutBoardImpl {
    fen_position(STARTPOS_FEN).unwrap()
}

#[cfg(test)]
mod uci_conversion_test {
    use super::Move;
    use myopic_core::*;

    #[test]
    fn test_pawn_standard_conversion() {
        assert_eq!("e2e4", Move::Standard(Piece::WP, Square::E2, Square::E4).uci_format());
    }

    #[test]
    fn test_rook_standard_conversion() {
        assert_eq!("h1h7", Move::Standard(Piece::BR, Square::H1, Square::H7).uci_format());
    }

    #[test]
    fn test_castling_conversion() {
        assert_eq!("e1g1", Move::Castle(CastleZone::WK).uci_format());
        assert_eq!("e1c1", Move::Castle(CastleZone::WQ).uci_format());
        assert_eq!("e8g8", Move::Castle(CastleZone::BK).uci_format());
        assert_eq!("e8c8", Move::Castle(CastleZone::BQ).uci_format());
    }

    #[test]
    fn test_promotion_conversion() {
        assert_eq!("e7d8q", Move::Promotion(Square::E7, Square::D8, Piece::WQ).uci_format())
    }

    #[test]
    fn test_enpassant_conversion() {
        assert_eq!("e5d6", Move::Enpassant(Square::E5, Square::D6).uci_format())
    }
}
