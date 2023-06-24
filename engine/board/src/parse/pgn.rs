use regex::Regex;

use myopic_core::{
    anyhow::{anyhow, Result},
    Class, Corner, Flank, Square,
};

use crate::parse::patterns::*;
use crate::{Board, Move, MoveComputeType};

/// Extracts the moves encoded in standard pgn format starting at
/// a custom board position.
pub fn moves(start: &Board, encoded: &str) -> Result<Vec<Move>> {
    let mut mutator_board = start.clone();
    let mut dest: Vec<Move> = Vec::new();
    for evolve in pgn_move().find_iter(encoded) {
        match parse_single_move(&mut mutator_board, evolve.as_str()) {
            Ok(result) => {
                dest.push(result.clone());
                mutator_board.make(result)?;
            }
            Err(_) => return Err(anyhow!("Failed at {} in: {}", evolve.as_str(), encoded)),
        };
    }
    Ok(dest)
}

fn parse_single_move(start: &mut Board, pgn_move: &str) -> Result<Move> {
    let legal = start.compute_moves(MoveComputeType::All);
    // If a castle move we can retrieve straight away
    if pgn_move == "O-O" {
        return legal
            .iter()
            .find(|&m| match m {
                Move::Castle { corner, .. } => *corner == Corner(start.active(), Flank::K),
                _ => false,
            })
            .cloned()
            .ok_or(anyhow!("Kingside castling not available!"));
    } else if pgn_move == "O-O-O" {
        return legal
            .iter()
            .find(|&m| match m {
                Move::Castle { corner, .. } => *corner == Corner(start.active(), Flank::Q),
                _ => false,
            })
            .cloned()
            .ok_or(anyhow!("Queenside castling not available!"));
    }
    // Otherwise we need to get more involved.
    // The target square of the move.
    let target = square()
        .find_iter(pgn_move)
        .map(|m| m.as_str().parse::<Square>().unwrap())
        .last()
        .map(|mv| mv.clone());

    // Functionality for checking if a piece type matches the pgn move.
    let (move_piece_ordinal, promote_piece_ordinal) = piece_ordinals(pgn_move);
    let move_piece_matches = |p: Class| move_piece_ordinal == (p as usize);
    let promote_piece_matches = |p: Class| promote_piece_ordinal == (p as usize);
    let move_matches_pawn = move_piece_matches(Class::P);

    // Functionality for differentiating ambiguous moves.
    let file = find_differentiating_rank_or_file(pgn_move, file());
    let rank = find_differentiating_rank_or_file(pgn_move, rank());
    let matches_start = |sq: Square| matches_square(file, rank, sq);

    // Retrieve the unique move which matches target square, piece type and
    // any differentiating information.
    let matching = legal
        .into_iter()
        .filter(|mv| match mv {
            &Move::Standard { moving, from, dest, .. } => {
                move_piece_matches(moving.1) && target == Some(dest) && matches_start(from)
            }
            &Move::Enpassant { from, .. } => {
                move_matches_pawn && target == start.enpassant() && matches_start(from)
            }
            &Move::Promotion { from, dest, promoted, .. } => {
                move_matches_pawn
                    && target == Some(dest)
                    && matches_start(from)
                    && promote_piece_matches(promoted.1)
            }
            &Move::Castle { .. } => false,
        })
        .map(|mv| mv.clone())
        .collect::<Vec<_>>();

    if matching.len() == 1 {
        Ok((&matching[0]).clone())
    } else {
        Err(anyhow!("Found no move matching {}", pgn_move))
    }
}

fn matches_square(file: Option<char>, rank: Option<char>, sq: Square) -> bool {
    let sq_str = sq.to_string();
    let matches_file = |f: char| char_at(&sq_str, 0) == f;
    let matches_rank = |r: char| char_at(&sq_str, 1) == r;
    match (file, rank) {
        (Some(f), Some(r)) => matches_file(f) && matches_rank(r),
        (None, Some(r)) => matches_rank(r),
        (Some(f), None) => matches_file(f),
        _ => true,
    }
}

fn char_at(string: &String, index: usize) -> char {
    string.chars().nth(index).unwrap()
}

