use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use Move::{Enpassant, Normal, Null, Promote};

use crate::board::iter;
use crate::{hash, lift, piece_side, Board, Piece, PieceMap, Square, Class, square_file, square_rank, piece_class};
use crate::constants::class;
use crate::moves::{Move, Moves};

use crate::position::Position;

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_fen(s).or_else(|_| {
            let mut position = Position::default();
            for m in PGN_MOVE.find_iter(s) {
                let m = parse_pgn_move(&position, m.as_str())?;
                position.make(m)?
            }
            Ok(position)
        })
    }
}

pub struct StringIndexMap {
    content: Vec<String>,
}

const FILE_CHARS: [char; 8] = ['h', 'g', 'f', 'e', 'd', 'c', 'b', 'a'];
const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl StringIndexMap {
    pub fn squares() -> StringIndexMap {
        StringIndexMap {
            content: RANK_CHARS
                .into_iter()
                .flat_map(|r| FILE_CHARS.iter().map(move |f| format!("{}{}", f, r)))
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

    pub fn pgn_pieces() -> StringIndexMap {
        StringIndexMap {
            content: vec!["", "N", "B", "R", "Q", "K"].into_iter().map(|s| s.to_owned()).collect(),
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
    // Index maps
    static ref SQUARE_MAP: StringIndexMap = StringIndexMap::squares();
    static ref FEN_PIECES_MAP: StringIndexMap = StringIndexMap::fen_pieces();

    // Patterns
    static ref SPACE: Regex = r"(\s+)".parse().unwrap();
    static ref FEN_RANK: Regex = r"([pnbrqkPNBRQK1-8]{1,8})".parse().unwrap();

    static ref FILE: Regex = r"([a-h])".parse().unwrap();
    static ref RANK: Regex = r"([1-8])".parse().unwrap();
    static ref INT: Regex = "([0-9]+)".parse().unwrap();
    static ref SQUARE: Regex = r"([a-h][1-8])".parse().unwrap();

    static ref PGN_PIECE: Regex = r"(N|B|R|Q|K)".parse().unwrap();
    static ref PGN_CASTLE: Regex = r"(O-O(-O)?)".parse().unwrap();

    static ref PGN_NORMAL_MOVE: Regex = format!(
        r"({}?({}|{}|{})?x?{}(=[NBRQ])?[+#]?)",
        PGN_PIECE.as_str(),
        RANK.as_str(),
        FILE.as_str(),
        SQUARE.as_str(),
        SQUARE.as_str(),
    ).as_str().parse().unwrap();

    static ref PGN_MOVE: Regex = format!(
        "({}|{})",
        PGN_NORMAL_MOVE.as_str(),
        PGN_CASTLE.as_str()
    ).as_str().parse().unwrap();
}

fn parse_pgn_move(position: &Position, input: &str) -> Result<Move> {
    let moves = position.moves(&Moves::All);

    if PGN_CASTLE.is_match(input) {
        return moves.iter().find(|&m| {
            if let Move::Castle { corner } = m {
                *corner % 2 == if input == "O-O" { 0 } else { 1 }
            } else {
                false
            }
        }).cloned().ok_or(anyhow!("{} not legal", input))
    }

    let target = SQUARE
        .find_iter(input)
        .map(|m| SQUARE_MAP.get(m.as_str()))
        .last()
        .map(|mv| mv.clone());

    let (move_piece_class, promote_piece_class) = parse_pgn_classes(input);
    let move_piece_matches = |p: Class| move_piece_class == p;
    let promote_piece_matches = |p: Class| promote_piece_class == p;
    let move_matches_pawn = move_piece_matches(class::P);

    let file = parse_extra_rank_file(&FILE, input);
    let rank = parse_extra_rank_file(&RANK, input);
    let matches_start = |sq: Square| matches_square(file, rank, sq);

    moves.into_iter().filter(|m| {
        match m {
            Null | Move::Castle { .. } => false,
            Enpassant { from, .. } => {
                move_matches_pawn && target == position.enpassant && matches_start(*from)
            },
            Normal { moving, from, dest, .. } => {
                move_piece_matches(piece_class(*moving))
                    && target == Some(*dest) && matches_start(*from)
            },
            Promote { from, dest, promoted, .. } => {
                move_matches_pawn &&
                    target == Some(*dest) &&
                    matches_start(*from) &&
                    promote_piece_matches(piece_class(*promoted))
            },
        }
    }).next().ok_or(anyhow!("No move matching {}", input))
}

fn matches_square(file: Option<char>, rank: Option<char>, square: Square) -> bool {
    let sq_file = FILE_CHARS[square_file(square)];
    let sq_rank = RANK_CHARS[square_rank(square)];
    file.map(|f| f == sq_file).unwrap_or(true) &&
        rank.map(|r| r == sq_rank).unwrap_or(true)
}

fn parse_extra_rank_file(re: &Regex, input: &str) -> Option<char> {
    re.find_iter(input)
        .map(|m| m.as_str().to_owned())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip(1)
        .last()
        .and_then(|s| s.chars().nth(0))
}

fn parse_pgn_classes(input: &str) -> (Class, Class) {
    let matches: Vec<_> = PGN_PIECE.find_iter(input).map(|m| m.as_str().to_owned()).collect();
    let is_promotion = input.contains("=");
    let piece = matches.get(0).and_then(|s| s.chars().nth(0));
    let (move_piece, promote_piece) = if is_promotion { (None, piece) } else { (piece, None) };
    (parse_class(move_piece), parse_class(promote_piece))
}

fn parse_class(piece: Option<char>) -> Class {
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

#[cfg(test)]
mod test_single_move {
    use crate::Board;
    use std::str::FromStr;

    use super::*;

    fn execute_success_test(
        expected: &'static str,
        start_fen: &'static str,
        pgn: &'static str,
    ) -> Result<()> {
        let mut board = start_fen.parse::<Position>()?;
        let parsed_expected = Move::from_str(expected)?;
        let pgn_parse = parse_pgn_move(&mut board, pgn)?;
        assert_eq!(parsed_expected, pgn_parse);
        Ok(())
    }

    #[test]
    fn case_one() -> Result<()> {
        execute_success_test(
            "sbbg4f3wn",
            "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 4",
            "Bxf3",
        )
    }

    #[test]
    fn case_two() -> Result<()> {
        execute_success_test(
            "ewe5f6f5",
            "r2qkbnr/pp1np1pp/2p5/3pPp2/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQkq f6 0 7",
            "exf6",
        )
    }

    #[test]
    fn case_three() -> Result<()> {
        execute_success_test(
            "pf7g8wnbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=N",
        )
    }

    #[test]
    fn case_four() -> Result<()> {
        execute_success_test(
            "pf7g8wqbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "fxg8=Q",
        )
    }

    #[test]
    fn case_five() -> Result<()> {
        execute_success_test(
            "sbra8e8-",
            "r5r1/ppqkb1pp/2p1pn2/3p2B1/3P4/2NB1Q1P/PPP2PP1/4RRK1 b - - 8 14",
            "Rae8",
        )
    }

    #[test]
    fn case_six() -> Result<()> {
        execute_success_test(
            "swre1e2-",
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "R1e2",
        )
    }

    #[test]
    fn case_seven() -> Result<()> {
        execute_success_test(
            "sbrf3f6wb",
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "R3xf6",
        )
    }

    #[test]
    fn case_eight() -> Result<()> {
        execute_success_test(
            "sbne4f2wp",
            "5r2/ppqkb1p1/2p1pr1p/3p4/3Pn2P/2NBR3/PPP1RPP1/7K b - - 1 21",
            "Nxf2+",
        )
    }

    #[test]
    fn case_nine() -> Result<()> {
        execute_success_test(
            "sbrf8f1wb",
            "5r2/ppqkb1p1/2p1p2p/3p4/P2P3P/2N1R3/1PP3P1/5B1K b - - 0 24",
            "Rf8xf1#",
        )
    }

    #[test]
    fn case_ten() -> Result<()> {
        execute_success_test(
            "cwk",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R3K2R w KQkq - 1 13",
            "O-O",
        )
    }

    #[test]
    fn case_eleven() -> Result<()> {
        execute_success_test(
            "cbq",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R4RK1 b kq - 2 13",
            "O-O-O",
        )
    }

    #[test]
    fn case_12() {
        execute_success_test(
            "pg7f8wqbr",
            "rnbq1rk1/p4pPp/2pbp3/8/3P4/8/Pp2BPPP/R1BQK1NR w KQ - 0 12",
            "gxf8=Q+",
        )
            .unwrap()
    }

    #[test]
    fn case_13() {
        execute_success_test(
            "sbqd8f8wq",
            "rnbq1Qk1/p4p1p/2pbp3/8/3P4/8/Pp2BPPP/R1BQK1NR b KQ - 0 12",
            "Qxf8",
        )
            .unwrap()
    }
}
