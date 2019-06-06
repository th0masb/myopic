use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::implementation::hash;
use crate::pieces;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct PieceTracker {
    boards: Vec<BitBoard>,
    hash: u64,
}

impl PieceTracker {
    pub fn new(initial_boards: Vec<BitBoard>) -> PieceTracker {
        assert_eq!(12, initial_boards.len());
        let initial_hash = initial_boards
            .iter()
            .zip(Piece::iter())
            .flat_map(|(&b, p)| b.into_iter().map(move |sq| hash::piece_feature(p, sq)))
            .fold(0u64, |a, b| a ^ b);

        PieceTracker {
            boards: initial_boards,
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
        self.locations(pieces::king(side))
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
        let mut boards: Vec<_> = iter::repeat(BitBoard::EMPTY).take(12).collect();
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
