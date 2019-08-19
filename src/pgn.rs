use crate::board::{Board, BoardImpl, Move};
use crate::regex::Regex;
use std::str::FromStr;

pub fn parse_pgn<B: Board>(start: B, pgn_moves: String) -> Result<(B, Vec<Move>), String> {
    unimplemented!()
}

pub fn find_matches(source: &String, regex: &Regex) -> Vec<String> {
    regex
        .captures_iter(source)
        .map(|cap| String::from(&cap[0]))
        .collect()
}

const FILE:   &'static str = r"([a-h])";
const RANK:   &'static str = r"([1-8])";
const SQUARE: &'static str = r"([a-h][1-8])";
const PIECE:  &'static str = r"(N|B|R|Q|K)";

lazy_static! {
    static ref CASTLE: Regex = rgx(r"(0-0(-0)?)".to_owned());
    static ref PROMOTION: Regex = rgx(format!("(({}x)?{}=[NBRQ])", FILE, SQUARE));
    static ref STANDARD: Regex = rgx(format!(
        "({}?({}|{}|{})?x?{}(?!=)))",
        PIECE, RANK, FILE, SQUARE, SQUARE
    ));
    static ref MOVE: Regex = rgx(format!(
        "({}|{}|{})",
        CASTLE.as_str(),
        PROMOTION.as_str(),
        STANDARD.as_str()
    ));
}

fn rgx(pattern: String) -> Regex {
    Regex::from_str(pattern.as_ref()).unwrap()
}