fn find_differentiating_rank_or_file(pgn_move: &str, re: &Regex) -> Option<char> {
    let all_matches: Vec<_> = re.find_iter(pgn_move).map(|m| m.as_str().to_owned()).collect();
    if all_matches.len() == 1 {
        None
    } else {
        Some(char_at(&all_matches[0], 0))
    }
}

fn piece_ordinals(pgn_move: &str) -> (usize, usize) {
    let matches: Vec<_> = pgn_piece().find_iter(pgn_move).map(|m| m.as_str().to_owned()).collect();
    let is_promotion = pgn_move.contains("=");
    let (move_piece, promote_piece) = if matches.is_empty() {
        (None, None)
    } else if matches.len() == 1 && is_promotion {
        (None, Some(char_at(&matches[0], 0)))
    } else {
        (Some(char_at(&matches[0], 0)), None)
    };
    let ord = |piece: Option<char>| match piece {
        None => 0,
        Some('N') => 1,
        Some('B') => 2,
        Some('R') => 3,
        Some('Q') => 4,
        Some('K') => 5,
        _ => panic!(),
    };
    (ord(move_piece), ord(promote_piece))
}

#[cfg(test)]
mod test {
    use crate::Board;

    use super::*;

    fn execute_success_test(expected_finish: &'static str, pgn: &'static str) -> Result<()> {
        let finish = expected_finish.parse::<Board>()?;
        let mut board = crate::START_FEN.parse::<Board>()?;
        for evolve in super::moves(&board, &String::from(pgn))? {
            board.make(evolve)?;
        }
        assert_eq!(finish, board);
        Ok(())
    }

