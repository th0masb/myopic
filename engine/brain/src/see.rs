use enum_map::EnumMap;
use lazy_static::lazy_static;
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

lazy_static! {
    static ref ATTDEF_CONSTRAINTS: EnumMap<Square, BitBoard> =
        compute_attack_location_constraints();
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
        let mut src = self.source.into();
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
            let (new_attadef, new_xray) = self.update_xray(removed, attacker, attadef, xray);
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
        let (mut attadef, mut xray) = (BitBoard::EMPTY, BitBoard::EMPTY);
        for (piece, loc) in self.compute_potential_attdef() {
            if can_xray(piece) {
                if piece.empty_control(loc).contains(target) {
                    xray ^= loc;
                    if piece.control(loc, whites, blacks).contains(target) {
                        xray ^= loc;
                        attadef ^= loc;
                    }
                }
            } else if piece.control(loc, whites, blacks).contains(target) {
                attadef ^= loc
            }
        }
        (attadef, xray)
    }

    fn compute_potential_attdef(&self) -> impl Iterator<Item = (Piece, Square)> + '_ {
        let constraints = ATTDEF_CONSTRAINTS[self.target];
        Piece::all().flat_map(move |p| {
            (self.locs(p) & constraints)
                .into_iter()
                .map(move |loc| (p, loc))
        })
    }

    fn update_xray(
        &self,
        all_removed: BitBoard,
        last_removed: Piece,
        attadef: BitBoard,
        xray: BitBoard,
    ) -> BitBoardPair {
        // A knight being removed cannot unlock a rank/file xray
        if xray.is_empty() || last_removed.is_knight() {
            (attadef, xray)
        } else {
            let (mut whites, mut blacks) = self.board.sides();
            whites = whites - all_removed;
            blacks = blacks - all_removed;
            let (mut new_attadef, mut new_xray) = (attadef, xray);
            xray.iter().for_each(|square| {
                let p = self.board.piece(square).unwrap();
                if p.control(square, whites, blacks).contains(self.target) {
                    new_xray ^= square;
                    new_attadef ^= square;
                }
            });
            (new_attadef, new_xray)
        }
    }

    fn least_valuable_piece(&self, options: BitBoard, side: Side) -> BitBoard {
        Piece::of(side)
            .map(|p| self.locs(p))
            .find(|locs| locs.intersects(options))
            .map_or(BitBoard::EMPTY, |locs| (locs & options).least_set_bit())
    }
}

fn can_xray(piece: Piece) -> bool {
    match piece {
        Piece::WP | Piece::BP | Piece::WN | Piece::BN | Piece::WK | Piece::BK => false,
        _ => true,
    }
}

fn compute_attack_location_constraints() -> EnumMap<Square, BitBoard> {
    let mut result = EnumMap::default();
    for square in Square::iter() {
        result[square] = Piece::WQ.empty_control(square) | Piece::WN.empty_control(square)
    }
    result
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

    #[test]
    fn see_case_4() {
        execute_case(TestCase {
            board: "3r2k1/3r1ppp/2n5/8/3P4/5N2/5PPP/3R1RK1 b - - 6 27"
                .parse::<Board>()
                .unwrap(),
            expected: vec![(Square::C6, Square::D4, 1)],
        })
    }

    #[test]
    fn see_case_5() {
        execute_case(TestCase {
            board: "r3k2r/pp1b1ppp/4p3/1Bbp4/8/2N5/PPP2PPP/R1BQ1RK1 w kq - 1 11"
                .parse::<Board>()
                .unwrap(),
            expected: vec![(Square::B5, Square::D7, 0)],
        })
    }
}
