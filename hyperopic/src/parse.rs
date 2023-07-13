

use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{Board, hash, lift, Piece, piece_side, PieceMap};
use crate::board::iter;

use crate::position::Position;

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_fen(s)
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
    use crate::constants::{side};
    let parts = SPACE.split(fen).map(|p| p.trim()).collect_vec();
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
        active, clock, enpassant, piece_boards,
        piece_locs, side_boards, key, castling_rights, history: vec![]
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

fn parse_pgn(_pgn: &str) -> Result<Position> {
    todo!()
}

fn parse_uci(_uci: &str) -> Result<Position> {
    todo!()
}

#[cfg(test)]
mod test_fen {
    use crate::{board, side, square_map, zobrist_hash};
    use crate::constants::*;
    use crate::constants::square::*;
    use crate::position::Position;

    #[test]
    fn case_1() {
        assert_eq!(
            "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19".parse::<Position>().unwrap(),
            Position {
                piece_locs: square_map!(
                    A3, B2, C4, E4, F2, G2, H2 => piece::WP,
                    C3, D4 => piece::WN,
                    E2 => piece::WB,
                    D1, F1 => piece::WR,
                    D2 => piece::WQ,
                    G1 => piece::WK,
                    A6, B7, D6, E6, F7, G6, H6 => piece::BP,
                    E7 => piece::BN,
                    C8, G7 => piece::BB,
                    A8, D8 => piece::BR,
                    C7 => piece::BQ,
                    G8 => piece::BK
                ),
                key: zobrist_hash!(
                    A3, B2, C4, E4, F2, G2, H2 => piece::WP,
                    C3, D4 => piece::WN,
                    E2 => piece::WB,
                    D1, F1 => piece::WR,
                    D2 => piece::WQ,
                    G1 => piece::WK,
                    A6, B7, D6, E6, F7, G6, H6 => piece::BP,
                    E7 => piece::BN,
                    C8, G7 => piece::BB,
                    A8, D8 => piece::BR,
                    C7 => piece::BQ,
                    G8 => piece::BK
                ),
                piece_boards: [
                    board!(A3, B2, C4, E4, F2, G2, H2),
                    board!(C3, D4),
                    board!(E2),
                    board!(D1, F1),
                    board!(D2),
                    board!(G1),
                    board!(A6, B7, D6, E6, F7, G6, H6),
                    board!(E7),
                    board!(C8, G7),
                    board!(A8, D8),
                    board!(C7),
                    board!(G8),
                ],
                side_boards: [
                    board!(A3, B2, C3, C4, D1, D2, D4, E2, E4, F1, F2, G1, G2, H2),
                    board!(A6, A8, B7, C7, C8, D6, D8, E6, E7, F7, G6, G7, G8, H6),
                ],
                castling_rights: [false, false, false, false],
                active: side::W,
                enpassant: None,
                clock: 3,
                history: vec![],
            }
        )
    }
}
