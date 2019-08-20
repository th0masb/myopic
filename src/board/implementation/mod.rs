use regex::Regex;

use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::cache::CalculationCache;
use crate::board::implementation::castling::Castling;
use crate::board::implementation::history::History;
use crate::board::implementation::positions::Positions;
use crate::board::Board;
use crate::board::Discards;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::pgn::find_matches;
use crate::pieces::Piece;

mod cache;
mod castling;
mod evolve;
mod history;
mod moves;
mod positions;
#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardImpl {
    history: History,
    pieces: Positions,
    castling: Castling,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    cache: CalculationCache,
}

lazy_static! {
    static ref NOT_WHITESPACE: Regex = Regex::new(r"[^ ]+").unwrap();
    static ref RANK: Regex = Regex::new(r"[PpNnBbRrQqKk1-8]{1, 8}").unwrap();
    static ref ACTIVE: Regex = Regex::new(r"[wb]").unwrap();
    static ref RIGHTS: Regex = Regex::new(r"([KkQq]{1, 4})|[-]").unwrap();
    static ref ENPASSANT: Regex = Regex::new(r"([a-h][36])|[-]").unwrap();
    static ref COUNT: Regex = Regex::new(r"[0-9]+").unwrap();
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
            let meta_match =
                fen_metadata_matchers().zip(&initial_split[1..]).all(|(re, s)| re.is_match(s));
            if ranks.len() != 8 || !meta_match {
                Err(fen_string)
            } else {
                // We know all parts are valid here...
                let pieces = Positions::from_fen(ranks);
                let active = side_from_fen(&initial_split[1]);
                let castling = Castling::from_fen(&initial_split[2]);
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
                    history: History::new(hash, n_previous_pos),
                    cache: CalculationCache::empty(),
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
        self.history.push_head(hash(&self.pieces, &self.castling, self.active, self.enpassant))
    }
}

fn hash(pt: &Positions, ct: &Castling, active: Side, ep: Option<Square>) -> u64 {
    pt.hash()
        ^ ct.hash()
        ^ crate::base::hash::side_feature(active)
        ^ ep.map_or(0u64, |x| crate::base::hash::enpassant_feature(x))
}

impl Move {
    fn standards(moving: Piece, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().map(move |target| Move::Standard(moving, src, target))
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

#[cfg(test)]
mod fen_test {
    use crate::base::bitboard::constants::*;
    use crate::base::castlezone::CastleZone;
    use crate::base::castlezone::CastleZoneSet;
    use crate::base::square::Square;
    use crate::base::Side;
    use crate::board::test_board::TestBoard;
    use crate::board::BoardImpl;

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

    #[test]
    fn case_3() {
        let fen = "r1bqkbnr/ppp1pppp/n7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let board = TestBoard {
            whites: vec![A2 | B2 | C2 | D2 | E5 | F2 | G2 | H2, B1 | G1, C1 | F1, A1 | H1, D1, E1],
            blacks: vec![A7 | B7 | C7 | D5 | E7 | F7 | G7 | H7, A6 | G8, C8 | F8, A8 | H8, D8, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            clock: 0,
            active: Side::White,
            enpassant: Some(Square::D6),
            history_count: 4,
        };
        test(board, String::from(fen));
    }

    #[test]
    fn case_4() {
        let fen = "r6k/p5pp/p1b2qnN/8/3Q4/2P1B3/PP4PP/R5K1 b - - 2 21";
        let board = TestBoard {
            whites: vec![A2 | B2 | C3 | G2 | H2, H6, E3, A1, D4, G1],
            blacks: vec![A7 | A6 | G7 | H7, G6, C6, A8, F6, H8],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::Black,
            enpassant: None,
            history_count: 41,
        };
        test(board, String::from(fen));
    }
}

// Trait implementations
impl Reflectable for BoardImpl {
    fn reflect(&self) -> Self {
        let pieces = self.pieces.reflect();
        let castling = self.castling.reflect();
        let active = self.active.reflect();
        let enpassant = self.enpassant.reflect();
        let history_count = self.history_count();
        let hash = hash(&pieces, &castling, active, enpassant);
        BoardImpl {
            history: History::new(hash, history_count),
            clock: self.clock,
            pieces,
            castling,
            active,
            enpassant,
            cache: CalculationCache::empty(),
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

impl Board for BoardImpl {
    fn evolve(&mut self, action: &Move) -> Discards {
        self.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: Discards) {
        self.devolve(action, discards)
    }

    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn termination_status(&mut self) -> Option<Termination> {
        self.termination_status_impl()
    }

    fn hash(&self) -> u64 {
        self.history.head()
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.pieces.locs_impl(piece)
    }

    fn king(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn side(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.pieces.whites(),
            Side::Black => self.pieces.blacks(),
        }
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        (self.pieces.side_locations(Side::White), self.pieces.side_locations(Side::Black))
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn history_count(&self) -> usize {
        self.history.position_count()
    }

    fn in_check(&mut self) -> bool {
        self.passive_control_impl().contains(self.king(self.active))
    }
}
