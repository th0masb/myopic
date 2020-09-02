use crate::implementation::MutBoardImpl;
use myopic_core::bitboard::BitBoard;
use myopic_core::pieces::Piece;
use myopic_core::reflectable::Reflectable;
use myopic_core::{Side, Square};

impl MutBoardImpl {
    pub fn passive_control_impl(&mut self) -> BitBoard {
        match &self.cache.passive_control {
            Some(x) => *x,
            None => {
                let result = self.compute_control(self.active.reflect());
                self.cache.passive_control = Some(result);
                result
            }
        }
    }

    /// Computes the total area of control on the board for a given side. Note
    /// that the passive king is treated as invisible so that if it is in
    /// check it cannot create it's own escape squares by blocking the
    /// control ray of an attacking slider. TODO Improve efficiency by
    /// treated all pawns as a block
    fn compute_control(&self, side: Side) -> BitBoard {
        let pieces = &self.pieces;
        let (whites, blacks) = match side {
            Side::White => (pieces.whites(), pieces.blacks() - pieces.king_location(Side::Black)),
            Side::Black => (pieces.whites() - pieces.king_location(Side::White), pieces.blacks()),
        };
        let locs = |piece: Piece| pieces.locs_impl(piece);
        let control = |piece: Piece, square: Square| piece.control(square, whites, blacks);
        Piece::on_side(side)
            .flat_map(|p| locs(p).into_iter().map(move |sq| control(p, sq)))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::implementation::test::TestBoard;
    use crate::implementation::MutBoardImpl;
    use myopic_core::bitboard::constants::*;
    use myopic_core::bitboard::BitBoard;
    use myopic_core::castlezone::CastleZoneSet;
    use myopic_core::Side;

    struct TestCase {
        board: TestBoard,
        side: Side,
        expected_control: BitBoard,
    }

    fn execute_test(case: TestCase) {
        assert_eq!(
            case.expected_control,
            MutBoardImpl::from(case.board).compute_control(case.side)
        );
    }

    fn get_test_board() -> TestBoard {
        TestBoard {
            whites: vec![A2 | B3 | C2 | D2 | E4 | F2 | G2 | H2, F3, B2 | F1, A1, D1, E1],
            blacks: vec![A7 | B7 | C7 | D7 | E5 | F7 | G7 | H5, C6 | G8, C8, A8 | H8, F6, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            enpassant: None,
            active: Side::White,
            clock: 20,
            history_count: 20,
        }
    }

    #[test]
    fn test_white_control() {
        let expected_control: BitBoard = vec![
            A1, A2, A3, A4, A6, B1, B3, B5, C1, C2, C3, C4, D1, D2, D3, D4, D5, E1, E2, E3, E5, F1,
            F2, F3, F5, G1, G2, G3, G5, H2, H3, H4,
        ]
        .into_iter()
        .collect();

        execute_test(TestCase { board: get_test_board(), side: Side::White, expected_control })
    }

    #[test]
    fn test_black_control() {
        let expected_control: BitBoard = vec![
            A7, A6, A5, B8, B7, B6, B4, C8, C6, D8, D7, D6, D4, E7, E6, E5, F8, F7, F6, F5, F4, F3,
            G8, G7, G6, G5, G4, H7, H6, H5, H4,
        ]
        .into_iter()
        .collect();

        execute_test(TestCase { board: get_test_board(), side: Side::Black, expected_control })
    }
}
