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
        for i in 0..12 {
            if moved.is_none() && self.boards[i].contains(source) {
                self.boards[i] ^= source.lift() ^ target;
                let p = PS[i];
                moved = Some(p);
                self.hash ^= hash::piece_feature(p, source) ^ hash::piece_feature(p, target);
            } else if removed.is_none() && self.boards[i].contains(target) {
                self.boards[i] ^= target;
                let p = PS[i];
                removed = Some(p);
                self.hash ^= hash::piece_feature(p, target);
            }
        }
        (moved.unwrap(), removed)
    }

    pub fn add_piece(&mut self, piece: P, location: Square) {
        self.boards[piece.index()] ^= location
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

    #[test]
    fn test_move_case_1() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.move_piece(E5, C3);
        assert_eq!(board, init_tracker(Some(C3), None));
        assert_eq!((pieces::WP, Some(pieces::BN)), result);
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
