use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::board::Board;
use crate::pieces::Piece;

/// Static exchange evaluator
struct See<'a, B: Board> {
    board: &'a B,
}
impl<B: Board> See<'_, B> {
    fn is_good_exchange(&self, src: Square, target: Square) -> bool {
        unimplemented!()
    }

    /// Get (direct attadef, xray attadef) involved.
    fn pieces_involved(&self, src: Square, target: Square) -> (BitBoard, BitBoard) {
        let board = self.board;
        let (whites, blacks) = board.side_locations();
        let zero = BitBoard::EMPTY;
        let (mut attadef, mut xray) = (zero, zero);
        let locs = |piece: Piece| board.piece_locations(piece);
        for (p, loc) in Piece::iter().flat_map(|p| locs(p).into_iter().map(move |loc| (p, loc))) {
            if p.control(loc, whites, blacks).contains(target) {
                attadef ^= loc;
            } else if is_slider(p) && p.control(loc, zero, zero).contains(target) {
                xray ^= loc;
            }
        }
        (attadef, xray)
    }
}

fn is_slider(piece: Piece) -> bool {
    match piece {
        Piece::WP | Piece::BP | Piece::WN | Piece::BN | Piece::WK | Piece::BK => false,
        _ => true,
    }
}
