use anyhow::Result;
use enum_map::enum_map;

use myopic_core::Square::*;
use myopic_core::*;

use crate::enumset::EnumSet;
use crate::imp::test::TestBoard;
use crate::imp::Board;
use crate::{ChessBoard, Move};
use crate::imp::rights::Rights;

#[derive(Debug, Clone)]
struct TestCase {
    mv: &'static str,
    start: TestBoard,
    end: TestBoard,
}

fn check_case(test_case: TestCase) -> Result<()> {
    let start = Board::from(test_case.start.clone());
    let end = Board::from(test_case.end.clone());
    let action = Move::from(test_case.mv, start.hash())?;

    let mut forward_subject = start.clone();
    forward_subject.make(action)?;
    check_constrained_board_equality(end, forward_subject.clone());
    forward_subject.unmake()?;
    check_constrained_board_equality(start, forward_subject);
    Ok(())
}

fn check_constrained_board_equality(left: Board, right: Board) {
    assert_eq!(left.clock, right.clock);
    assert_eq!(left.enpassant, right.enpassant);
    assert_eq!(left.active, right.active);
    assert_eq!(left.pieces, right.pieces);
    assert_eq!(left.rights, right.rights);
    assert_eq!(left.position_count(), right.position_count());
    assert_eq!(left.half_move_clock(), right.half_move_clock());
    assert_eq!(left.hash(), right.hash());
}

const EMPTY: BitBoard = BitBoard::EMPTY;

