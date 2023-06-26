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

type FeatureScores = EnumMap<Feature, (i32, i32)>;
type FeatureCounts = EnumMap<Feature, fn(BitBoard, BitBoard) -> i32>;

pub struct PawnStructureFacet {
    scores: FeatureScores,
    counts: FeatureCounts,
}

impl Default for PawnStructureFacet {
    fn default() -> Self {
        PawnStructureFacet {
            scores: enum_map! {
                Passed => (80, 200),
                Doubled => (-70, -75),
                Isolated => (-60, -30),
                Backward => (-15, 0),
            },
            counts: enum_map! {
                Passed => count_passed_pawns,
                Doubled => count_doubled_pawns,
                Isolated => count_isolated_pawns,
                Backward => count_backward_pawns,
            }
        }
    }
}

impl EvalFacet for PawnStructureFacet {
    fn static_eval(&self, board: &Board) -> Evaluation {
        let whites = board.locs(&[Piece(Side::W, Class::P)]);
        let blacks = board.locs(&[Piece(Side::B, Class::P)]);
        let (mut mid, mut end) = (0, 0);
        for &f in &[Passed, Doubled, Isolated, Backward] {
            let count = self.counts[f](whites, blacks);
            let (m, e) = self.scores[f];
            mid += count * m;
            end += count * e;
        }
        Evaluation::Phased { mid, end }
    }

    fn make(&mut self, _: &Move, _: &Board) {
    }

    fn unmake(&mut self, _: &Move) {
    }
}

fn count_passed_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
    let mut count = 0i32;
    for file_index in 0..8 {
        let file = BitBoard::FILES[file_index];
        let adj_files = ADJACENT_FILES[file_index];

        let last_black_def = (adj_files & blacks).iter().last()
            .map(|s| s.rank_index()).unwrap_or(0);
        count += (file & whites).iter()
            .filter(|s| s.rank_index() >= last_black_def).count() as i32;

        let last_white_def = (adj_files & whites).iter().last()
            .map(|s| s.rank_index()).unwrap_or(10);
        count -= (file & blacks).iter()
            .filter(|s| s.rank_index() <= last_white_def).count() as i32;
    }
    count
}

fn count_doubled_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
    let mut count = 0i32;
    for file_index in 0..8 {
        let file = BitBoard::FILES[file_index];
        count += count_doubling(file & whites);
        count -= count_doubling(file & blacks);
    }
    count
}

fn count_doubling(board: BitBoard) -> i32 {
    board.iter()
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| b.file_index() == a.file_index() + 1)
        .count() as i32
}

fn count_isolated_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
    let mut count = 0i32;
    for file_index in 0..8 {
        let file = BitBoard::FILES[file_index];
        let adj_files = ADJACENT_FILES[file_index];
        if (adj_files & whites).first().is_none() && (file & whites).size() == 1 {
            count += 1
        }
        if (adj_files & blacks).first().is_none() && (file & blacks).size() == 1 {
            count -= 1
        }
    }
    count
}

fn count_backward_pawns(whites: BitBoard, blacks: BitBoard) -> i32 {
    let mut count = 0i32;
    for file_index in 1..7 {
        let file = BitBoard::FILES[file_index];
        let adj_files = ADJACENT_FILES[file_index];
        if let Some(candidate) = (file & whites).first() {
            let rank = candidate.rank_index();
            let adj_rank = (adj_files & whites).first()
                .map(|s| s.rank_index()).unwrap_or(10);
            if adj_rank > rank {
                count += 1
            }
        }
        if let Some(candidate) = (file & blacks).iter().last() {
            let rank = candidate.rank_index();
            let adj_rank = (adj_files & blacks).iter().last()
                .map(|s| s.rank_index()).unwrap_or(0);
            if adj_rank < rank {
                count -= 1
            }
        }
    }
    count
}

#[cfg(test)]
mod backward_test {
    use crate::Reflectable;
    use super::*;
    use crate::Square::*;

    fn execute_test(whites: BitBoard, blacks: BitBoard, expected: i32) {
        assert_eq!(count_backward_pawns(whites, blacks), expected);
        assert_eq!(count_backward_pawns(blacks.reflect(), whites.reflect()), -expected);
    }

    #[test]
    fn case_0() {
        execute_test(
            A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2,
            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
            0
        );
    }

    #[test]
    fn case_1() {
        execute_test(
            C3 | D2 | E3 | F2 | G2 | H2,
            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
            1
        );
    }

    #[test]
    fn case_2() {
        execute_test(
            C3 | D2 | F2 | G2 | H2,
            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
            1
        );
    }

    #[test]
    fn case_3() {
        execute_test(
            A2 | C3 | D2 | F4 | G2,
            C7 | D6 | E7 | F7 | G6 | H7,
            1
        );
    }
}