    #[test]
    fn case_zero() -> Result<()> {
        execute_success_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "")
    }

    #[test]
    fn case_one() -> Result<()> {
        execute_success_test(
            "8/1P4pk/P1N2pp1/8/P3q2P/6P1/5PK1/8 w - - 6 56",
            "1. d4 d5 2. c4 c6 3. Nf3 Nf6 4. e3 Bf5 5. Nc3 e6 6. Nh4 Bg6
             7. Nxg6 hxg6 8. g3 Nbd7 9. Bg2 dxc4 10. Qe2 Nb6 11. O-O Bb4
             12. Bd2 O-O 13. Ne4 Qe7 14. Bxb4 Qxb4 15. Nc5 Rab8 16. Rfc1
            Rfd8 17. Qc2 Nfd7 18. Ne4 e5 19. a3 Qe7 20. Re1 Nf6 21. Ng5
            exd4 22. exd4 Qd6 23. Nf3 Re8 24. Re5 Nfd7 25. Ra5 a6 26. Rd1
            Rbd8 27. Bf1 Re7 28. Rg5 Qf6 29. Kg2 Rde8 30. h4 Qe6 31. a4
            Qe4 32. Qc1 f6 33. Ra5 Qe6 34. Qc2 Qe4 35. Qc1 Kh8 36. Re1 Qg4
            37. Rxe7 Rxe7 38. Bxc4 Nxc4 39. Qxc4 Qe4 40. Qb3 c5 41. dxc5
            Qc6 42. Qc3 Re2 43. b4 Ne5 44. b5 Qe4 45. c6 Nd3 46. Qxd3 Qxd3
            47. cxb7 Re8 48. bxa6 Qb3 49. Rc5 Kh7 50. Rc8 Rg8 51. Nd4 Qb6
            52. Rxg8 Kxg8 53. Kg1 Kh7 54. Nc6 Qb1+ 55. Kg2 Qe4+ 1/2-1/2",
        )
    }

    #[test]
    fn case_two() -> Result<()> {
        execute_success_test(
            "5rk1/pp2p3/3p2pb/2pP4/2q5/3b1B1P/PPn2Q2/R1NK2R1 w - - 0 28",
            "
            [Event \"F/S Return Match\"]
            [Site \"Belgrade, Serbia JUG\"]
            [Date \"1992.11.04\"]
            [Round \"29\"]
            [White \"Fischer, Robert J.\"]
            [Black \"Spassky, Boris V.\"]
            [Result \"1/2-1/2\"]

            1.d4 Nf6 2.c4 g6 3.Nc3 Bg7 4.e4 d6 5.f3 O-O 6.Be3 Nbd7 7.Qd2
            c5 8.d5 Ne5 9.h3 Nh5 10.Bf2 f5 11.exf5 Rxf5 12.g4 Rxf3 13.gxh5
            Qf8 14.Ne4 Bh6 15.Qc2 Qf4 16.Ne2 Rxf2 17.Nxf2 Nf3+ 18.Kd1 Qh4
            19.Nd3 Bf5 20.Nec1 Nd2 21.hxg6 hxg6 22.Bg2 Nxc4 23.Qf2 Ne3+
            24.Ke2 Qc4 25.Bf3 Rf8 26.Rg1 Nc2 27.Kd1 Bxd3 0-1
            ",
        )
    }
}

#[cfg(test)]
mod test_single_move {
    use crate::Board;
    use std::str::FromStr;

    use super::*;

    fn execute_success_test(
        expected: &'static str,
        start_fen: &'static str,
        pgn: &'static str,
    ) -> Result<()> {
        let mut board = start_fen.parse::<Board>()?;
        let parsed_expected = Move::from_str(expected)?;
        let pgn_parse = parse_single_move(&mut board, pgn)?;
        assert_eq!(parsed_expected, pgn_parse);
        Ok(())
    }

    #[test]
    fn case_one() -> Result<()> {
        execute_success_test(
            "sbbg4f3wn",
            "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 4",
            "Bxf3",
        )
    }

    #[test]
    fn case_two() -> Result<()> {
        execute_success_test(
            "ewe5f6f5",
            "r2qkbnr/pp1np1pp/2p5/3pPp2/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQkq f6 0 7",
            "exf6",
        )
    }

    #[test]
    fn case_three() -> Result<()> {
        execute_success_test(
            "pf7g8wnbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=N",
        )
    }

    #[test]
    fn case_four() -> Result<()> {
        execute_success_test(
            "pf7g8wqbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=Q",
        )
    }

    #[test]
    fn case_five() -> Result<()> {
        execute_success_test(
            "sbra8e8-",
            "r5r1/ppqkb1pp/2p1pn2/3p2B1/3P4/2NB1Q1P/PPP2PP1/4RRK1 b - - 8 14",
            "Rae8",
        )
    }

    #[test]
    fn case_six() -> Result<()> {
        execute_success_test(
            "swre1e2-",
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "R1e2",
        )
    }

    #[test]
    fn case_seven() -> Result<()> {
        execute_success_test(
            "sbrf3f6wb",
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "R3xf6",
        )
    }

    #[test]
    fn case_eight() -> Result<()> {
        execute_success_test(
            "sbne4f2wp",
            "5r2/ppqkb1p1/2p1pr1p/3p4/3Pn2P/2NBR3/PPP1RPP1/7K b - - 1 21",
            "Nxf2+",
        )
    }

    #[test]
    fn case_nine() -> Result<()> {
        execute_success_test(
            "sbrf8f1wb",
            "5r2/ppqkb1p1/2p1p2p/3p4/P2P3P/2N1R3/1PP3P1/5B1K b - - 0 24",
            "Rf8xf1#",
        )
    }

    #[test]
    fn case_ten() -> Result<()> {
        execute_success_test(
            "cwk",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R3K2R w KQkq - 1 13",
            "O-O",
        )
    }

    #[test]
    fn case_eleven() -> Result<()> {
        execute_success_test(
            "cbq",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R4RK1 b kq - 2 13",
            "O-O-O",
        )
    }

    #[test]
    fn case_12() {
        execute_success_test(
            "pg7f8wqbr",
            "rnbq1rk1/p4pPp/2pbp3/8/3P4/8/Pp2BPPP/R1BQK1NR w KQ - 0 12",
            "gxf8=Q+",
        )
        .unwrap()
    }

    #[test]
    fn case_13() {
        execute_success_test(
            "sbqd8f8wq",
            "rnbq1Qk1/p4p1p/2pbp3/8/3P4/8/Pp2BPPP/R1BQK1NR b KQ - 0 12",
            "Qxf8",
        )
        .unwrap()
    }
}
