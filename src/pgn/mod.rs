use crate::board::{Board, BoardImpl, Move};
use crate::regex::Regex;
use std::str::FromStr;
use patterns::*;

mod patterns;

pub fn parse_pgn<B: Board>(start: B, pgn_moves: String) -> Result<(B, Vec<Move>), String> {
    unimplemented!()
}

pub fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex
        .captures_iter(source)
        .map(|cap| String::from(&cap[0]))
        .collect()
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
    unimplemented!()
}

fn parse_promotion<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    unimplemented!()
}

fn parse_castle<B: Board>(start: &mut B, pgn_move: String) -> Result<Move, String> {
    unimplemented!()
}
