use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use enum_map::{Enum, enum_map, EnumMap};
use itertools::Itertools;
use Feature::{Backward, Doubled, Isolated, Passed};
use myopic_board::{Board, Class, Move};
use crate::{BitBoard, Piece, Side};
use crate::eval::{EvalFacet, Evaluation};

const ADJACENT_FILES: [BitBoard; 8] = [
    BitBoard::FILES[1],
    BitBoard(BitBoard::FILES[0].0 | BitBoard::FILES[2].0),
    BitBoard(BitBoard::FILES[1].0 | BitBoard::FILES[3].0),
    BitBoard(BitBoard::FILES[2].0 | BitBoard::FILES[4].0),
    BitBoard(BitBoard::FILES[3].0 | BitBoard::FILES[5].0),
    BitBoard(BitBoard::FILES[4].0 | BitBoard::FILES[6].0),
    BitBoard(BitBoard::FILES[5].0 | BitBoard::FILES[7].0),
    BitBoard::FILES[6],
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Enum)]
enum Feature {
    Passed,
    Doubled,
    Isolated,
    Backward,
}

//type FeatureScores = EnumMap<Feature, (i32, i32)>;
//type FeatureCounts = EnumMap<Feature, fn(BitBoard, BitBoard) -> i32>;
type Bonus = (i32, i32);

#[derive(Clone, PartialEq)]
struct CachedEval {
    whites: BitBoard,
    blacks: BitBoard,
    mid: i32,
    end: i32,
}

pub struct PawnStructureFacet {
    connected_passer_bonus: Bonus,
    passer_rank_bonuses: [Bonus; 6],
    cache: RefCell<Vec<Option<CachedEval>>>,

}


impl Default for PawnStructureFacet {
    fn default() -> Self {
        PawnStructureFacet {
            cache: RefCell::new(vec![None; 10000]),
            connected_passer_bonus: (70, 120),
            passer_rank_bonuses: [
                // Starting rank
                (0, 0),
                (10, 10),
                (20, 20),
                (40, 60),
                (80, 100),
                // Last rank before promotion
                (160, 200),
            ]
        }
    }
}

impl EvalFacet for PawnStructureFacet {
    fn static_eval(&self, board: &Board) -> Evaluation {
        let whites = board.locs(&[Piece(Side::W, Class::P)]);
        let blacks = board.locs(&[Piece(Side::B, Class::P)]);
        let mut cache_ref = self.cache.borrow_mut();
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(whites.0);
        hasher.write_u64(blacks.0);
        let hash = hasher.finish();
        let index = (hash % cache_ref.len() as u64) as usize;
        let existing = cache_ref[index].as_ref();
        if let Some(entry) = existing {
            if entry.whites == whites && entry.blacks == blacks {
                return Evaluation::Phased { mid: entry.mid, end: entry.end }
            }
        }
        let (mut mid, mut end) = (0, 0);
        for &f in &[Passed, Doubled, Isolated, Backward] {
            let (m, e) = match f {
                Doubled => (0, 0),
                Isolated => (0, 0),
                Backward => (0, 0),
                Passed => self.evaluate_passed_pawns(whites, blacks),
            };
            mid += m;
            end += e;
        }
        cache_ref[index] = Some(CachedEval { whites, blacks, mid, end });
        Evaluation::Phased { mid, end }
    }

    fn make(&mut self, _: &Move, _: &Board) {
    }

    fn unmake(&mut self, _: &Move) {
    }
}

