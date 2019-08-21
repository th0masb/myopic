use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Move::Castle;
use crate::board::{Board, BoardImpl, Move, MoveComputeType};
use crate::regex::Regex;
use patterns::*;
use std::str::FromStr;
use crate::pieces::Piece;

mod patterns;

pub fn parse_pgn<B: Board>(start: B, pgn_moves: String) -> Result<(B, Vec<Move>), String> {
    unimplemented!()
}

pub fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex.captures_iter(source).map(|cap| String::from(&cap[0])).collect()
}

fn parse_single_move<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    let trimmed = pgn_move.trim().to_owned();
    if castle_regex().is_match(trimmed.as_str()) {
        parse_castle(start, trimmed)
    } else if promotion_regex().is_match(trimmed.as_str()) {
        parse_promotion(start, trimmed)
    } else if standard_regex().is_match(trimmed.as_str()) {
        parse_standard(start, trimmed)
    } else {
        Err(trimmed)
    }
}

fn parse_standard<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    let legal = start.compute_moves(MoveComputeType::All);
    let square_matches: Vec<_> = find_matches(&pgn_move, square_regex())
        .into_iter()
        .map(|s| Square::from_string(&s))
        .collect();
    if square_matches.len() == 0 {
        Err(pgn_move)
    } else if square_matches.len() == 2 {
        // Rare case of two squares specified we can search straight away.
        let (start, end) = (square_matches[0], square_matches[1]);
        let matched_move = legal.iter().find(|&mv| match mv {
            &Move::Standard(_, s, e) => s == start && e == end,
            _ => false,
        });
        matched_move.map(|mv| mv.clone()).ok_or(pgn_move)
    } else {
        let end = square_matches[0];
        let file = find_differentiating_rank_or_file(&pgn_move, file_regex());
        let ranks = find_differentiating_rank_or_file(&pgn_move, rank_regex());
        let piece_ordinal = piece_ordinal(&pgn_move);
        let piece_matches = |p: Piece| piece_ordinal == (p as usize % 6);


        unimplemented!()
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
    let piece = find_matches(&pgn_move, piece_regex())
        .first()
        .and_then(|s| s.chars().nth(0));
    match piece {
        None => 0,
        Some('N') => 1,
        Some('B') => 2,
        Some('R') => 3,
        Some('Q') => 4,
        Some('K') => 5,
        _ => panic!()
    }
}

fn parse_promotion<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    unimplemented!()
}

fn parse_castle<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    if pgn_move.as_str() == "0-0" {
        Ok(Castle(CastleZone::kingside(start.active())))
    } else {
        Ok(Castle(CastleZone::queenside(start.active())))
    }
}
