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

    // Functionality for checking if a piece type matches the pgn move.
    let piece_ordinal = piece_ordinal(&pgn_move);
    let piece_matches = |p: Piece| piece_ordinal == (p as usize % 6);
    let matches_pawn = piece_matches(Piece::WP);

    // Functionality for differentiating ambiguous moves.
    let file = find_differentiating_rank_or_file(&pgn_move, file_regex());
    let rank = find_differentiating_rank_or_file(&pgn_move, rank_regex());
    let matches_start = |sq: Square| matches_square(file, rank, sq);

    // Retrieve the unique move which matches target square, piece type and
    // any differentiating information.
    legal
        .into_iter()
        .filter(|mv| match mv {
            &Move::Standard(p, s, e) => piece_matches(p) && target == Some(e) && matches_start(s),
            &Move::Enpassant(s) => matches_pawn && target == start.enpassant() && matches_start(s),
            &Move::Promotion(s, e, p) => matches_pawn && target == Some(e) && matches_start(s),
            _ => panic!(),
        })
        .map(|mv| mv.clone())
        .next()
        .ok_or(pgn_move)
}

fn matches_square(file: Option<char>, rank: Option<char>, sq: Square) -> bool {
    match (file, rank) {
        (Some(f), Some(r)) => matches_file(f, sq) && matches_rank(r, sq),
        (None, Some(r)) => matches_rank(r, sq),
        (Some(f), None) => matches_file(f, sq),
        _ => true,
    }
}

fn matches_file(file: char, sq: Square) -> bool {
    char_at(&format!("{:?}", sq), 0) == file
}

fn matches_rank(rank: char, sq: Square) -> bool {
    char_at(&format!("{:?}", sq), 1) == rank
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
    let piece = find_matches(&pgn_move, piece_regex()).first().and_then(|s| s.chars().nth(0));
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
