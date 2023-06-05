use std::str::FromStr;

use enum_map::EnumMap;
use myopic_core::anyhow::{anyhow, Error, Result};
use myopic_core::*;

use crate::parse::patterns;

type PiecePositions = [BitBoard; 12];
type SidePositions = [BitBoard; 2];
type SquarePositions = EnumMap<Square, Option<Piece>>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Positions {
    pieces: PiecePositions,
    sides: SidePositions,
    squares: SquarePositions,
    hash: u64,
}

fn compute_square_positions(boards: &PiecePositions) -> SquarePositions {
    let mut squares = SquarePositions::default();
    for piece in Piece::all() {
        for square in boards[piece as usize] {
            squares[square] = Some(piece)
        }
    }
    squares
}

fn compute_side_positions(boards: &[BitBoard]) -> SidePositions {
    [
        boards.iter().take(6).fold(BitBoard::EMPTY, |a, &b| a | b),
        boards.iter().skip(6).fold(BitBoard::EMPTY, |a, &b| a | b),
    ]
}

impl FromStr for Positions {
    type Err = Error;

    fn from_str(ranks: &str) -> Result<Self, Self::Err> {
        if !patterns::fen_positions().is_match(ranks) {
            Err(anyhow!("{}", ranks))
        } else {
            let mut board = patterns::fen_rank()
                .find_iter(ranks)
                .flat_map(|m| convert_rank(m.as_str().to_owned()))
                .collect::<Vec<_>>();
            board.reverse();
            let mut bitboards = [BitBoard::EMPTY; 12];
            for (i, x) in board.into_iter().enumerate() {
                match x {
                    Some(p) => bitboards[p as usize] |= <usize as Into<Square>>::into(i),
                    _ => (),
                }
            }
            Ok(Positions {
                pieces: bitboards,
                hash: hash_boards(&bitboards),
                sides: compute_side_positions(&bitboards),
                squares: compute_square_positions(&bitboards),
            })
        }
    }
}

impl Reflectable for Positions {
    fn reflect(&self) -> Self {
        let mut new_boards = [BitBoard::EMPTY; 12];
        for i in 0..12 {
            new_boards[i] = self.pieces[(i + 6) % 12].reflect();
        }
        Positions {
            pieces: new_boards,
            hash: hash_boards(&new_boards),
            sides: compute_side_positions(&new_boards),
            squares: compute_square_positions(&new_boards),
        }
    }
}

fn hash_boards(boards: &[BitBoard]) -> u64 {
    assert_eq!(12, boards.len());
    boards
        .iter()
        .zip(Piece::all())
        .flat_map(|(&b, p)| b.into_iter().map(move |sq| hash::piece(p, sq)))
        .fold(0u64, |a, b| a ^ b)
}

fn convert_rank(fen_rank: String) -> Vec<Option<Piece>> {
    let mut dest: Vec<Option<Piece>> = Vec::new();
    for character in fen_rank.chars() {
        if character.is_numeric() {
            let space = character.to_string().parse::<usize>().unwrap();
            dest.extend(itertools::repeat_n(None, space));
        } else {
            dest.extend(&[Some(match character {
                'P' => Piece::WP,
                'N' => Piece::WN,
                'B' => Piece::WB,
                'R' => Piece::WR,
                'Q' => Piece::WQ,
                'K' => Piece::WK,
                'p' => Piece::BP,
                'n' => Piece::BN,
                'b' => Piece::BB,
                'r' => Piece::BR,
                'q' => Piece::BQ,
                'k' => Piece::BK,
                _ => panic!(),
            })]);
        }
    }
    dest
}

impl Positions {
    #[cfg(test)]
    pub fn new(initial_boards: &[BitBoard]) -> Positions {
        assert_eq!(12, initial_boards.len());
        let initial_hash = hash_boards(initial_boards);
        let mut dest: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        dest.copy_from_slice(initial_boards);
        Positions {
            pieces: dest,
            hash: initial_hash,
            sides: compute_side_positions(&dest),
            squares: compute_square_positions(&dest),
        }
    }

    pub fn side_locations(&self, side: Side) -> BitBoard {
        self.sides[side as usize]
    }

    pub fn king_location(&self, side: Side) -> Square {
        self.locs(Piece::king(side)).into_iter().next().unwrap()
    }

    pub fn whites(&self) -> BitBoard {
        self.sides[Side::W as usize]
    }

    pub fn blacks(&self) -> BitBoard {
        self.sides[Side::B as usize]
    }

    pub fn locs(&self, piece: Piece) -> BitBoard {
        self.pieces[piece as usize]
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square]
    }

    pub(crate) fn set_piece(&mut self, piece: Piece, location: Square) {
        self.hash ^= hash::piece(piece, location);
        self.pieces[piece as usize] |= location;
        self.sides[piece.side() as usize] |= location;
        self.squares[location] = Some(piece);
    }

    pub(crate) fn unset_piece(&mut self, piece: Piece, location: Square) {
        self.hash ^= hash::piece(piece, location);
        self.pieces[piece as usize] -= location;
        self.sides[piece.side() as usize] -= location;
        self.squares[location] = None;
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod test {
    use myopic_core::Piece;
    use myopic_core::Square::C3;
    use myopic_core::Square::E4;
    use myopic_core::Square::E5;

    use super::*;

    #[test]
    fn test_toggle_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        board.unset_piece(Piece::WP, E5);
        board.set_piece(Piece::WP, E4);
        board.unset_piece(Piece::BN, C3);
        assert_eq!(init_tracker(Some(E4), None), board);
    }

    fn init_tracker(pawn_loc: Option<Square>, knight_loc: Option<Square>) -> Positions {
        let mut boards: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        boards[Piece::WP as usize] = pawn_loc.map_or(BitBoard::EMPTY, |x| x.into());
        boards[Piece::BN as usize] = knight_loc.map_or(BitBoard::EMPTY, |x| x.into());
        let p_hash = pawn_loc.map_or(0u64, |x| hash::piece(Piece::WP, x));
        let n_hash = knight_loc.map_or(0u64, |x| hash::piece(Piece::BN, x));
        Positions {
            pieces: boards,
            hash: p_hash ^ n_hash,
            sides: compute_side_positions(&boards),
            squares: compute_square_positions(&boards),
        }
    }
}
