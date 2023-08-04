use lazy_static::lazy_static;
use std::{array, cmp};

use crate::board::{control, iter, union_boards};
use crate::constants::{
    class, create_piece, in_board, intersects, lift, piece_class, piece_side, reflect_side, side,
};
use crate::eval::material::PieceValues;
use crate::position::Position;
use crate::{Board, Class, Piece, Side, Square, SquareMap};

pub fn exchange_value(
    board: &Position,
    source: Square,
    target: Square,
    piece_values: &PieceValues,
) -> i32 {
    See { board, source, target, values: piece_values }.exchange_value()
}

type BoardPair = (Board, Board);

/// Static exchange evaluator
struct See<'a> {
    board: &'a Position,
    source: Square,
    target: Square,
    values: &'a PieceValues,
}

lazy_static! {
    static ref ATTDEF_CONSTRAINTS: SquareMap<Board> = compute_attack_location_constraints();
}

impl See<'_> {
    fn value(&self, piece: Piece) -> i32 {
        self.values[piece_class(piece)]
    }

    fn exchange_value(&self) -> i32 {
        let board = self.board;
        let first_attacker = board.piece_locs[self.source].unwrap();
        let first_victim = board.piece_locs[self.target].unwrap();
        let mut d = 0;
        let mut gain: [i32; 32] = [0; 32];
        gain[d] = self.value(first_victim);

        let mut attacker = first_attacker;
        let mut active = piece_side(first_attacker);
        let mut src = lift(self.source);
        let mut removed = 0u64;
        let pieces_involved = self.pieces_involved();
        let (mut attadef, mut xray) = pieces_involved;
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
            active = reflect_side(active);
            src = self.least_valuable_piece(attadef, active);
            if src == 0 {
                break;
            } else {
                let attacker_square = src.trailing_zeros() as Square;
                match board.piece_locs[attacker_square] {
                    None => panic!("See error: {} -> {} on {}", self.source, self.target, board),
                    Some(p) => attacker = p,
                }
            }
        }
        d -= 1;
        while d > 0 {
            gain[d - 1] = -cmp::max(-gain[d - 1], gain[d]);
            d -= 1;
        }
        gain[0]
    }

    /// Get (direct attadef, xray attadef) involved.
    fn pieces_involved(&self) -> BoardPair {
        let target = self.target;
        let occupied = union_boards(&self.board.side_boards);
        let (mut attadef, mut xray) = (0u64, 0u64);
        for (piece, loc) in self.compute_potential_attdef() {
            if can_xray(piece_class(piece)) {
                if in_board(control(piece, loc, 0), target) {
                    let toggle = lift(loc);
                    xray ^= toggle;
                    if in_board(control(piece, loc, occupied), target) {
                        xray ^= toggle;
                        attadef ^= toggle;
                    }
                }
            } else if in_board(control(piece, loc, occupied), target) {
                attadef ^= lift(loc);
            }
        }
        (attadef, xray)
    }

    fn compute_potential_attdef(&self) -> impl Iterator<Item = (Piece, Square)> + '_ {
        let constraints = ATTDEF_CONSTRAINTS[self.target];
        self.board
            .piece_boards
            .iter()
            .enumerate()
            .flat_map(move |(p, &locs)| iter(locs & constraints).map(move |loc| (p, loc)))
    }

    fn update_xray(
        &self,
        all_removed: Board,
        last_removed: Piece,
        attadef: Board,
        xray: Board,
    ) -> BoardPair {
        // A knight being removed cannot unlock a rank/file xray
        if xray == 0 || piece_class(last_removed) == class::N {
            (attadef, xray)
        } else {
            let occupied = union_boards(&self.board.side_boards) & !all_removed;
            let (mut new_attadef, mut new_xray) = (attadef, xray);
            iter(xray).for_each(|square| {
                let p = self.board.piece_locs[square].unwrap();
                if in_board(control(p, square, occupied), self.target) {
                    let toggle = lift(square);
                    new_xray ^= toggle;
                    new_attadef ^= toggle;
                }
            });
            (new_attadef, new_xray)
        }
    }

    fn least_valuable_piece(&self, options: Board, side: Side) -> Board {
        (0..6)
            .map(|class| create_piece(side, class))
            .map(|p| self.board.piece_boards[p])
            .find(|locs| intersects(options, *locs))
            .map_or(0, |locs| lift((locs & options).trailing_zeros() as usize))
    }
}

fn can_xray(class: Class) -> bool {
    2 <= class && class < 5
}

fn compute_attack_location_constraints() -> SquareMap<Board> {
    let queen = create_piece(side::W, class::Q);
    let knight = create_piece(side::W, class::N);
    array::from_fn(|sq| control(queen, sq, 0) | control(knight, sq, 0))
}

