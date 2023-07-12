use std::array;
use std::cmp::max;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use crate::constants::side;

use crate::position::Position;
use crate::{Board, hash, lift, Piece, PieceMap, side, SquareMap};
use crate::board::iter;

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct StringIndexMap {
    content: Vec<String>
}

impl StringIndexMap {
    fn squares() -> StringIndexMap {
        StringIndexMap {
            content: (1usize..=8).into_iter()
                .flat_map(|r| ["h", "g", "f", "e", "d", "c", "b", "a"].map(|f| format!("{}{}", f, r)))
                .collect()
        }
    }

    fn fen_pieces() -> StringIndexMap {
        StringIndexMap {
            content: vec!["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"]
                .into_iter().map(|s| s.to_owned()).collect()
        }
    }
}

impl StringIndexMap {
    fn get(&self, s: &str) -> usize {
        self.content.iter().position(|s1| s1.as_str() == s).unwrap()
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
    let parts = SPACE.split(fen).map(|p| p.trim()).collect_vec();
    let active = if parts[1] == "w" { side::W } else { side::B };
    let curr_move = parts[5].parse::<usize>()?;
    let enpassant = if parts[3] == "-" { None } else { Some(SQUARE_MAP.get(parts[3])) };
    let clock = parts[4].parse::<usize>()?;
    let piece_boards = parse_fen_pieces(parts[0]);
    let mut piece_locs = [None; 64];
    (0usize..12).for_each(|p| iter(piece_boards[p]).for_each(|s| piece_locs[s] = Some(p)));
    let mut side_boards = [0u64, 2];
    (0usize..12).for_each(|p| side_boards[side(p)] |= piece_boards[p]);
    let rights_fn = |s: &str| parts[2].contains(s);
    let castling_rights = [rights_fn("k"), rights_fn("q"), rights_fn("K"), rights_fn("Q")];
    let mut key = hash::side(active);
    (0..4).filter(|i| castling_rights[*i]).for_each(|r| key ^= hash::corner(r));
    (0..12).for_each(|p| iter(piece_boards[p]).for_each(|s| key ^= hash::piece(p, s)));
    enpassant.map(|sq| key ^= hash::enpassant(sq));
    let prior_positions = 2 * (max(curr_move, 1) - 1) + (active as usize);
    Ok(Position {
        active, clock, enpassant, piece_boards, piece_locs, side_boards, key, prior_positions, castling_rights, history: vec![]
    })
}

fn parse_fen_pieces(fen: &str) -> PieceMap<Board> {
    let mut piece_boards = [0u64; 12];
    FEN_RANK.find_iter(fen)
        .flat_map(|m| parse_fen_rank(m.as_str()))
        .collect_vec()
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
    rank.chars().flat_map(|c| {
        if c.is_numeric() {
            vec![None; c.to_string().parse::<usize>().unwrap()]
        } else {
            vec![Some(FEN_PIECES_MAP.get(c.to_string().as_str()))]
        }
    }).collect()
}

fn parse_pgn(pgn: &str) -> Result<Position> {
    todo!()
}

fn parse_uci(uci: &str) -> Result<Position> {
    todo!()
}

#[cfg(test)]
mod test_fen {

}
