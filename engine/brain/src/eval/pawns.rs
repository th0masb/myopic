use crate::BitBoard;

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
