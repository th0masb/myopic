use std::str::FromStr;

use anyhow::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::board::iter;
use crate::{hash, lift, piece_side, Board, Piece, PieceMap};

use crate::position::Position;

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_fen(s)
    }
}

pub struct StringIndexMap {
    content: Vec<String>,
}

impl StringIndexMap {
    pub fn squares() -> StringIndexMap {
        StringIndexMap {
            content: (1usize..=8)
                .into_iter()
                .flat_map(|r| {
                    ["h", "g", "f", "e", "d", "c", "b", "a"].map(|f| format!("{}{}", f, r))
                })
                .collect(),
        }
    }

    pub fn sides() -> StringIndexMap {
        StringIndexMap { content: vec!["w", "b"].into_iter().map(|s| s.to_owned()).collect() }
    }

    pub fn corners() -> StringIndexMap {
        StringIndexMap {
            content: vec!["wk", "wq", "bk", "bq"].into_iter().map(|s| s.to_owned()).collect(),
        }
    }

    pub fn pieces() -> StringIndexMap {
        StringIndexMap {
            content: vec!["wp", "wn", "wb", "wr", "wq", "wk", "bp", "bn", "bb", "br", "bq", "bk"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    pub fn fen_pieces() -> StringIndexMap {
        StringIndexMap {
            content: vec!["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        }
    }
}

impl StringIndexMap {
    pub fn get_op<S: AsRef<str>>(&self, s: S) -> Option<usize> {
        self.content.iter().position(|s1| s1.as_str() == s.as_ref())
    }

    pub fn get<S: AsRef<str>>(&self, s: S) -> usize {
        self.get_op(s).unwrap()
    }
}

lazy_static! {
    static ref FILE: Regex = r"([a-h])".parse().unwrap();
    static ref RANK: Regex = r"([1-8])".parse().unwrap();
    static ref SQUARE: Regex = r"([a-h][1-8])".parse().unwrap();
    static ref SPACE: Regex = r"(\s+)".parse().unwrap();
    static ref FEN_RANK: Regex = r"([pnbrqkPNBRQK1-8]{1,8})".parse().unwrap();
    static ref TEST: Regex = format!("{}", SQUARE.as_str()).as_str().parse().unwrap();
    static ref SQUARE_MAP: StringIndexMap = StringIndexMap::squares();
    static ref FEN_PIECES_MAP: StringIndexMap = StringIndexMap::fen_pieces();
}

fn parse_fen(fen: &str) -> Result<Position> {
    use crate::constants::side;
    let parts = SPACE.split(fen).map(|p| p.trim()).collect::<Vec<_>>();
    let active = if parts[1] == "w" { side::W } else { side::B };
    let enpassant = if parts[3] == "-" { None } else { Some(SQUARE_MAP.get(parts[3])) };
    let clock = parts[4].parse::<usize>()?;
    let piece_boards = parse_fen_pieces(parts[0]);
    let mut piece_locs = [None; 64];
    (0..12).for_each(|p| iter(piece_boards[p]).for_each(|s| piece_locs[s] = Some(p)));
    let mut side_boards = [0u64; 2];
    (0..12).for_each(|p| side_boards[piece_side(p)] |= piece_boards[p]);
    let rights_fn = |s: &str| parts[2].contains(s);
    let castling_rights = [rights_fn("K"), rights_fn("Q"), rights_fn("k"), rights_fn("q")];
    let mut key = if active == side::W { 0u64 } else { hash::black_move() };
    (0..4).filter(|i| castling_rights[*i]).for_each(|r| key ^= hash::corner(r));
    (0..12).for_each(|p| iter(piece_boards[p]).for_each(|s| key ^= hash::piece(p, s)));
    enpassant.map(|sq| key ^= hash::enpassant(sq));
    Ok(Position {
        active,
        clock,
        enpassant,
        piece_boards,
        piece_locs,
        side_boards,
        key,
        castling_rights,
        history: vec![],
    })
}

fn parse_fen_pieces(fen: &str) -> PieceMap<Board> {
    let mut piece_boards = [0u64; 12];
    FEN_RANK
        .find_iter(fen)
        .flat_map(|m| parse_fen_rank(m.as_str()))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(i, op)| {
            if let Some(p) = op {
                piece_boards[p] |= lift(i)
            }
        });
    piece_boards
}

fn parse_fen_rank(rank: &str) -> Vec<Option<Piece>> {
    rank.chars()
        .flat_map(|c| {
            if c.is_numeric() {
                vec![None; c.to_string().parse::<usize>().unwrap()]
            } else {
                vec![Some(FEN_PIECES_MAP.get(c.to_string().as_str()))]
            }
        })
        .collect()
}

fn parse_pgn(_pgn: &str) -> Result<Position> {
    todo!()
}

fn parse_uci(_uci: &str) -> Result<Position> {
    todo!()
}

#[cfg(test)]
mod test_fen {
    use crate::constants::square::*;
    use crate::constants::*;
    use crate::position::Position;
    use crate::square_map;

    #[test]
    fn case_1() {
        assert_eq!(
            "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19"
                .parse::<Position>()
                .unwrap(),
            Position::new(
                side::W,
                None,
                3,
                [false, false, false, false],
                square_map!(
                    A3, B2, C4, E4, F2, G2, H2 => Some(piece::WP),
                    C3, D4 => Some(piece::WN),
                    E2 => Some(piece::WB),
                    D1, F1 => Some(piece::WR),
                    D2 => Some(piece::WQ),
                    G1 => Some(piece::WK),
                    A6, B7, D6, E6, F7, G6, H6 => Some(piece::BP),
                    E7 => Some(piece::BN),
                    C8, G7 => Some(piece::BB),
                    A8, D8 => Some(piece::BR),
                    C7 => Some(piece::BQ),
                    G8 => Some(piece::BK)
                )
            )
        )
    }
}
