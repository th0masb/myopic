use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::{
    castletracker::CastleTracker, hashcache::HashCache, piecetracker::PieceTracker,
};
use crate::board::Board;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::ReversalData;
use crate::pieces::Piece;
use regex::Regex;

pub mod evolve;
pub mod hash;
pub mod moves;

mod castletracker;
mod hashcache;
mod piecetracker;

#[cfg(test)]
pub mod testutils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardImpl {
    hashes: HashCache,
    pieces: PieceTracker,
    castling: CastleTracker,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
}

impl Board for BoardImpl {
    fn evolve(&mut self, action: &Move) -> ReversalData {
        self.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: ReversalData) {
        self.devolve(action, discards)
    }

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn hash(&self) -> u64 {
        self.hashes.head()
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant_square(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn piece_locations(&self, piece: Piece) -> BitBoard {
        self.pieces.locations(piece)
    }

    fn king_location(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn whites_blacks(&self) -> (BitBoard, BitBoard) {
        (
            self.pieces.side_locations(Side::White),
            self.pieces.side_locations(Side::Black),
        )
    }

    fn piece_at(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn game_counter(&self) -> usize {
        self.hashes.pop_dist()
    }
}

lazy_static! {
    static ref NOT_WHITESPACE: Regex = Regex::new(r"[^ ]+").unwrap();
    static ref RANK: Regex = Regex::new(r"[PpNnBbRrQqKk1-8]{1, 8}").unwrap();
    static ref ACTIVE: Regex = Regex::new(r"[wb]").unwrap();
    static ref RIGHTS: Regex = Regex::new(r"([KkQq]{1, 4})|[-]").unwrap();
    static ref ENPASSANT: Regex = Regex::new(r"[a-h][36]").unwrap();
    static ref COUNT: Regex = Regex::new(r"[0-9]+").unwrap();
}


fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex
        .captures_iter(source)
        .map(|cap| String::from(&cap[0]))
        .collect()
}

fn fen_metadata<'a>() -> impl Iterator<Item = &'a Regex> {
    let mut dest: Vec<&'a Regex> = Vec::new();
    dest.push(&ACTIVE);
    dest.push(&RIGHTS);
    dest.push(&ENPASSANT);
    dest.push(&COUNT);
    dest.push(&COUNT);
    dest.into_iter()
}

impl BoardImpl {
    fn from_fen(fen_string: String) -> Result<BoardImpl, String> {
        let initial_split = find_matches(&fen_string, &NOT_WHITESPACE);
        if initial_split.len() != 6 {
            Err(fen_string)
        } else {
            let ranks = find_matches(&initial_split[0], &RANK);
            let meta_match = fen_metadata().zip(&initial_split[1..]).all(|(re, s)| re.is_match(s));
            if ranks.len() != 8 || !meta_match {
                Err(fen_string)
            } else {
                // We know all parts are valid here...
                unimplemented!()
            }
        }
    }

    fn switch_side(&mut self) {
        self.active = self.active.reflect();
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache, returning the overwritten value.
    fn update_hash(&mut self) {
        let next_hash = self.pieces.hash()
            ^ self.castling.hash()
            ^ hash::side_feature(self.active)
            ^ self.enpassant.map_or(0u64, |x| hash::enpassant_feature(x));
        self.hashes.push_head(next_hash)
    }
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
