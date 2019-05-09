use itertools::Itertools;

use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square;
use crate::base::Side;
use crate::board::testutils::TestBoard;
use crate::board::Board;
use crate::pieces;
use crate::pieces::PieceRef;

use super::{PinnedPiece, PinnedSet};

struct TestCase {
    input: TestBoard,
    expected: PinnedSet,
}

fn execute_test(case: TestCase) {
    assert_eq!(case.expected, case.input.to_board().compute_pinned());
}

const EMPTY: BitBoard = BitBoard::EMPTY;

#[test]
fn case_one() {
    execute_test(TestCase {
        input: TestBoard {
            active: Side::Black,
            whites: vec![EMPTY, EMPTY, G1 | B6, G4, D8, EMPTY],
            blacks: vec![E3 | E4 | H8, G6, C5 | F2, C4 | G2, D5, D4],
            castle_rights: CastleZoneSet::ALL,
            enpassant: None,
            white_status: None,
            black_status: None,
            clock: 10,
            hash_offset: 10,
        },

        expected: (
            C5 | D5 | E4,
            vec![
                (square::constants::E4, D4 | E4 | F4 | G4),
                (square::constants::C5, B6 | C5 | D4),
                (square::constants::D5, D4 | D5 | D6 | D7 | D8),
            ],
        ),
    })
}