#[test]
fn test_white_kingside_castling() -> Result<()> {
    let blacks = vec![A7 | C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8];
    check_case(TestCase {
        mv: "cwk",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: blacks.clone(),
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 20,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | F1, !!C2, !!G1],
            blacks: blacks.clone(),
            castle_rights: Rights::side(Side::B),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_queenside_castling() -> Result<()> {
    let blacks = vec![A7 | C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8];
    check_case(TestCase {
        mv: "cwq",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: blacks.clone(),
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 20,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, H1 | D1, !!C2, !!C1],
            blacks: blacks.clone(),
            castle_rights: Rights::side(Side::B),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_kingside_castling() -> Result<()> {
    let whites = vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1];
    check_case(TestCase {
        mv: "cbk",
        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 20,
            history_count: 11,
        },
        end: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, !!B6, !!D6, A8 | F8, !!D7, !!G8],
            castle_rights: Rights::side(Side::W),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_queenside_castling() -> Result<()> {
    let whites = vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1];
    check_case(TestCase {
        mv: "cbq",
        start: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 20,
            history_count: 11,
        },
        end: TestBoard {
            whites: whites.clone(),
            blacks: vec![A7 | C5 | E7 | F7, !!B6, !!D6, D8 | H8, !!D7, !!C8],
            castle_rights: Rights::side(Side::W),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_rook_taking_black_rook_removing_kingside_rights() -> Result<()> {
    check_case(TestCase {
        mv: "swrh1h8br",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H8, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, !!A8, !!D7, !!E8],
            castle_rights: Rights::flank(Flank::Q),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_kingside_rights() -> Result<()> {
    check_case(TestCase {
        mv: "sbrh8h1wr",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, !!A1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H1, !!D7, !!E8],
            castle_rights: Rights::flank(Flank::Q),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_rook_taking_black_rook_removing_queenside_rights() -> Result<()> {
    check_case(TestCase {
        mv: "swra1a8br",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A8 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, !!H8, !!D7, !!E8],
            castle_rights: Rights::flank(Flank::K),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_rook_taking_white_rook_removing_queenside_rights() -> Result<()> {
    check_case(TestCase {
        mv: "sbra8a1wr",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, !!H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A1 | H8, !!D7, !!E8],
            castle_rights: Rights::flank(Flank::K),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_king_moving_removes_castling_rights() -> Result<()> {
    check_case(TestCase {
        mv: "swke1f1-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!F1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights(enum_map! {
                Side::B => EnumSet::all(),
                Side::W => EnumSet::empty()
            }),
            active: Side::B,
            enpassant: None,
            clock: 22,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_king_moving_removes_castling_rights() -> Result<()> {
    check_case(TestCase {
        mv: "sbke8f8-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!F8],
            castle_rights: Rights(enum_map! {
                Side::B => EnumSet::empty(),
                Side::W => EnumSet::all()
            }),
            active: Side::W,
            enpassant: None,
            clock: 22,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_two() -> Result<()> {
    check_case(TestCase {
        mv: "swpf2f4-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F4 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: Some(F3),
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_two() -> Result<()> {
    check_case(TestCase {
        mv: "sbpf7f5-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F5, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: Some(F6),
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_moves_forward_one() -> Result<()> {
    check_case(TestCase {
        mv: "swpf2f3-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F3 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_moves_forward_one() -> Result<()> {
    check_case(TestCase {
        mv: "sbpf7f6-",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F6, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

// On case014
#[test]
fn test_white_knight_takes_black_knight() -> Result<()> {
    check_case(TestCase {
        mv: "swnb3b6bn",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B6, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, EMPTY, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_knight_takes_white_knight() -> Result<()> {
    check_case(TestCase {
        mv: "sbnb6b3wn",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B3, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_bishop_takes_black_bishop() -> Result<()> {
    check_case(TestCase {
        mv: "swbc4d6bb",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!D6, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, EMPTY, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_bishop_takes_white_bishop() -> Result<()> {
    check_case(TestCase {
        mv: "sbbd6c4wb",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, EMPTY, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!C4, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_pawn_takes_black_pawn() -> Result<()> {
    check_case(TestCase {
        mv: "swpf2c5bp",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![C5 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_pawn_takes_white_bishop() -> Result<()> {
    check_case(TestCase {
        mv: "sbpc5c4wb",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, EMPTY, A1 | H1, !!C2, !!E1],
            blacks: vec![C4 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

// next is case020
#[test]
fn test_white_queen_takes_black_queen() -> Result<()> {
    check_case(TestCase {
        mv: "swqc2d7bq",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!D7, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_queen_takes_white_queen() -> Result<()> {
    check_case(TestCase {
        mv: "sbqd7c2wq",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, EMPTY, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!C2, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_king_takes_black_king() -> Result<()> {
    check_case(TestCase {
        mv: "swke1e8bk",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E8],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, EMPTY],
            castle_rights: Rights::empty(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_king_takes_white_king() -> Result<()> {
    check_case(TestCase {
        mv: "sbke8e1wk",
        start: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, !!E1],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, !!B3, !!C4, A1 | H1, !!C2, EMPTY],
            blacks: vec![C5 | E7 | F7, !!B6, !!D6, A8 | H8, !!D7, !!E1],
            castle_rights: Rights::empty(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_enpassant() -> Result<()> {
    check_case(TestCase {
        mv: "ewd5e6e5",
        start: TestBoard {
            whites: vec![D5 | F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![E5 | F7 | G7 | H7, EMPTY, !!G6, EMPTY, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: Some(E6),
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![E6 | F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![F7 | G7 | H7, EMPTY, !!G6, EMPTY, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_enpassant() -> Result<()> {
    check_case(TestCase {
        mv: "ebe4d3d4",
        start: TestBoard {
            whites: vec![D4 | F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![E4 | F7 | G7 | H7, EMPTY, !!G6, EMPTY, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: Some(D3),
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![D3 | F7 | G7 | H7, EMPTY, !!G6, EMPTY, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_white_promotion() -> Result<()> {
    check_case(TestCase {
        mv: "pc7b8wqbr",
        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, !!F3, !!B1, EMPTY, !!E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, !!G6, !!B8, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: Some(Square::D4),
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![F2 | G2, EMPTY, !!F3, !!B1, !!B8, !!E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, !!G6, EMPTY, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}

#[test]
fn test_black_promotion() -> Result<()> {
    check_case(TestCase {
        mv: "pc2b1bn-",
        start: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![C2 | F7 | G7 | H7, EMPTY, !!G6, !!B8, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::B,
            enpassant: None,
            clock: 21,
            history_count: 11,
        },
        end: TestBoard {
            whites: vec![C7 | F2 | G2, EMPTY, !!F3, EMPTY, EMPTY, !!E1],
            blacks: vec![F7 | G7 | H7, !!B1, !!G6, !!B8, EMPTY, !!E8],
            castle_rights: Rights::all(),
            active: Side::W,
            enpassant: None,
            clock: 0,
            history_count: 12,
        },
    })
}
