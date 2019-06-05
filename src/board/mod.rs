use crate::base::castlezone::CastleZoneSet;
use crate::pieces::Piece;
use crate::base::square::Square;
use crate::base::castlezone::CastleZone;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::bitboard::BitBoard;

mod implementation;

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

impl Move {
    pub fn standards(
        moving_piece: Piece,
        source: Square,
        targets: BitBoard,
    ) -> impl Iterator<Item = Move> {
        targets
            .into_iter()
            .map(move |target| Move::Standard(moving_piece, source, target))
    }

    pub fn promotions(side: Side, source: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().flat_map(move |target| {
            Move::promotion_targets(side)
                .iter()
                .map(move |&piece| Move::Promotion(source, target, piece))
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
