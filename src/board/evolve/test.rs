use crate::base::bitboard::{BitBoard};
use crate::base::castlezone::CastleZoneSet;
use crate::base::castlezone::CastleZone;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::piecetracker::PieceTracker;
use std::iter;
use crate::board::castletracker::CastleTracker;
use crate::board::hashcache::HashCache;
use crate::board::Move;
use crate::base::bitboard::constants::*;
use crate::pieces;

#[derive(Debug, Clone)]
struct TestBoard {
    whites: Vec<BitBoard>,
    blacks: Vec<BitBoard>,
    castle_rights: CastleZoneSet,
    white_status: Option<CastleZone>,
    black_status: Option<CastleZone>,
    active: Side,
    clock: usize,
    enpassant: Option<Square>,
    hash_offset: usize,
}

impl TestBoard {
    fn to_board(self) -> Board {
        let pieces = PieceTracker::new(vec![self.whites, self.blacks].iter()
            .flat_map(|x| x.into_iter()).map(|&x| x).collect());
        let castling = CastleTracker::new(self.castle_rights, self.white_status, self.black_status);
        let mut hashes = HashCache::new(0u64);
        for i in 0..self.hash_offset {
            hashes.push_head(i as u64);
        }
        let mut result = Board {
            hashes,
            pieces,
            castling,
            active: self.active,
            enpassant: self.enpassant,
            clock: self.clock,
        };
        result.update_hash();
        result
    }
}

#[derive(Debug, Clone)]
struct TestCase {
    action : Move,
    start: TestBoard,
    end: TestBoard,
}

fn check_case(test_case: TestCase) {
    let action = test_case.action.clone();
    let start = test_case.start.clone().to_board();
    let end = test_case.end.clone().to_board();

    let mut forward_subject = start.clone();
    let rev_data = forward_subject.evolve(&action);
    check_constrained_board_equality(end, forward_subject.clone());
    forward_subject.devolve(&action, rev_data);
    check_constrained_board_equality(start, forward_subject);
}

fn check_constrained_board_equality(left: Board, right: Board) {
    assert_eq!(left.clock, right.clock);
    assert_eq!(left.enpassant, right.enpassant);
    assert_eq!(left.active, right.active);
    assert_eq!(left.pieces, right.pieces);
    assert_eq!(left.castling, right.castling);
    assert_eq!(left.hashes.head(), right.hashes.head());
}


const EMPTY: BitBoard = BitBoard::EMPTY;

/// Testing white kingside castling.
#[test]
fn case_1() {
    let blacks = vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WK),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::all(),
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 20,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | F1, C2, G1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::black(),
            white_status: Some(CastleZone::WK),
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

/// Testing white queenside castling.
#[test]
fn case_2() {
    let blacks = vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WQ),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::all(),
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 20,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, H1 | D1, C2, C1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::black(),
            white_status: Some(CastleZone::WQ),
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

/// Black kingside castling test
#[test]
fn case_3() {
    let whites = vec![F2 | G2, B3, C4, A1 | H1, C2, E1];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BK),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::all(),
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 20,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, A8 | F8, D7, G8],
            castle_rights: CastleZoneSet::white(),
            white_status: None,
            black_status: Some(CastleZone::BK),
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

/// Black queenside castling test
#[test]
fn case_4() {
    let whites = vec![F2 | G2, B3, C4, A1 | H1, C2, E1];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BQ),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::all(),
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 20,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, D8 | H8, D7, C8],
            castle_rights: CastleZoneSet::white(),
            white_status: None,
            black_status: Some(CastleZone::BQ),
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}


/////
//#[test]
//fn case_x() {
//    check_case(TestCase {
//        action: Move::Standard(pieces::WR, H1, H8),
//
//        start: TestBoard {
//            whites: vec![F2 | G2, B3, ],
//            blacks: vec![],
//            castle_rights: CastleZoneSet::none(),
//            white_status: None,
//            black_status: None,
//            active: Side::White,
//            enpassant: None,
//            clock: 21,
//            hash_offset: 12,
//        },
//
//    })
//}