#[cfg(test)]
mod test {
    use super::See;
    use crate::{Square, Symmetric};

    use crate::constants::square::*;
    use crate::constants::{class, corner, create_piece, reflect_square, side};
    use crate::eval::material::PieceValues;
    use crate::moves::Move;
    use crate::node::TreeNode;
    use crate::position::Position;

    fn dummy_values() -> PieceValues {
        [1, 3, 3, 5, 9, 1000]
    }

    #[derive(Clone, Debug)]
    struct TestCase {
        board: Position,
        expected: Vec<(Square, Square, i32)>,
    }

    impl Symmetric for TestCase {
        fn reflect(&self) -> Self {
            let mut reflected_expected = Vec::new();
            for (src, targ, result) in self.expected.iter() {
                reflected_expected.push((reflect_square(*src), reflect_square(*targ), *result));
            }
            TestCase { board: self.board.reflect(), expected: reflected_expected }
        }
    }

    fn execute_case(test_case: TestCase) {
        execute_case_impl(test_case.clone());
        execute_case_impl(test_case.reflect())
    }

    fn execute_case_impl(test_case: TestCase) {
        let board = test_case.board;
        for (source, target, expected_value) in test_case.expected.into_iter() {
            let see = See { board: &board, source, target, values: &dummy_values() };
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
            board: "1b5k/5n2/3p2q1/2P5/8/3R4/1K1Q4/8 w KQkq - 5 20".parse::<Position>().unwrap(),
            expected: vec![(C5, D6, 0), (D3, D6, -2)],
        })
    }

    #[test]
    fn see_case_2() {
        execute_case(TestCase {
            board: "k7/6n1/2q1b2R/1P3P2/5N2/4Q3/8/K7 w KQkq - 10 30".parse::<Position>().unwrap(),
            expected: vec![
                (B5, C6, 9),
                (C6, B5, 1),
                (E3, E6, -3),
                (F5, E6, 3),
                (F4, E6, 3),
                (H6, E6, 1),
                (E6, F5, 1),
            ],
        })
    }

    #[test]
    fn see_case_3() {
        execute_case(TestCase {
            board: "r1n2qk1/pp5p/2ppr1pQ/4p3/8/2N4R/PPP3PP/6K1 w - - 0 3"
                .parse::<Position>()
                .unwrap(),
            expected: vec![(H6, H7, 1)],
        })
    }

    #[test]
    fn see_case_4() {
        execute_case(TestCase {
            board: "3r2k1/3r1ppp/2n5/8/3P4/5N2/5PPP/3R1RK1 b - - 6 27".parse::<Position>().unwrap(),
            expected: vec![(C6, D4, 1)],
        })
    }

    #[test]
    fn see_case_5() {
        execute_case(TestCase {
            board: "r3k2r/pp1b1ppp/4p3/1Bbp4/8/2N5/PPP2PPP/R1BQ1RK1 w kq - 1 11"
                .parse::<Position>()
                .unwrap(),
            expected: vec![(B5, D7, 0)],
        })
    }

    #[test]
    fn see_case_6() {
        execute_case(TestCase {
            board: "r1bq1rk1/1pp1npb1/3p2p1/pQBPp1Pp/2P1P2P/2N2P1B/PP6/R3K2R b KQ - 0 14"
                .parse::<Position>()
                .unwrap(),
            expected: vec![(C8, H3, 0)],
        })
    }

    #[test]
    fn see_case_7() {
        let initial_position =
            "r1bqk2r/1ppnnpb1/3p2p1/p2Pp1Pp/2P1P2P/2N1BP2/PP6/R2QKBNR w KQkq a6 0 11";
        let mut node: TreeNode = initial_position.parse::<Position>().unwrap().into();
        let moves = vec![
            Move::Normal {
                moving: create_piece(side::W, class::B),
                from: F1,
                dest: H3,
                capture: None,
            },
            Move::Castle { corner: corner::BK },
            Move::Normal {
                moving: create_piece(side::W, class::Q),
                from: D1,
                dest: B3,
                capture: None,
            },
            Move::Normal {
                moving: create_piece(side::B, class::N),
                from: D7,
                dest: C5,
                capture: None,
            },
            Move::Normal {
                moving: create_piece(side::W, class::Q),
                from: B3,
                dest: B5,
                capture: None,
            },
            Move::Null,
            Move::Normal {
                moving: create_piece(side::W, class::B),
                from: E3,
                dest: C5,
                capture: Some(create_piece(side::B, class::N)),
            },
        ];
        moves.into_iter().for_each(|m| node.make(m).unwrap());
        assert_eq!(0, node.see(C8, H3));
    }
}
