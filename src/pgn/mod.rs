use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Move::Castle;
use crate::board::{Board, BoardImpl, Move, MoveComputeType};
use crate::pieces::Piece;
use crate::regex::Regex;
use patterns::*;
use std::str::FromStr;

mod patterns;

pub fn parse_pgn<B: Board>(start: B, pgn_moves: String) -> Result<(B, Vec<Move>), String> {
    unimplemented!()
}

pub fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex.captures_iter(source).map(|cap| String::from(&cap[0])).collect()
}

fn parse_single_move<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    // If a castle move we can retrieve straight away
    if pgn_move.as_str() == "O-O" {
        return Ok(Castle(CastleZone::kingside(start.active())));
    } else if pgn_move.as_str() == "O-O-O" {
        return Ok(Castle(CastleZone::queenside(start.active())));
    }

    // Otherwise we need to get more involved and look through the legal moves.
    let legal = start.compute_moves(MoveComputeType::All);
    // The target square of the move.
    let target = find_matches(&pgn_move, square_regex())
        .into_iter()
        .map(|s| Square::from_string(&s))
        .last()
        .map(|mv| mv.clone());

    println!("{:?}", target);

    // Functionality for checking if a piece type matches the pgn move.
    let (move_piece_ordinal, promote_piece_ordinal) = piece_ordinals(&pgn_move);
    let move_piece_matches = |p: Piece| move_piece_ordinal == (p as usize % 6);
    let promote_piece_matches = |p: Piece| promote_piece_ordinal == (p as usize % 6);
    let move_matches_pawn = move_piece_matches(Piece::WP);
    println!("{:?}", move_matches_pawn);

    // Functionality for differentiating ambiguous moves.
    let file = find_differentiating_rank_or_file(&pgn_move, file_regex());
    let rank = find_differentiating_rank_or_file(&pgn_move, rank_regex());
    let matches_start = |sq: Square| matches_square(file, rank, sq);
    println!("{:?}, {:?}", file, rank);

    // Retrieve the unique move which matches target square, piece type and
    // any differentiating information.
    let matching = legal
        .into_iter()
        .filter(|mv| match mv {
            &Move::Standard(p, s, e) => {
                move_piece_matches(p) && target == Some(e) && matches_start(s)
            }
            &Move::Enpassant(s) => {
                move_matches_pawn && target == start.enpassant() && matches_start(s)
            }
            &Move::Promotion(s, e, p) => {
                move_matches_pawn
                    && target == Some(e)
                    && matches_start(s)
                    && promote_piece_matches(p)
            }
            _ => panic!(),
        })
        .map(|mv| mv.clone())
        .collect::<Vec<_>>();

    if matching.len() == 1 {
        Ok((&matching[0]).clone())
    } else {
        Err(pgn_move)
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

fn find_differentiating_rank_or_file(pgn_move: &String, re: &Regex) -> Option<char> {
    let all_matches = find_matches(pgn_move, re);
    if all_matches.len() == 1 {
        None
    } else {
        Some(char_at(&all_matches[0], 0))
    }
}

fn piece_ordinals(pgn_move: &String) -> (usize, usize) {
    let matches = find_matches(&pgn_move, piece_regex());
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
    use super::*;
    use crate::base::square::Square::*;

    fn execute_success_test(expected: Move, start_fen: &'static str, pgn: &'static str) {
        let mut board = crate::board::from_fen(start_fen).unwrap();
        let pgn_parse = parse_single_move(&mut board, String::from(pgn)).unwrap();
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
            Move::Enpassant(E5),
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
            "Rae8"
        )
    }

    #[test]
    fn case_six() {
        execute_success_test(
            Move::Standard(Piece::WR, E1, E2),
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "R1e2"
        )
    }

    #[test]
    fn case_seven() {
        execute_success_test(
            Move::Standard(Piece::BR, F3, F6),
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "R3xf6"
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
