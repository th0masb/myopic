use crate::base::bitboard::BitBoard;
use crate::pieces::Piece;
use crate::pieces;
use crate::base::square::Square;
use crate::board::hash;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct PieceTracker {
    boards: Vec<BitBoard>,
    hash: u64,
}

impl PieceTracker {
    pub fn move_piece(&mut self, source: Square, target: Square) -> (Option<&dyn Piece>, u64) {
        unimplemented!()
    }

    pub fn piece_at(&self, location: Square) -> Option<&dyn Piece> {
        self.boards.iter().enumerate()
            .find(|(_, &board)| board.contains(location))
            .map(|(i, _)| pieces::ALL[i])
    }

    pub fn contains(&self, piece: &dyn Piece, location: Square) -> bool {
        self.locations(piece).contains(location)
    }

    pub fn locations(&self, piece: &dyn Piece) -> BitBoard {
        self.boards[piece.index()]
    }

    pub fn whites(&self) -> BitBoard {
        (&self.boards).into_iter().take(6).map(|x| *x).collect()
    }

    pub fn blacks(&self) -> BitBoard {
        (&self.boards).into_iter().skip(6).map(|x| *x).collect()
    }

    pub fn add(&mut self, piece: &dyn Piece, location: Square) {
        debug_assert!(!self.boards[piece.index()].contains(location));
        self.perform_xor(piece, location);
    }

    pub fn remove(&mut self, location: Square) -> Option<&dyn Piece> {
        let mut removed = None;
        for (i, &board) in self.boards.iter().enumerate() {
            if board.contains(location) {
                self.boards[i] ^= location;
                removed = Some(pieces::ALL[i]);
                break;
            }
        }
        removed
    }

    fn perform_xor(&mut self, piece: &dyn Piece, location: Square) {
        self.boards[piece.index()] ^= location;
        self.hash ^= hash::piece_feature(piece, location);
    }
}

#[cfg(test)]
mod test {
    use std::iter;

    use crate::base::square::constants::C5;
    use crate::base::square::constants::E3;
    use crate::pieces;

    use super::*;

    /// We test with a simple setup of one white pawn at E3 and one
    /// black knight at C5.
    #[test]
    fn test() {
        let mut tracker = init_pawn_and_knight();
        tracker.remove(pieces::WP, E3);
        assert_eq!(init_knight(), tracker);
        tracker.add(pieces::WP, E3);
        assert_eq!(init_pawn_and_knight(), tracker);
        tracker.remove(pieces::BN, C5);
        assert_eq!(init_pawn(), tracker);
        tracker.remove(pieces::WP, E3);
        assert_eq!(init_empty(), tracker);
        tracker.add(pieces::WP, E3);
        tracker.add(pieces::BN, C5);
        assert_eq!(init_pawn_and_knight(), tracker);
    }

    fn init_pawn_and_knight() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[0] = E3.lift();
        boards[7] = C5.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(pieces::WP, E3) ^ hash::piece_feature(pieces::BN, C5)
        }
    }

    fn init_pawn() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[0] = E3.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(pieces::WP, E3),
        }
    }

    fn init_knight() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[7] = C5.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(pieces::BN, C5),
        }
    }

    fn init_empty() -> PieceTracker {
        PieceTracker {
            boards: init_empty_boards(),
            hash: 0u64,
        }
    }

    fn init_empty_boards() -> Vec<BitBoard> {
        iter::repeat(BitBoard::EMPTY).take(12).collect()
    }
}
