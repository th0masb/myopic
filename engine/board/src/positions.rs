use std::str::FromStr;

use enum_map::{enum_map, EnumMap};
use myopic_core::anyhow::{anyhow, Error, Result};
use myopic_core::*;

use crate::parse::patterns;

type PiecePositions = EnumMap<Side, EnumMap<Class, BitBoard>>;
type SidePositions = EnumMap<Side, BitBoard>;
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
    iter(boards).for_each(|(side, class, square)| squares[square] = Some(Piece(side, class)));
    squares
}

fn iter(boards: &PiecePositions) -> impl Iterator<Item = (Side, Class, Square)> + '_ {
    boards
        .iter()
        .flat_map(|(side, classes)| classes.iter().map(move |(class, board)| (side, class, *board)))
        .flat_map(|(side, class, board)| board.into_iter().map(move |sq| (side, class, sq)))
}

fn compute_side_positions(boards: &PiecePositions) -> SidePositions {
    enum_map! {
        Side::W => boards[Side::W].iter().fold(BitBoard::EMPTY, |a, (_, b)| a | *b),
        Side::B => boards[Side::B].iter().fold(BitBoard::EMPTY, |a, (_, b)| a | *b),
    }
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
            let mut bitboards = PiecePositions::default();
            for (i, x) in board.into_iter().enumerate() {
                if let Some(Piece(side, class)) = x {
                    bitboards[side][class] |= <usize as Into<Square>>::into(i)
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
        let mut new_boards = PiecePositions::default();
        iter(&self.pieces).for_each(|(side, class, square)| {
            new_boards[side.reflect()][class] |= square.reflect()
        });
        Positions {
            pieces: new_boards,
            hash: hash_boards(&new_boards),
            sides: compute_side_positions(&new_boards),
            squares: compute_square_positions(&new_boards),
        }
    }
}

fn hash_boards(boards: &PiecePositions) -> u64 {
    iter(boards).fold(0u64, |a, (side, class, square)| a ^ hash::piece(Piece(side, class), square))
}

fn convert_rank(fen_rank: String) -> Vec<Option<Piece>> {
    let mut dest: Vec<Option<Piece>> = Vec::new();
    for character in fen_rank.chars() {
        if character.is_numeric() {
            let space = character.to_string().parse::<usize>().unwrap();
            dest.extend(itertools::repeat_n(None, space));
        } else {
            dest.extend(&[Some(match character {
                'P' => Piece(Side::W, Class::P),
                'N' => Piece(Side::W, Class::N),
                'B' => Piece(Side::W, Class::B),
                'R' => Piece(Side::W, Class::R),
                'Q' => Piece(Side::W, Class::Q),
                'K' => Piece(Side::W, Class::K),
                'p' => Piece(Side::B, Class::P),
                'n' => Piece(Side::B, Class::N),
                'b' => Piece(Side::B, Class::B),
                'r' => Piece(Side::B, Class::R),
                'q' => Piece(Side::B, Class::Q),
                'k' => Piece(Side::B, Class::K),
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
        let mut positions = PiecePositions::default();
        PiecePositions::default()
            .iter()
            .flat_map(|(side, classes)| classes.iter().map(move |(class, _)| (side, class)))
            .for_each(|(side, class)| {
                positions[side][class] = initial_boards[(side as usize) * 6 + (class as usize)]
            });
        Positions {
            hash: hash_boards(&positions),
            sides: compute_side_positions(&positions),
            squares: compute_square_positions(&positions),
            pieces: positions,
        }
    }

    #[cfg(debug_assertions)]
    pub fn check_consistent(&self) -> Result<()> {
        for sq in Square::iter() {
            let pieces_piece = iter(&self.pieces)
                .find(|(_, _, loc)| sq == *loc)
                .map(|(s, c, _)| Piece(s, c));
            let squares_piece = self.squares[sq];
            if pieces_piece != squares_piece {
                return Err(anyhow!("Mismatch at {}, pieces: {:?}, squares: {:?}", sq, pieces_piece, squares_piece))
            }
            if let Some(Piece(side, _)) = pieces_piece {
                if !self.sides[side].contains(sq) {
                    return Err(anyhow!("{} does not contain piece at {}", side, sq))
                } else if self.sides[side.reflect()].contains(sq) {
                    return Err(anyhow!("{} contains opponent piece at {}", side.reflect(), sq))
                }
            }
        }
        Ok(())
    }

    pub fn side_locations(&self, side: Side) -> BitBoard {
        self.sides[side]
    }

    pub fn whites(&self) -> BitBoard {
        self.sides[Side::W]
    }

    pub fn blacks(&self) -> BitBoard {
        self.sides[Side::B]
    }

    pub fn locs(&self, Piece(side, class): Piece) -> BitBoard {
        self.pieces[side][class]
    }

    pub fn king_loc(&self, side: Side) -> Square {
        self.locs(Piece(side, Class::K)).into_iter().next().unwrap()
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square].clone()
    }

    pub(crate) fn set_piece(&mut self, piece: Piece, location: Square) {
        self.hash ^= hash::piece(piece, location);
        self.pieces[piece.0][piece.1] |= location;
        self.sides[piece.0] |= location;
        self.squares[location] = Some(piece);
    }

    pub(crate) fn unset_piece(&mut self, piece: Piece, location: Square) {
        self.hash ^= hash::piece(piece, location);
        self.pieces[piece.0][piece.1] -= location;
        self.sides[piece.0] -= location;
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
        board.unset_piece(Piece(Side::W, Class::P), E5);
        board.set_piece(Piece(Side::W, Class::P), E4);
        board.unset_piece(Piece(Side::B, Class::N), C3);
        assert_eq!(init_tracker(Some(E4), None), board);
    }

    fn init_tracker(pawn_loc: Option<Square>, knight_loc: Option<Square>) -> Positions {
        let mut boards = PiecePositions::default();
        boards[Side::W][Class::P] = pawn_loc.map_or(BitBoard::EMPTY, |x| x.into());
        boards[Side::B][Class::N] = knight_loc.map_or(BitBoard::EMPTY, |x| x.into());
        let p_hash = pawn_loc.map_or(0u64, |x| hash::piece(Piece(Side::W, Class::P), x));
        let n_hash = knight_loc.map_or(0u64, |x| hash::piece(Piece(Side::B, Class::N), x));
        Positions {
            pieces: boards,
            hash: p_hash ^ n_hash,
            sides: compute_side_positions(&boards),
            squares: compute_square_positions(&boards),
        }
    }
}
