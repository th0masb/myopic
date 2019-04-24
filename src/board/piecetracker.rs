use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::board::hash;
use crate::pieces;
use crate::pieces::Piece;

type P = &'static dyn Piece;
const PS: [P; 12] = pieces::ALL;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct PieceTracker {
    boards: Vec<BitBoard>,
    hash: u64,
}

impl PieceTracker {
    /// Moves a piece from a source square to a target square erasing any piece
    /// found at the target square before returning a pair containing the moving
    /// piece and the erased piece (if it exists). Will panic if there is no
    /// piece found at the source square.
    pub fn move_piece(&mut self, source: Square, target: Square) -> (P, Option<P>) {
        let mut moved = None;
        let mut removed = None;
        for (i, &p) in PS.iter().enumerate() {
            if moved.is_none() && self.boards[i].contains(source) {
                debug_assert!(!self.boards[i].contains(target));
                self.boards[i] ^= source.lift() ^ target;
                self.hash ^= hash::piece_feature(p, source) ^ hash::piece_feature(p, target);
                moved = Some(p);
            } else if removed.is_none() && self.boards[i].contains(target) {
                self.boards[i] ^= target;
                self.hash ^= hash::piece_feature(p, target);
                removed = Some(p);
            }
        }
        (moved.unwrap(), removed)
    }

    pub fn toggle_piece(&mut self, piece: P, locations: &[Square]) {
        for &location in locations.iter() {
            self.boards[piece.index()] ^= location
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
    fn test_move_case_1() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.move_piece(E5, C3);
        assert_eq!(board, init_tracker(Some(C3), None));
        assert_eq!((pieces::WP, Some(pieces::BN)), result);
    }

    #[test]
    fn test_move_case_2() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.move_piece(C3, E5);
        assert_eq!(board, init_tracker(None, Some(E5)));
        assert_eq!((pieces::BN, Some(pieces::WP)), result);
    }

    #[test]
    fn test_move_case_3() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.move_piece(E5, E4);
        assert_eq!(board, init_tracker(Some(E4), Some(C3)));
        assert_eq!((pieces::WP, None), result);
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