impl PawnStructureFacet {
    fn evaluate_passed_pawns(&self, whites: BitBoard, blacks: BitBoard) -> (i32, i32) {
        let (w_passers, b_passers) = find_passed_pawns(whites, blacks);
        let (mut mid, mut end) = (0i32, 0i32);
        // Evaluate the rank rewards for advancing
        for i in 1..7 {
            let rank = BitBoard::RANKS[i];
            let w_count = (w_passers & rank).size() as i32;
            let (w_mid, w_end) = self.passer_rank_bonuses[i - 1];
            let b_count = (b_passers & rank).size() as i32;
            let (b_mid, b_end) = self.passer_rank_bonuses[6 - i];
            mid += w_count * w_mid - b_count * b_mid;
            end += w_count * w_end - b_count * b_end;
        }
        // Evaluate the connection rewards
        let (con_mid, con_end) = self.connected_passer_bonus;
        for i in 0..7 {
            let this_file = BitBoard::FILES[i];
            let next_file = BitBoard::FILES[i + 1];
            let w_count = count_connections(this_file & w_passers, next_file & w_passers);
            let b_count = count_connections(this_file & b_passers, next_file & b_passers);
            mid += (w_count - b_count) * con_mid;
            end += (w_count - b_count) * con_end;
        }
        (mid, end)
    }
}

fn count_connections(a: BitBoard, b: BitBoard) -> i32 {
    let mut count = 0;
    for sq_a in a.into_iter() {
        for sq_b in b.into_iter() {
            if (sq_a.rank_index() as i32 - sq_b.rank_index() as i32).abs() < 2 {
                count += 1;
            }
        }
    }
    count
}

fn find_passed_pawns(whites: BitBoard, blacks: BitBoard) -> (BitBoard, BitBoard) {
    let (mut passed_w, mut passed_b) = (BitBoard::EMPTY, BitBoard::EMPTY);
    for file_index in 0..8 {
        let file = BitBoard::FILES[file_index];
        let block_files = ADJACENT_FILES[file_index] | file;

        let last_black_def = (block_files & blacks).iter().last()
            .map(|s| s.rank_index()).unwrap_or(0);
        (file & whites).iter()
            .filter(|s| s.rank_index() >= last_black_def)
            .for_each(|sq| passed_w |= sq);

        let last_white_def = (block_files & whites).iter().next()
            .map(|s| s.rank_index()).unwrap_or(10);
        (file & blacks).iter()
            .filter(|s| s.rank_index() <= last_white_def)
            .for_each(|sq| passed_b |= sq);
    }
    (passed_w, passed_b)
}

#[cfg(test)]
mod test_passed {
    use std::ops::Not;
    use crate::Reflectable;
    use super::*;
    use crate::Square::*;

    #[test]
    fn eval_1() {
        test_eval(
            (160, 200),
            !!B7,
            BitBoard::EMPTY
        )
    }

    #[test]
    fn eval_2() {
        test_eval(
            (2 * 160 + 70, 2 * 200 + 120),
            B7 | C7,
            BitBoard::EMPTY
        )
    }

    #[test]
    fn eval_3() {
        test_eval(
            (2 * 160 + 70 - 40, 2 * 200 + 120 - 60),
            B7 | C7 | F4,
            F5 | G4
        )
    }

    fn test_eval(expected: Bonus, whites: BitBoard, blacks: BitBoard) {
        let f = PawnStructureFacet::default();
        let (mid, end) = expected;
        assert_eq!(expected, f.evaluate_passed_pawns(whites, blacks));
        assert_eq!((-mid, -end), f.evaluate_passed_pawns(blacks.reflect(), whites.reflect()));
    }

    #[test]
    fn count_connections_1() {
        assert_eq!(2, count_connections(C2 | C5, B4 | B5 | B7))
    }

    #[test]
    fn find_passers_1() {
        test_find_passers(
            A4 | E3 | E5 | G4,
            B3 | B6 | D3 | D4 | E6 | F7,
            BitBoard::EMPTY,
            B3 | D3,
        )
    }

    #[test]
    fn find_passers_2() {
        test_find_passers(
            C4 | C5 | C6,
            B5 | D5,
            C5 | C6,
            BitBoard::EMPTY,
        )
    }

    fn test_find_passers(
        whites: BitBoard,
        blacks: BitBoard,
        expected_white_passers: BitBoard,
        expected_black_passers: BitBoard,
    ) {
        test_find_passers_impl(whites, blacks, expected_white_passers, expected_black_passers);
        test_find_passers_impl(
            blacks.reflect(),
            whites.reflect(),
            expected_black_passers.reflect(),
            expected_white_passers.reflect(),
        )
    }

