use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::pieces::Piece;

mod implementation;
#[cfg(test)]
mod testutils;


#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<Piece>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
    discarded_clock: usize,
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
pub trait Board: Clone + Eq + Reflectable {
    fn evolve(&mut self, action: &Move) -> ReversalData;

    fn devolve(&mut self, action: &Move, discards: ReversalData);

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move>;

    fn hash(&self) -> u64;

    fn active(&self) -> Side;

    fn enpassant_square(&self) -> Option<Square>;

    fn castle_status(&self, side: Side) -> Option<CastleZone>;

    fn piece_locations(&self, piece: Piece) -> BitBoard;

    fn king_location(&self, side: Side) -> Square;

    fn whites_blacks(&self) -> (BitBoard, BitBoard);

    fn piece_at(&self, location: Square) -> Option<Piece>;

    fn half_move_clock(&self) -> usize;

    fn game_counter(&self) -> usize;
}

impl Move {
    pub fn standards(moving: Piece, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets
            .into_iter()
            .map(move |target| Move::Standard(moving, src, target))
    }

    pub fn promotions(side: Side, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().flat_map(move |target| {
            Move::promotion_targets(side)
                .iter()
                .map(move |&piece| Move::Promotion(src, target, piece))
        })
    }

    fn promotion_targets<'a>(side: Side) -> &'a [Piece; 4] {
        match side {
            Side::White => &[Piece::WQ, Piece::WR, Piece::WB, Piece::WN],
            Side::Black => &[Piece::BQ, Piece::BR, Piece::BB, Piece::BN],
        }
    }
}

impl Reflectable for Move {
    fn reflect(&self) -> Self {
        match self {
            Move::Castle(zone) => Move::Castle(zone.reflect()),
            Move::Enpassant(square) => Move::Enpassant(square.reflect()),
            Move::Standard(p, s, t) => Move::Standard(p.reflect(), s.reflect(), t.reflect()),
            Move::Promotion(s, t, p) => Move::Promotion(s.reflect(), t.reflect(), p.reflect()),
        }
    }
}
