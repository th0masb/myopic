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
    if pgn_move.as_str() == "0-0" {
        return Ok(Castle(CastleZone::kingside(start.active())));
    } else if pgn_move.as_str() == "0-0-0" {
        return Ok(Castle(CastleZone::queenside(start.active())));
    }

    // Otherwise we need to get more involved and look through the legal moves.
    let legal = start.compute_moves(MoveComputeType::All);
    // The target square of the move.
    let target = find_matches(&pgn_move, square_regex())
        .into_iter()
        .map(|s| Square::from_string(&s))
        .collect::<Vec<_>>()
        .last()
        .map(|mv| mv.clone());

    println!("{:?}", target);

    // Functionality for checking if a piece type matches the pgn move.
    let piece_ordinal = piece_ordinal(&pgn_move);
    let piece_matches = |p: Piece| piece_ordinal == (p as usize % 6);
    let matches_pawn = piece_matches(Piece::WP);
    println!("{:?}", matches_pawn);

    // Functionality for differentiating ambiguous moves.
    let file = find_differentiating_rank_or_file(&pgn_move, file_regex());
    let rank = find_differentiating_rank_or_file(&pgn_move, rank_regex());
    let matches_start = |sq: Square| matches_square(file, rank, sq);
    println!("{:?}, {:?}", file, rank);

    // Retrieve the unique move which matches target square, piece type and
    // any differentiating information.
    legal
        .into_iter()
        .filter(|mv| match mv {
            &Move::Standard(p, s, e) => piece_matches(p) && target == Some(e) && matches_start(s),
            &Move::Promotion(s, e, p) => matches_pawn && target == Some(e) && matches_start(s),
            &Move::Enpassant(s) => matches_pawn && target == start.enpassant() && matches_start(s),
            _ => panic!(),
        })
        .map(|mv| mv.clone())
        .next()
        .ok_or(pgn_move)
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

fn piece_ordinal(pgn_move: &String) -> usize {
    let matches = find_matches(&pgn_move, piece_regex());
    let is_promotion = pgn_move.contains("=");
    let piece = if matches.is_empty() || (matches.len() == 1 && is_promotion) {
        None
    } else {
        Some(char_at(&matches[0], 0))
    };
    match piece {
        None => 0,
        Some('N') => 1,
        Some('B') => 2,
        Some('R') => 3,
        Some('Q') => 4,
        Some('K') => 5,
        _ => panic!(),
    }
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

}
