use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::implementation::hash;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct PieceTracker {
    boards: [BitBoard; 12],
    hash: u64,
}

fn hash_boards(boards: &[BitBoard]) -> u64 {
    assert_eq!(12, boards.len());
    boards
        .iter()
        .zip(Piece::iter())
        .flat_map(|(&b, p)| b.into_iter().map(move |sq| hash::piece_feature(p, sq)))
        .fold(0u64, |a, b| a ^ b)
}

fn convert_rank(fen_rank: String) -> Vec<Option<Piece>> {
    let mut dest: Vec<Option<Piece>> = Vec::new();
    for character in fen_rank.chars() {
        if character.is_numeric() {
            dest.extend(itertools::repeat_n(None, character as usize));
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

impl PieceTracker {
    pub fn from_fen(ranks: Vec<String>) -> PieceTracker {
        assert_eq!(8, ranks.len());
        let mut board = ranks
            .into_iter()
            .flat_map(|r| convert_rank(r).into_iter())
            .collect::<Vec<_>>();
        board.reverse();
        let mut bitboards = [BitBoard::EMPTY; 12];
        for (i, x) in board.into_iter().enumerate() {
            match x {
                Some(p) => bitboards[p as usize] |= Square::from_index(i),
                _ => (),
            }
        }
        PieceTracker {
            boards: bitboards,
            hash: hash_boards(&bitboards),
        }
    }

    pub fn new(initial_boards: &[BitBoard]) -> PieceTracker {
        assert_eq!(12, initial_boards.len());
        let initial_hash = hash_boards(initial_boards);
        let mut dest: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        dest.copy_from_slice(initial_boards);
        PieceTracker {
            boards: dest,
            hash: initial_hash,
        }
    }

    pub fn side_locations(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.whites(),
            Side::Black => self.blacks(),
        }
    }

    pub fn king_location(&self, side: Side) -> Square {
        self.locations(Piece::king(side))
            .into_iter()
            .next()
            .unwrap()
    }

    pub fn whites(&self) -> BitBoard {
        self.boards
            .iter()
            .take(6)
            .fold(BitBoard::EMPTY, |a, &b| a | b)
    }

    pub fn blacks(&self) -> BitBoard {
        self.boards
            .iter()
            .skip(6)
            .fold(BitBoard::EMPTY, |a, &b| a | b)
    }

    pub fn locations(&self, piece: Piece) -> BitBoard {
        self.boards[piece as usize]
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.boards
            .iter()
            .zip(Piece::iter())
            .find(|(&b, _)| b.contains(square))
            .map(|(_, p)| p)
    }

    pub fn erase_square(&mut self, square: Square) -> Option<Piece> {
        let mut erased_piece = None;
        for (i, p) in Piece::iter().enumerate() {
            if self.boards[i].contains(square) {
                erased_piece = Some(p);
                self.boards[i] ^= square;
                self.hash ^= hash::piece_feature(p, square);
                break;
            }
        }
        erased_piece
    }

    pub fn toggle_piece(&mut self, piece: Piece, locations: &[Square]) {
        for &location in locations.iter() {
            self.boards[piece as usize] ^= location;
            self.hash ^= hash::piece_feature(piece, location);
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod test {
    use std::iter;

    use crate::base::square::Square::C3;
    use crate::base::square::Square::E4;
    use crate::base::square::Square::E5;
    use crate::pieces::Piece;

    use super::*;

    #[test]
    fn test_erase_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.erase_square(E5);
        assert_eq!(Some(Piece::WP), result);
        assert_eq!(init_tracker(None, Some(C3)), board);
        let result2 = board.erase_square(E4);
        assert_eq!(None, result2);
        assert_eq!(init_tracker(None, Some(C3)), board);
    }

    #[test]
    fn test_toggle_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        board.toggle_piece(Piece::WP, &[E5, E4]);
        assert_eq!(init_tracker(Some(E4), Some(C3)), board);
    }

    fn init_tracker(pawn_loc: Option<Square>, knight_loc: Option<Square>) -> PieceTracker {
        let mut boards: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        boards[Piece::WP as usize] = pawn_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        boards[Piece::BN as usize] = knight_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        let p_hash = pawn_loc.map_or(0u64, |x| hash::piece_feature(Piece::WP, x));
        let n_hash = knight_loc.map_or(0u64, |x| hash::piece_feature(Piece::BN, x));
        PieceTracker {
            boards,
            hash: p_hash ^ n_hash,
        }
    }
}
