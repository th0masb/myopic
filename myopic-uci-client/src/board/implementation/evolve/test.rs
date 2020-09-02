use crate::board::implementation::test::TestBoard;
use crate::board::implementation::MutBoardImpl;
use crate::board::Move;
use myopic_core::bitboard::constants::*;
use myopic_core::bitboard::BitBoard;
use myopic_core::castlezone::CastleZone;
use myopic_core::castlezone::CastleZoneSet;
use myopic_core::pieces::Piece;
use myopic_core::{Side, Square};

#[derive(Debug, Clone)]
struct TestCase {
    action: Move,
    start: TestBoard,
    end: TestBoard,
}

fn check_case(test_case: TestCase) {
    let action = test_case.action.clone();
    let start = MutBoardImpl::from(test_case.start.clone());
    let end = MutBoardImpl::from(test_case.end.clone());

    let mut forward_subject = start.clone();
    let rev_data = forward_subject.evolve(&action);
    check_constrained_board_equality(end, forward_subject.clone());
    forward_subject.devolve(&action, rev_data);
    check_constrained_board_equality(start, forward_subject);
}

fn check_constrained_board_equality(left: MutBoardImpl, right: MutBoardImpl) {
    assert_eq!(left.clock, right.clock);
    assert_eq!(left.enpassant, right.enpassant);
    assert_eq!(left.active, right.active);
    assert_eq!(left.pieces, right.pieces);
    assert_eq!(left.castling, right.castling);
    assert_eq!(left.history.head(), right.history.head());
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
            history_count: 11,
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
            history_count: 12,
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
            history_count: 11,
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
            history_count: 12,
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
            history_count: 11,
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
            history_count: 12,
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
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_rook_taking_black_rook_removing_kingside_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::WR, Square::H1, Square::H8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_kingside_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::BR, Square::H8, Square::H1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_rook_taking_black_rook_removing_queenside_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::WR, Square::A1, Square::A8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_queenside_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::BR, Square::A8, Square::A1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_king_moving_removes_castling_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::WK, Square::E1, Square::F1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_king_moving_removes_castling_rights() {
    check_case(TestCase {
        action: Move::Standard(Piece::BK, Square::E8, Square::F8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_two() {
    check_case(TestCase {
        action: Move::Standard(Piece::WP, Square::F2, Square::F4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },

        end: TestBoard {
            whites: vec![F4 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: Some(Square::F3),
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_two() {
    check_case(TestCase {
        action: Move::Standard(Piece::BP, Square::F7, Square::F5),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },

        end: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F5, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(Square::F6),
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_one() {
    check_case(TestCase {
        action: Move::Standard(Piece::WP, Square::F2, Square::F3),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_one() {
    check_case(TestCase {
        action: Move::Standard(Piece::BP, Square::F7, Square::F6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

// On case014
#[test]
fn test_white_knight_takes_black_knight() {
    check_case(TestCase {
        action: Move::Standard(Piece::WN, Square::B3, Square::B6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_knight_takes_white_knight() {
    check_case(TestCase {
        action: Move::Standard(Piece::BN, Square::B6, Square::B3),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_bishop_takes_black_bishop() {
    check_case(TestCase {
        action: Move::Standard(Piece::WB, Square::C4, Square::D6),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_bishop_takes_white_bishop() {
    check_case(TestCase {
        action: Move::Standard(Piece::BB, Square::D6, Square::C4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_takes_black_pawn() {
    check_case(TestCase {
        action: Move::Standard(Piece::WP, Square::F2, Square::C5),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_takes_white_bishop() {
    check_case(TestCase {
        action: Move::Standard(Piece::BP, Square::C5, Square::C4),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

// next is case020
#[test]
fn test_white_queen_takes_black_queen() {
    check_case(TestCase {
        action: Move::Standard(Piece::WQ, Square::C2, Square::D7),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_queen_takes_white_queen() {
    check_case(TestCase {
        action: Move::Standard(Piece::BQ, Square::D7, Square::C2),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_king_takes_black_king() {
    check_case(TestCase {
        action: Move::Standard(Piece::WK, Square::E1, Square::E8),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_king_takes_white_king() {
    check_case(TestCase {
        action: Move::Standard(Piece::BK, Square::E8, Square::E1),

        start: TestBoard {
            whites: vec![F2 | G2, B3, C4, A1 | H1, C2, E1],
            blacks: vec![C5 | E7 | F7, B6, D6, A8 | H8, D7, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_enpassant() {
    check_case(TestCase {
        action: Move::Enpassant(Square::D5, Square::E6),

        start: TestBoard {
            whites: vec![D5 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![E5 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(Square::E6),
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_enpassant() {
    check_case(TestCase {
        action: Move::Enpassant(Square::E4, Square::D3),

        start: TestBoard {
            whites: vec![D4 | F2 | G2, EMPTY, F3, EMPTY, EMPTY, E1],
            blacks: vec![E4 | F7 | G7 | H7, EMPTY, G6, EMPTY, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: Some(Square::D3),
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_white_promotion() {
    check_case(TestCase {
        action: Move::Promotion(Square::C7, Square::B8, Piece::WQ),

        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, F3, B1, EMPTY, E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, G6, B8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::White,
            enpassant: Some(Square::D4),
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}

#[test]
fn test_black_promotion() {
    check_case(TestCase {
        action: Move::Promotion(Square::C2, Square::B1, Piece::BN),

        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, F3, B1, EMPTY, E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, G6, B8, EMPTY, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            active: Side::Black,
            enpassant: None,
            clock: 21,
            history_count: 11,
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
            history_count: 12,
        },
    })
}
