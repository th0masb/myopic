use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::pieces::Piece;

pub use self::implementation::BoardImpl;

#[cfg(test)]
mod test_board;
mod implementation;

#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    pub discarded_rights: CastleZoneSet,
    pub discarded_piece: Option<Piece>,
    pub discarded_enpassant: Option<Square>,
    pub discarded_hash: u64,
    pub discarded_clock: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Move {
    Standard(Piece, Square, Square),
    Enpassant(Square),
    Promotion(Square, Square, Piece),
    Castle(CastleZone),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum MoveComputeType {
    All, Attacks,
    AttacksChecks
}

/// Trait representing a mutable state of play of a chess game
/// which can be evolved/devolved via (applicable) Move instances,
/// compute the set of legal moves and queried for a variety of
/// properties.
///
pub trait Board: Clone + Reflectable {
    /// Evolves this board in place according to the given move reference.
    /// The move must be one that is legal in this position otherwise the
    /// results are undefined. The data which is lost during this evolution
    /// is returned at the end of the procedure allowing for devolution to
    /// take place.
    ///
    fn evolve(&mut self, action: &Move) -> ReversalData;

    /// Reverses the given move, i.e. it devolves the board. It can only be
    /// called after the same move has been used to evolve the board. The
    /// discarded information produced by the evolve call must be provided
    /// here. If any of these conditions are not met the results of this
    /// procedure are undefined.
    ///
    fn devolve(&mut self, action: &Move, discards: ReversalData);

    /// Compute a vector of all the legal moves in this position for the
    /// given computation type. Note there is no particular ordering to the
    /// move vector.
    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move>;

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

    /// Return the locations of all white and black pieces.
    fn sides(&self) -> (BitBoard, BitBoard);

    /// Return the piece occupying the given location.
    fn piece(&self, location: Square) -> Option<Piece>;

    /// Return the half move clock value at this position.
    fn half_move_clock(&self) -> usize;

    /// Return the total number of half moves played to reach this position.
    fn history_count(&self) -> usize;
}

pub fn from_fen(fen: &str) -> Result<BoardImpl, String> {
    BoardImpl::from_fen(String::from(fen))
}

pub fn start() -> BoardImpl {
    from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
}
