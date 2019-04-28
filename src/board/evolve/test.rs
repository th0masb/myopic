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
use crate::base::square::constants::F2;
use crate::base::square::constants::G2;
use crate::base::square::constants::B3;
use crate::base::square::constants::C4;
use crate::base::square::constants::A1;
use crate::base::square::constants::H1;
use crate::base::square::constants::C2;
use crate::base::square::constants::E1;
use crate::base::square::constants::A7;
use crate::base::square::constants::C5;
use crate::base::square::constants::E7;
use crate::base::square::constants::F7;
use crate::base::square::constants::B6;
use crate::base::square::constants::D6;
use crate::base::square::constants::A8;
use crate::base::square::constants::H8;
use crate::base::square::constants::D7;
use crate::base::square::constants::E8;
use crate::base::square::constants::G1;
use crate::base::square::constants::F1;
use crate::base::square::constants::C1;
use crate::base::square::constants::D1;
use crate::base::square::constants::H2;
use crate::base::square::constants::F8;
use crate::base::square::constants::G8;
use crate::base::square::constants::D8;
use crate::base::square::constants::C8;

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
    let blacks = vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), A8 | H8, D7.lift(), E8.lift()];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WK),

        start: TestBoard {
            whites: vec![F2 | G2, B3.lift(), C4.lift(), A1 | H1, C2.lift(), E1.lift()],
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
            whites: vec![F2 | G2, B3.lift(), C4.lift(), A1 | F1, C2.lift(), G1.lift()],
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
    let blacks = vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), A8 | H8, D7.lift(), E8.lift()];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WQ),

        start: TestBoard {
            whites: vec![F2 | G2, B3.lift(), C4.lift(), A1 | H1, C2.lift(), E1.lift()],
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
            whites: vec![F2 | G2, B3.lift(), C4.lift(), H1 | D1, C2.lift(), C1.lift()],
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
    let whites = vec![F2 | G2, B3.lift(), C4.lift(), A1 | H1, C2.lift(), E1.lift()];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BK),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), A8 | H8, D7.lift(), E8.lift()],
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
            blacks: vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), A8 | F8, D7.lift(), G8.lift()],
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
    let whites = vec![F2 | G2, B3.lift(), C4.lift(), A1 | H1, C2.lift(), E1.lift()];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BQ),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), A8 | H8, D7.lift(), E8.lift()],
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
            blacks: vec![A7 | C5 | E7 | F7, B6.lift(), D6.lift(), D8 | H8, D7.lift(), C8.lift()],
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
