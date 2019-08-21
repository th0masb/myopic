use regex::Regex;
use std::str::FromStr;

pub(super) fn move_regex() -> &'static Regex {
    &MOVE
}


pub(super) fn file_regex() -> &'static Regex {
    &FILE_RE
}

pub(super) fn rank_regex() -> &'static Regex {
    &RANK_RE
}

pub(super) fn square_regex() -> &'static Regex {
    &SQUARE_RE
}

pub(super) fn piece_regex() -> &'static Regex {
    &PIECE_RE
}

const FILE: &'static str = r"([a-h])";
const RANK: &'static str = r"([1-8])";
const SQUARE: &'static str = r"([a-h][1-8])";
const PIECE: &'static str = r"(N|B|R|Q|K)";

lazy_static! {
    static ref FILE_RE: Regex = rgx(FILE.to_owned());
    static ref RANK_RE: Regex = rgx(RANK.to_owned());
    static ref SQUARE_RE: Regex = rgx(SQUARE.to_owned());
    static ref PIECE_RE: Regex = rgx(PIECE.to_owned());
    static ref CASTLE: Regex = rgx(r"(0-0(-0)?)".to_owned());
    static ref NOT_CASTLE: Regex = rgx(format!(r"({}?({}|{}|{})?x?{}(=[NBRQ])?[+#]?)", PIECE, RANK, FILE, SQUARE, SQUARE).to_owned());
    static ref MOVE: Regex =
        rgx(format!("({}|{})", CASTLE.as_str(), NOT_CASTLE.as_str()));
}

fn rgx(pattern: String) -> Regex {
    Regex::from_str(pattern.as_ref()).unwrap()
}
