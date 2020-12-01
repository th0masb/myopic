use crate::parse::patterns::*;
use crate::regex::Regex;
use crate::Move::Castle;
use crate::{Move, MoveComputeType, MutBoard};
use myopic_core::castlezone::CastleZone;
use myopic_core::pieces::Piece;
use myopic_core::Square;

/// Extracts the moves encoded in standard pgn format contained in
/// a single string.
pub fn pgn(moves: &String) -> Result<Vec<Move>, String> {
    return partial_pgn(&crate::start_position(), moves);
}

/// Extracts the moves encoded in standard pgn format starting at
/// a custom board position.
pub fn partial_pgn<B: MutBoard>(start: &B, moves: &String) -> Result<Vec<Move>, String> {
    let mut mutator_board = start.clone();
    let mut dest: Vec<Move> = Vec::new();
    for evolve in pgn_move().find_iter(moves) {
        match parse_single_move(&mut mutator_board, evolve.as_str()) {
            Ok(result) => dest.push(result.clone()),
            Err(_) => return Err(format!("Failed at {} in: {}", evolve.as_str(), moves)),
        };
        mutator_board.evolve(dest.last().unwrap());
    }
    Ok(dest)
}

fn parse_single_move<B: MutBoard>(start: &mut B, pgn_move: &str) -> Result<Move, String> {
    // If a castle move we can retrieve straight away
    if pgn_move == "O-O" {
        return Ok(Castle(CastleZone::kingside(start.active())));
    } else if pgn_move == "O-O-O" {
        return Ok(Castle(CastleZone::queenside(start.active())));
    }

    // Otherwise we need to get more involved and look through the legal moves.
    let legal = start.compute_moves(MoveComputeType::All);
    // The target square of the move.
    let target = square()
        .find_iter(pgn_move)
        .map(|m| Square::from_string(m.as_str()).unwrap())
        .last()
        .map(|mv| mv.clone());

    // Functionality for checking if a piece type matches the pgn move.
    let (move_piece_ordinal, promote_piece_ordinal) = piece_ordinals(pgn_move);
    let move_piece_matches = |p: Piece| move_piece_ordinal == (p as usize % 6);
    let promote_piece_matches = |p: Piece| promote_piece_ordinal == (p as usize % 6);
    let move_matches_pawn = move_piece_matches(Piece::WP);

    // Functionality for differentiating ambiguous moves.
    let file = find_differentiating_rank_or_file(pgn_move, file());
    let rank = find_differentiating_rank_or_file(pgn_move, rank());
    let matches_start = |sq: Square| matches_square(file, rank, sq);

    // Retrieve the unique move which matches target square, piece type and
    // any differentiating information.
    let matching = legal
        .into_iter()
        .filter(|mv| match mv {
            &Move::Standard(p, s, e) => {
                move_piece_matches(p) && target == Some(e) && matches_start(s)
            }
            &Move::Enpassant(s, _) => {
                move_matches_pawn && target == start.enpassant() && matches_start(s)
            }
            &Move::Promotion(s, e, p) => {
                move_matches_pawn
                    && target == Some(e)
                    && matches_start(s)
                    && promote_piece_matches(p)
            }
            _ => false,
        })
        .map(|mv| mv.clone())
        .collect::<Vec<_>>();

    if matching.len() == 1 {
        Ok((&matching[0]).clone())
    } else {
        Err(pgn_move.to_owned())
    }
}

fn matches_square(file: Option<char>, rank: Option<char>, sq: Square) -> bool {
    let lower = format!("{:?}", sq).to_lowercase();
    let matches_file = |f: char| char_at(&lower, 0) == f;
    let matches_rank = |r: char| char_at(&lower, 1) == r;
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
    fn execute_success_test(expected_finish: &'static str, pgn: &'static str) {
        let finish = crate::fen_position(expected_finish).unwrap();
        let mut board = crate::start_position();
        for evolve in super::partial_pgn(&board, &String::from(pgn)).unwrap() {
            board.evolve(&evolve);
        }
        assert_eq!(finish, board);
    }

    #[test]
    fn case_zero() {
        execute_success_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "")
    }

    #[test]
    fn case_one() {
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
        );
    }
    #[test]
    fn case_two() {
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
        );
    }
}

#[cfg(test)]
mod test_single_move {
    use super::*;
    use myopic_core::Square::*;

    fn execute_success_test(expected: Move, start_fen: &'static str, pgn: &'static str) {
        let mut board = crate::fen_position(start_fen).unwrap();
        let pgn_parse = parse_single_move(&mut board, &String::from(pgn)).unwrap();
        assert_eq!(expected, pgn_parse);
    }

    #[test]
    fn case_one() {
        execute_success_test(
            Move::Standard(Piece::BB, G4, F3),
            "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 4",
            "Bxf3",
        )
    }

    #[test]
    fn case_two() {
        execute_success_test(
            Move::Enpassant(E5, Square::F6),
            "r2qkbnr/pp1np1pp/2p5/3pPp2/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQkq f6 0 7",
            "exf6",
        )
    }

    #[test]
    fn case_three() {
        execute_success_test(
            Move::Promotion(F7, G8, Piece::WN),
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=N",
        )
    }

    #[test]
    fn case_four() {
        execute_success_test(
            Move::Promotion(F7, G8, Piece::WQ),
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=Q",
        )
    }

    #[test]
    fn case_five() {
        execute_success_test(
            Move::Standard(Piece::BR, A8, E8),
            "r5r1/ppqkb1pp/2p1pn2/3p2B1/3P4/2NB1Q1P/PPP2PP1/4RRK1 b - - 8 14",
            "Rae8",
        )
    }

    #[test]
    fn case_six() {
        execute_success_test(
            Move::Standard(Piece::WR, E1, E2),
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "R1e2",
        )
    }

    #[test]
    fn case_seven() {
        execute_success_test(
            Move::Standard(Piece::BR, F3, F6),
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "R3xf6",
        )
    }

    #[test]
    fn case_eight() {
        execute_success_test(
            Move::Standard(Piece::BN, E4, F2),
            "5r2/ppqkb1p1/2p1pr1p/3p4/3Pn2P/2NBR3/PPP1RPP1/7K b - - 1 21",
            "Nxf2+",
        )
    }

    #[test]
    fn case_nine() {
        execute_success_test(
            Move::Standard(Piece::BR, F8, F1),
            "5r2/ppqkb1p1/2p1p2p/3p4/P2P3P/2N1R3/1PP3P1/5B1K b - - 0 24",
            "Rf8xf1#",
        )
    }

    #[test]
    fn case_ten() {
        execute_success_test(
            Move::Castle(CastleZone::WK),
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R3K2R w KQkq - 1 13",
            "O-O",
        )
    }
    #[test]
    fn case_eleven() {
        execute_success_test(
            Move::Castle(CastleZone::BQ),
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R4RK1 b kq - 2 13",
            "O-O-O",
        )
    }
}
