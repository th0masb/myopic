use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::board::hash;
use crate::pieces;
use crate::pieces::Piece;
use crate::board::PieceRef;
use crate::board::Board;
use crate::base::Side;

const PS: [PieceRef; 12] = pieces::ALL;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct PieceTracker {
    boards: Vec<BitBoard>,
    hash: u64,
}

impl PieceTracker {
    pub fn new(initial_boards: Vec<BitBoard>) -> PieceTracker {
        assert_eq!(12, initial_boards.len());
        let initial_hash = initial_boards.iter().zip(&pieces::ALL)
            .flat_map(|(&b, &p)| b.into_iter().map(move |sq| hash::piece_feature(p, sq)))
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

    pub fn whites(&self) -> BitBoard {
        self.boards.iter().take(6).fold(BitBoard::EMPTY, |a, &b| a | b)
    }

    pub fn blacks(&self) -> BitBoard {
        self.boards.iter().skip(6).fold(BitBoard::EMPTY, |a, &b| a | b)
    }

    pub fn locations(&self, piece: PieceRef) -> BitBoard {
        self.boards[piece.index()]
    }

    pub fn piece_at(&self, square: Square) -> Option<PieceRef> {
        self.boards.iter().zip(&PS).find(|(&b, _)| b.contains(square)).map(|(_, &p)| p)
    }

    pub fn erase_square(&mut self, square: Square) -> Option<PieceRef> {
        let mut erased_piece = None;
        for (i, &p) in PS.iter().enumerate() {
            if self.boards[i].contains(square) {
                erased_piece = Some(p);
                self.boards[i] ^= square;
                self.hash ^= hash::piece_feature(p, square);
                break;
            }
        }
        erased_piece
    }

    pub fn toggle_piece(&mut self, piece: PieceRef, locations: &[Square]) {
        for &location in locations.iter() {
            self.boards[piece.index()] ^= location;
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

    use crate::base::square::constants::C5;
    use crate::base::square::constants::E3;
    use crate::pieces;

    use super::*;
    use crate::base::square::constants::E5;
    use crate::base::square::constants::C3;
    use crate::base::square::constants::E4;

    #[test]
    fn test_erase_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.erase_square(E5);
        assert_eq!(Some(pieces::WP), result);
        assert_eq!(init_tracker(None, Some(C3)), board);
        let result2 = board.erase_square(E4);
        assert_eq!(None, result2);
        assert_eq!(init_tracker(None, Some(C3)), board);
    }

    #[test]
    fn test_toggle_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        board.toggle_piece(pieces::WP, &[E5, E4]);
        assert_eq!(init_tracker(Some(E4), Some(C3)), board);
    }

    fn init_tracker(pawn_loc: Option<Square>, knight_loc: Option<Square>) -> PieceTracker {
        let mut boards: Vec<_> = iter::repeat(BitBoard::EMPTY).take(12).collect();
        boards[pieces::WP.index()] = pawn_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        boards[pieces::BN.index()] = knight_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        let p_hash = pawn_loc.map_or(0u64, |x| hash::piece_feature(pieces::WP, x));
        let n_hash = knight_loc.map_or(0u64, |x| hash::piece_feature(pieces::BN, x));
        PieceTracker {
            boards,
            hash: p_hash ^ n_hash,
        }
    }
}