    fn test_find_passers_impl(
        whites: BitBoard,
        blacks: BitBoard,
        expected_white_passers: BitBoard,
        expected_black_passers: BitBoard,
    ) {
        let (actual_w, actual_b) = find_passed_pawns(whites, blacks);
        assert_eq!(expected_white_passers, actual_w);
        assert_eq!(expected_black_passers, actual_b);
    }
}

//fn count_passed_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
//    let mut count = 0i32;
//    for file_index in 0..8 {
//        let file = BitBoard::FILES[file_index];
//        let adj_files = ADJACENT_FILES[file_index];
//
//        let last_black_def = (adj_files & blacks).iter().last()
//            .map(|s| s.rank_index()).unwrap_or(0);
//        count += (file & whites).iter()
//            .filter(|s| s.rank_index() >= last_black_def).count() as i32;
//
//        let last_white_def = (adj_files & whites).iter().last()
//            .map(|s| s.rank_index()).unwrap_or(10);
//        count -= (file & blacks).iter()
//            .filter(|s| s.rank_index() <= last_white_def).count() as i32;
//    }
//    count
//}
//
//fn count_doubled_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
//    let mut count = 0i32;
//    for file_index in 0..8 {
//        let file = BitBoard::FILES[file_index];
//        count += count_doubling(file & whites);
//        count -= count_doubling(file & blacks);
//    }
//    count
//}
//
//fn count_doubling(board: BitBoard) -> i32 {
//    board.iter()
//        .tuple_windows::<(_, _)>()
//        .filter(|(a, b)| b.file_index() == a.file_index() + 1)
//        .count() as i32
//}
//
//fn count_isolated_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
//    let mut count = 0i32;
//    for file_index in 0..8 {
//        let file = BitBoard::FILES[file_index];
//        let adj_files = ADJACENT_FILES[file_index];
//        if (adj_files & whites).first().is_none() && (file & whites).size() == 1 {
//            count += 1
//        }
//        if (adj_files & blacks).first().is_none() && (file & blacks).size() == 1 {
//            count -= 1
//        }
//    }
//    count
//}
//
//fn count_backward_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
//    let mut count = 0i32;
//    for file_index in 1..7 {
//        let file = BitBoard::FILES[file_index];
//        let adj_files = ADJACENT_FILES[file_index];
//        if let Some(candidate) = (file & whites).first() {
//            let rank = candidate.rank_index();
//            let adj_rank = (adj_files & whites).first()
//                .map(|s| s.rank_index()).unwrap_or(10);
//            if adj_rank > rank {
//                count += 1
//            }
//        }
//        if let Some(candidate) = (file & blacks).iter().last() {
//            let rank = candidate.rank_index();
//            let adj_rank = (adj_files & blacks).iter().last()
//                .map(|s| s.rank_index()).unwrap_or(0);
//            if adj_rank < rank {
//                count -= 1
//            }
//        }
//    }
//    count
//}
//
//#[cfg(test)]
//mod backward_test {
//    use crate::Reflectable;
//    use super::*;
//    use crate::Square::*;
//
//    fn execute_test(whites: BitBoard, blacks: BitBoard, expected: i32) {
//        assert_eq!(count_backward_pawns(whites, blacks), expected);
//        assert_eq!(count_backward_pawns(blacks.reflect(), whites.reflect()), -expected);
//    }
//
//    #[test]
//    fn case_0() {
//        execute_test(
//            A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2,
//            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
//            0
//        );
//    }
//
//    #[test]
//    fn case_1() {
//        execute_test(
//            C3 | D2 | E3 | F2 | G2 | H2,
//            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
//            1
//        );
//    }
//
//    #[test]
//    fn case_2() {
//        execute_test(
//            C3 | D2 | F2 | G2 | H2,
//            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
//            1
//        );
//    }
//
//    #[test]
//    fn case_3() {
//        execute_test(
//            A2 | C3 | D2 | F4 | G2,
//            C7 | D6 | E7 | F7 | G6 | H7,
//            1
//        );
//    }
//}
//