use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::board::Board;
use crate::pieces::Piece;
use crate::base::Side;

/// Static exchange evaluator
struct See<'a, B: Board> {
    board: &'a B,
    source: Square,
    target: Square,
}
impl<B: Board> See<'_, B> {
    fn is_good_exchange(&self) -> bool {
        unimplemented!()
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.board.piece_locations(piece)
    }

    /// Get (direct attadef, xray attadef) involved.
    fn pieces_involved(&self) -> (BitBoard, BitBoard) {
        let (board, target) = (self.board, self.target);
        let (whites, blacks) = board.side_locations();
        let zero = BitBoard::EMPTY;
        let (mut attadef, mut xray) = (zero, zero);
        for (p, loc) in
            Piece::iter().flat_map(|p| self.locs(p).into_iter().map(move |loc| (p, loc)))
        {
            if p.control(loc, whites, blacks).contains(target) {
                attadef ^= loc;
            } else if is_slider(p) && p.control(loc, zero, zero).contains(target) {
                xray ^= loc;
            }
        }
        (attadef, xray)
    }

    fn update_xrays(&self, attadef: BitBoard, xray: BitBoard) -> (BitBoard, BitBoard) {
        if xray.is_empty() {
            (attadef, xray)
        } else {
            let (whites, blacks) = self.board.side_locations();
            let (mut new_attadef, mut new_xray) = (attadef, xray);
            sliders()
                .iter()
                .cloned()
                .map(|p| (p, self.locs(p)))
                .filter(|(_, locs)| locs.intersects(xray))
                .flat_map(|(p, locs)| locs.iter().map(move |loc| (p, loc)))
                .filter(|(_, loc)| xray.contains(*loc))
                .filter(|(p, loc)| p.control(*loc, whites, blacks).contains(self.target))
                .for_each(|(_, loc)| {
                    new_xray ^= loc;
                    new_attadef ^= loc;
                });
            (new_attadef, new_xray)
        }
    }

    fn least_valuable_piece(&self, options: BitBoard, side: Side) -> BitBoard {
        unimplemented!()//Piece::on
    }
}

fn sliders<'a>() -> &'a [Piece] {
    &[
        Piece::WB,
        Piece::WR,
        Piece::WQ,
        Piece::BB,
        Piece::BR,
        Piece::BQ,
    ]
}

fn is_slider(piece: Piece) -> bool {
    match piece {
        Piece::WP | Piece::BP | Piece::WN | Piece::BN | Piece::WK | Piece::BK => false,
        _ => true,
    }
}
