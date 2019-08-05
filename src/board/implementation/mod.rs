use regex::Regex;

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

mod evolve;
mod hash;
mod moves;
mod castletracker;
mod hashcache;
mod piecetracker;
mod traitimpl;

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

lazy_static! {
    static ref NOT_WHITESPACE: Regex = Regex::new(r"[^ ]+").unwrap();
    static ref RANK: Regex = Regex::new(r"[PpNnBbRrQqKk1-8]{1, 8}").unwrap();
    static ref ACTIVE: Regex = Regex::new(r"[wb]").unwrap();
    static ref RIGHTS: Regex = Regex::new(r"([KkQq]{1, 4})|[-]").unwrap();
    static ref ENPASSANT: Regex = Regex::new(r"([a-h][36])|[-]").unwrap();
    static ref COUNT: Regex = Regex::new(r"[0-9]+").unwrap();
}

fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex
        .captures_iter(source)
        .map(|cap| String::from(&cap[0]))
        .collect()
}

fn fen_metadata_matchers<'a>() -> impl Iterator<Item = &'a Regex> {
    let mut dest: Vec<&'a Regex> = Vec::new();
    dest.extend_from_slice(&[&ACTIVE, &RIGHTS, &ENPASSANT, &COUNT, &COUNT]);
    dest.into_iter()
}

fn side_from_fen(fen: &String) -> Side {
    match fen.to_lowercase().as_ref() {
        "w" => Side::White,
        "b" => Side::Black,
        _ => panic!(),
    }
}

fn enpassant_from_fen(fen: &String) -> Option<Square> {
    if fen.contains("-") {
        None
    } else {
        Some(Square::from_string(fen))
    }
}

impl BoardImpl {
    pub(super) fn from_fen(fen_string: String) -> Result<BoardImpl, String> {
        let initial_split = find_matches(&fen_string, &NOT_WHITESPACE);
        if initial_split.len() != 6 {
            Err(fen_string)
        } else {
            let ranks = find_matches(&initial_split[0], &RANK);
            let meta_match = fen_metadata_matchers()
                .zip(&initial_split[1..])
                .all(|(re, s)| re.is_match(s));
            if ranks.len() != 8 || !meta_match {
                Err(fen_string)
            } else {
                // We know all parts are valid here...
                let pieces = PieceTracker::from_fen(ranks);
                let active = side_from_fen(&initial_split[1]);
                let castling = CastleTracker::from_fen(&initial_split[2]);
                let enpassant = enpassant_from_fen(&initial_split[3]);
                let clock = *(&initial_split[4].parse::<usize>().unwrap());
                let move_count = *(&initial_split[5].parse::<usize>().unwrap());
                let hash = hash(&pieces, &castling, active, enpassant);
                let n_previous_pos = 2 * (move_count - 1) + (active as usize);
                Ok(BoardImpl {
                    pieces,
                    castling,
                    active,
                    enpassant,
                    clock,
                    hashes: HashCache::new(hash, n_previous_pos),
                })
            }
        }
    }

    fn switch_side(&mut self) {
        self.active = self.active.reflect();
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache.
    fn update_hash(&mut self) {
        self.hashes.push_head(hash(
            &self.pieces,
            &self.castling,
            self.active,
            self.enpassant,
        ))
    }
}

fn hash(pt: &PieceTracker, ct: &CastleTracker, active: Side, ep: Option<Square>) -> u64 {
    pt.hash()
        ^ ct.hash()
        ^ hash::side_feature(active)
        ^ ep.map_or(0u64, |x| hash::enpassant_feature(x))
}

impl Move {
    fn standards(moving: Piece, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets
            .into_iter()
            .map(move |target| Move::Standard(moving, src, target))
    }

    fn promotions(side: Side, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
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

#[cfg(test)]
mod fen_test {
    use super::testutils;
    use crate::board::test_board::TestBoard;
    use crate::board::BoardImpl;
    use crate::base::bitboard::constants::*;
    use crate::base::castlezone::CastleZoneSet;
    use crate::base::castlezone::CastleZone;
    use crate::base::Side;

    fn test(expected: TestBoard, fen_string: String) {
        assert_eq!(BoardImpl::from(expected), BoardImpl::from_fen(fen_string).unwrap())
    }

    #[test]
    fn case_1() {
        let fen = "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19";
        let board = TestBoard {
            whites: vec![A3 | B2 | C4 | E4 | F2 | G2 | H2, C3 | D4, E2, D1 | F1, D2, G1],
            blacks: vec![A6 | B7 | D6 | E6 | F7 | G6 | H6, E7, C8 | G7, A8 | D8, C7, G8],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 3,
            active: Side::White,
            enpassant: None,
            history_count: 36,
        };
        test(board, String::from(fen));
    }

    #[test]
    fn case_2() {
        let fen = "rnb2rk1/ppp2ppp/4pq2/8/2PP4/5N2/PP3PPP/R2QKB1R w KQ - 2 9";
        let board = TestBoard {
            whites: vec![A2 | B2 | C4 | D4 | F2 | G2 | H2, F3, F1, A1 | H1, D1, E1],
            blacks: vec![A7 | B7 | C7 | E6 | F7 | G7 | H7, B8, C8, A8 | F8, F6, G8],
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::White,
            enpassant: None,
            history_count: 16,
        };
        test(board, String::from(fen));
    }
}
