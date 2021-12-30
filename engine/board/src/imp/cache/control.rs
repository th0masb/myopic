use myopic_core::*;

use crate::imp::Board;

impl Board {
    pub fn passive_control(&mut self) -> BitBoard {
        match &self.cache.passive_control {
            Some(x) => *x,
            None => {
                let result = self.compute_passive_control(self.active.reflect());
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
    fn compute_passive_control(&self, side: Side) -> BitBoard {
        let pieces = &self.pieces;
        let (whites, blacks) = match side {
            Side::White => (
                pieces.whites(),
                pieces.blacks() - pieces.king_location(Side::Black),
            ),
            Side::Black => (
                pieces.whites() - pieces.king_location(Side::White),
                pieces.blacks(),
            ),
        };
        let locs = |piece: Piece| pieces.locs(piece);
        let control = |piece: Piece, square: Square| piece.control(square, whites, blacks);
        Piece::of(side)
            .flat_map(|p| locs(p).into_iter().map(move |sq| control(p, sq)))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{BitBoard, constants::*, Side};
    use crate::enumset::EnumSet;
    use crate::imp::Board;
    use crate::imp::test::TestBoard;

    struct TestCase {
        board: TestBoard,
        side: Side,
        expected_control: BitBoard,
    }

    fn execute_test(case: TestCase) {
        assert_eq!(
            case.expected_control,
            Board::from(case.board).compute_passive_control(case.side)
        );
    }

    fn get_test_board() -> TestBoard {
        TestBoard {
            whites: vec![
                A2 | B3 | C2 | D2 | E4 | F2 | G2 | H2,
                F3,
                B2 | F1,
                A1,
                D1,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C7 | D7 | E5 | F7 | G7 | H5,
                C6 | G8,
                C8,
                A8 | H8,
                F6,
                E8,
            ],
            castle_rights: EnumSet::all(),
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

        execute_test(TestCase {
            board: get_test_board(),
            side: Side::White,
            expected_control,
        })
    }

    #[test]
    fn test_black_control() {
        let expected_control: BitBoard = vec![
            A7, A6, A5, B8, B7, B6, B4, C8, C6, D8, D7, D6, D4, E7, E6, E5, F8, F7, F6, F5, F4, F3,
            G8, G7, G6, G5, G4, H7, H6, H5, H4,
        ]
            .into_iter()
            .collect();

        execute_test(TestCase {
            board: get_test_board(),
            side: Side::Black,
            expected_control,
        })
    }
}
