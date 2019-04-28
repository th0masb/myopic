use std::iter;

use crate::base::bitboard::BitBoard;
use crate::base::bitboard::constants::*;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Side;
use crate::base::square;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::castletracker::CastleTracker;
use crate::board::hashcache::HashCache;
use crate::board::Move;
use crate::board::piecetracker::PieceTracker;
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

#[test]
fn test_white_kingside_castling() {
    let blacks = vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WK),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::ALL,
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
            castle_rights: CastleZoneSet::BLACK,
            white_status: Some(CastleZone::WK),
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_queenside_castling() {
    let blacks = vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8];
    check_case(TestCase {
        action: Move::Castle(CastleZone::WQ),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: blacks.clone(),
            castle_rights: CastleZoneSet::ALL,
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
            castle_rights: CastleZoneSet::BLACK,
            white_status: Some(CastleZone::WQ),
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_kingside_castling() {
    let whites = vec![F2 | G2, B3, C4, A1 | H1, C2, E1];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BK),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
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
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_queenside_castling() {
    let whites = vec![F2 | G2, B3, C4, A1 | H1, C2, E1];
    check_case(TestCase {
        action: Move::Castle(CastleZone::BQ),

        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
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
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BQ),
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 12,
        },
    })
}


#[test]
fn test_white_rook_taking_black_rook_removing_kingside_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::WR, square::constants::H1, square::constants::H8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H8, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8, D7, E8],
            castle_rights: CastleZoneSet::WQ | CastleZoneSet::BQ,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_kingside_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::BR, square::constants::H8, square::constants::H1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H1, D7, E8],
            castle_rights: CastleZoneSet::WQ | CastleZoneSet::BQ,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_rook_taking_black_rook_removing_queenside_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::WR, square::constants::A1, square::constants::A8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A8 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, H8, D7, E8],
            castle_rights: CastleZoneSet::WK | CastleZoneSet::BK,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_queenside_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::BR, square::constants::A8, square::constants::A1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A1 | H8, D7, E8],
            castle_rights: CastleZoneSet::WK | CastleZoneSet::BK,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_king_moving_removes_castling_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::WK, square::constants::E1, square::constants::F1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, F1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::BLACK,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 22,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_king_moving_removes_castling_rights() {
    check_case(TestCase {
        action: Move::Standard(pieces::BK, square::constants::E8, square::constants::F8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, F8],
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 22,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_two() {
    check_case(TestCase {
        action: Move::Standard(pieces::WP, square::constants::F2, square::constants::F4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F4 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: Some(square::constants::F3),
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_two() {
    check_case(TestCase {
        action: Move::Standard(pieces::BP, square::constants::F7, square::constants::F5),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F5, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(square::constants::F6),
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_one() {
    check_case(TestCase {
        action: Move::Standard(pieces::WP, square::constants::F2, square::constants::F3),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F3 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_one() {
    check_case(TestCase {
        action: Move::Standard(pieces::BP, square::constants::F7, square::constants::F6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F6, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

// On case014
#[test]
fn test_white_knight_takes_black_knight() {
    check_case(TestCase {
        action: Move::Standard(pieces::WN, square::constants::B3, square::constants::B6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B6, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, EMPTY, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_knight_takes_white_knight() {
    check_case(TestCase {
        action: Move::Standard(pieces::BN, square::constants::B6, square::constants::B3),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B3, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_bishop_takes_black_bishop() {
    check_case(TestCase {
        action: Move::Standard(pieces::WB, square::constants::C4, square::constants::D6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, D6, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, EMPTY, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_bishop_takes_white_bishop() {
    check_case(TestCase {
        action: Move::Standard(pieces::BB, square::constants::D6, square::constants::C4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, EMPTY, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, C4, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_pawn_takes_black_pawn() {
    check_case(TestCase {
        action: Move::Standard(pieces::WP, square::constants::F2, square::constants::C5),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![C5 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_pawn_takes_white_bishop() {
    check_case(TestCase {
        action: Move::Standard(pieces::BP, square::constants::C5, square::constants::C4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, EMPTY, A1 | H1, C2, E1],
            blacks: vec![C4 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

// next is case020
#[test]
fn test_white_queen_takes_black_queen() {
    check_case(TestCase {
        action: Move::Standard(pieces::WQ, square::constants::C2, square::constants::D7),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, D7, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_queen_takes_white_queen() {
    check_case(TestCase {
        action: Move::Standard(pieces::BQ, square::constants::D7, square::constants::C2),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, EMPTY, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, C2, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_king_takes_black_king() {
    check_case(TestCase {
        action: Move::Standard(pieces::WK, square::constants::E1, square::constants::E8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E8],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, EMPTY],
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_king_takes_white_king() {
    check_case(TestCase {
        action: Move::Standard(pieces::BK, square::constants::E8, square::constants::E1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, EMPTY],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E1],
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_enpassant() {
    check_case(TestCase {
        action: Move::Enpassant(square::constants::D5),

        start: TestBoard {
            whites: vec![D5 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![E5 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(square::constants::E6),
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![E6 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_enpassant() {
    check_case(TestCase {
        action: Move::Enpassant(square::constants::E4),

        start: TestBoard {
            whites: vec![D4 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![E4 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: Some(square::constants::D3),
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![D3 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_white_promotion() {
    check_case(TestCase {
        action: Move::Promotion(square::constants::C7, square::constants::B8, pieces::WQ),

        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, F3, B1, EMPTY, E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, G6, B8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(square::constants::D4),
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, F3, B1, B8, E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}

#[test]
fn test_black_promotion() {
    check_case(TestCase {
        action: Move::Promotion(square::constants::C2, square::constants::B1, pieces::BN),

        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, F3, B1, EMPTY, E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, G6, B8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            hash_offset: 11,
        },

        end: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![F7 | G7 | H7, B1, G6, B8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 0,
            hash_offset: 12,
        },
    })
}
