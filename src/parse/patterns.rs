use regex::Regex;
use std::str::FromStr;

pub fn space() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(SPACE.to_owned());
    }
    &RE
}

pub fn file() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(FILE.to_owned());
    }
    &RE
}

pub fn rank() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(RANK.to_owned());
    }
    &RE
}

pub fn square() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(SQUARE.to_owned());
    }
    &RE
}

pub fn int() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(INT.to_owned());
    }
    &RE
}

pub fn fen() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!(
            r"{} {} {} {} {} {}",
            fen_positions().as_str(),
            FEN_SIDE,
            FEN_RIGHTS,
            FEN_EP,
            INT,
            INT
        ));
    }
    &RE
}

pub fn fen_positions() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!(r"(({}/){{7}}{})", FEN_RNK, FEN_RNK));
    }
    &RE
}

pub fn fen_rank() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(FEN_RNK.to_owned());
    }
    &RE
}

pub fn fen_side() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!("{}", FEN_SIDE));
    }
    &RE
}

pub fn fen_enpassant() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!("{}", FEN_EP));
    }
    &RE
}

pub fn fen_rights() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!("{}", FEN_RIGHTS));
    }
    &RE
}

pub fn pgn_piece() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(PGN_PIECE.to_owned());
    }
    &RE
}

pub fn pgn_move() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex =
            rgx(format!("({}|{})", pgn_non_castle_move().as_str(), pgn_castle_move().as_str()));
    }
    &RE
}

pub fn pgn_castle_move() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(PGN_CASTLE.to_owned());
    }
    &RE
}

pub fn pgn_non_castle_move() -> &'static Regex {
    lazy_static! {
        static ref RE: Regex = rgx(format!(
            r"({}?({}|{}|{})?x?{}(=[NBRQ])?[+#]?)",
            PGN_PIECE, RANK, FILE, SQUARE, SQUARE
        ));
    }
    &RE
}

type StrConst = &'static str;

/// Standard patterns
const FILE: StrConst = r"([a-h])";
const RANK: StrConst = r"([1-8])";
const SQUARE: StrConst = r"([a-h][1-8])";
const INT: StrConst = "([0-9]+)";
const SPACE: StrConst = r"(\s+)";

/// PGN patterns
const PGN_PIECE: StrConst = r"(N|B|R|Q|K)";
const PGN_CASTLE: StrConst = r"(O-O(-O)?)";

/// FEN patterns
const FEN_RNK: StrConst = "([pnbrqkPNBRQK1-8]{1,8})";
const FEN_SIDE: StrConst = "([bw])";
const FEN_RIGHTS: StrConst = r"(-|([kqKQ]{1,4}))";
const FEN_EP: StrConst = r"(-|([a-h][1-8]))";

fn rgx(pattern: String) -> Regex {
    Regex::from_str(pattern.as_ref()).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fen_regex() {
        assert!(fen().is_match("4r1r1/pb1Q2bp/1p1Rnkp1/5p2/2P1P3/4BP2/qP2B1PP/2R3K1 w - - 1 0"));
        assert!(fen().is_match("3r4/4RRpk/5n1N/8/p1p2qPP/P1Qp1P2/1P4K1/3b4 w Qk c2 5 21"));
        assert!(fen().is_match("8/7p/4Nppk/R7/6PP/3n2K1/Pr6/8 w KkQq - 0 10"));
    }

    #[test]
    fn test_move_regex() {
        let re = pgn_move();
        assert!(re.is_match("e4"));
        assert!(re.is_match("Re1"));
        assert!(re.is_match("Nf3"));
        assert!(re.is_match("Bxf7+"));
        assert!(re.is_match("Qe4xe7#"));
        assert!(re.is_match("fxg8+"));
        assert!(re.is_match("dxc8=Q+"))
    }
}
