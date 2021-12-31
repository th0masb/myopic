use std::cmp;

use myopic_board::{BitBoard, ChessBoard, Piece, Reflectable, Side, Square};

/// API function for determining whether an exchange is good on the given
/// board. The board must have a piece at both the source and target square
/// otherwise this function will panic. The pieces must be on opposing
/// sides and the quality of the return value is in relation to the side of
/// the attacker, higher is good for the attacker. Positive means a good
/// exchange, negative mean a bad one. If the pieces are on the same side the
/// result is undefined.
pub fn exchange_value<B: ChessBoard>(
    board: &B,
    source: Square,
    target: Square,
    piece_values: &[i32; 6],
) -> i32 {
    See {
        board,
        source,
        target,
        values: piece_values,
    }
    .exchange_value()
}

type BitBoardPair = (BitBoard, BitBoard);

/// Static exchange evaluator
struct See<'a, B: ChessBoard> {
    board: &'a B,
    source: Square,
    target: Square,
    values: &'a [i32; 6],
}

impl<B: ChessBoard> See<'_, B> {
    fn value(&self, piece: Piece) -> i32 {
        self.values[(piece as usize) % 6]
    }

    fn exchange_value(&self) -> i32 {
        let board = self.board;
        let first_attacker = board.piece(self.source).unwrap();
        let first_victim = board.piece(self.target).unwrap();
        let mut d = 0;
        let mut gain: [i32; 32] = [0; 32];
        gain[d] = self.value(first_victim);

        let mut attacker = first_attacker;
        let mut active = first_attacker.side();
        let mut src = self.source.lift();
        let mut removed = BitBoard::EMPTY;
        let (mut attadef, mut xray) = self.pieces_involved();
        loop {
            d += 1;
            gain[d] = self.value(attacker) - gain[d - 1];
            // TODO Can add this optimization in if we only want to know is exchange is good
            //if cmp::max(-gain[d - 1], gain[d]) < 0 {
            //    break;
            //}
            attadef ^= src;
            removed ^= src;
            let (new_attadef, new_xray) = self.update_xray(removed, attadef, xray);
            attadef = new_attadef;
            xray = new_xray;
            active = active.reflect();
            src = self.least_valuable_piece(attadef, active);
            if src.is_empty() {
                break;
            } else {
                attacker = board.piece(src.first().unwrap()).unwrap();
            }
        }
        d -= 1;
        while d > 0 {
            gain[d - 1] = -cmp::max(-gain[d - 1], gain[d]);
            d -= 1;
        }
        gain[0]
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.board.locs(&[piece])
    }

    /// Get (direct attadef, xray attadef) involved.
    fn pieces_involved(&self) -> BitBoardPair {
        let (board, target) = (self.board, self.target);
        let (whites, blacks) = board.sides();
        let zero = BitBoard::EMPTY;
        let (mut attadef, mut xray) = (zero, zero);
        for (p, loc) in Piece::all().flat_map(|p| self.locs(p).into_iter().map(move |loc| (p, loc)))
        {
            if p.control(loc, whites, blacks).contains(target) {
                attadef ^= loc;
            } else if is_slider(p) && p.control(loc, zero, zero).contains(target) {
                xray ^= loc;
            }
        }
        (attadef, xray)
    }

    fn update_xray(&self, removed: BitBoard, attadef: BitBoard, xray: BitBoard) -> BitBoardPair {
        if xray.is_empty() || self.is_knight_position(removed) {
            (attadef, xray)
        } else {
            let (mut whites, mut blacks) = self.board.sides();
            whites = whites - removed;
            blacks = blacks - removed;
            let (mut new_attadef, mut new_xray) = (attadef, xray);
            sliders()
                .iter()
                .map(|&p| (p, self.locs(p) & xray))
                .flat_map(|(p, locs)| locs.iter().map(move |loc| (p, loc)))
                .filter(|(p, loc)| p.control(*loc, whites, blacks).contains(self.target))
                .for_each(|(_, loc)| {
                    new_xray ^= loc;
                    new_attadef ^= loc;
                });
            (new_attadef, new_xray)
        }
    }

    fn is_knight_position(&self, square: BitBoard) -> bool {
        (self.board.locs(&[Piece::WN]) | self.board.locs(&[Piece::BN])).intersects(square)
    }

    fn least_valuable_piece(&self, options: BitBoard, side: Side) -> BitBoard {
        Piece::of(side)
            .map(|p| self.locs(p))
            .find(|locs| locs.intersects(options))
            .map_or(BitBoard::EMPTY, |locs| (locs & options).least_set_bit())
    }
}

fn sliders<'a>() -> &'a [Piece] {
    &[
        Piece::WB,
        Piece::WR,
        Piece::WQ,
        Piece::BB,
        Piece::BR,
        Piece::BQ,
    ]
}

fn is_slider(piece: Piece) -> bool {
    match piece {
        Piece::WP | Piece::BP | Piece::WN | Piece::BN | Piece::WK | Piece::BK => false,
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use myopic_board::{ChessBoard, Reflectable, Square};

    use crate::see::See;
    use crate::Board;

    fn dummy_values() -> [i32; 6] {
        [1, 3, 3, 5, 9, 1000]
    }

    #[derive(Clone, Debug)]
    struct TestCase<B> {
        board: B,
        expected: Vec<(Square, Square, i32)>,
    }

    impl<B: ChessBoard + Reflectable + Clone> Reflectable for TestCase<B> {
        fn reflect(&self) -> Self {
            let mut reflected_expected = Vec::new();
            for (src, targ, result) in self.expected.iter() {
                reflected_expected.push((src.reflect(), targ.reflect(), *result));
            }
            TestCase {
                board: self.board.reflect(),
                expected: reflected_expected,
            }
        }
    }

    fn execute_case<B: ChessBoard + Reflectable + Clone>(test_case: TestCase<B>) {
        execute_case_impl(test_case.clone());
        execute_case_impl(test_case.reflect())
    }

    fn execute_case_impl<B: ChessBoard>(test_case: TestCase<B>) {
        let board = test_case.board;
        for (source, target, expected_value) in test_case.expected.into_iter() {
            let see = See {
                board: &board,
                source,
                target,
                values: &dummy_values(),
            };
            assert_eq!(
                expected_value,
                see.exchange_value(),
                "Source: {:?}, target: {:?}",
                source,
                target
            )
        }
    }

    #[test]
    fn see_case_1() {
        execute_case(TestCase {
            board: "1b5k/5n2/3p2q1/2P5/8/3R4/1K1Q4/8 w KQkq - 5 20"
                .parse::<Board>()
                .unwrap(),
            expected: vec![(Square::C5, Square::D6, 0), (Square::D3, Square::D6, -2)],
        })
    }

    #[test]
    fn see_case_2() {
        execute_case(TestCase {
            board: "k7/6n1/2q1b2R/1P3P2/5N2/4Q3/8/K7 w KQkq - 10 30"
                .parse::<Board>()
                .unwrap(),
            expected: vec![
                (Square::B5, Square::C6, 9),
                (Square::C6, Square::B5, 1),
                (Square::E3, Square::E6, -3),
                (Square::F5, Square::E6, 3),
                (Square::F4, Square::E6, 3),
                (Square::H6, Square::E6, 1),
                (Square::E6, Square::F5, 1),
            ],
        })
    }

    #[test]
    fn see_case_3() {
        execute_case(TestCase {
            board: "r1n2qk1/pp5p/2ppr1pQ/4p3/8/2N4R/PPP3PP/6K1 w - - 0 3"
                .parse::<Board>()
                .unwrap(),
            expected: vec![(Square::H6, Square::H7, 1)],
        })
    }
}
