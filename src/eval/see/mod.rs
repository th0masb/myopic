use std::cmp;

use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::Board;
use crate::eval::values;
use crate::pieces::Piece;

#[cfg(test)]
mod test;

/// API function for determining whether an exchange is good on the given
/// board. The board must have a piece at both the source and target square
/// otherwise this function will panic. The pieces must be on opposing
/// sides and the quality of the return value is in relation to the side of
/// the attacker, higher is good for the attacker. Positive means a good exchange,
/// negative mean a bad one. If the pieces are on the same side the result is
/// undefined.
///
pub fn exchange_value<B: Board>(board: &B, source: Square, target: Square) -> i32 {
    See {
        board,
        source,
        target,
        value: values::abs_midgame,
    }
    .exchange_value()
}

/// Static exchange evaluator
struct See<'a, B: Board> {
    board: &'a B,
    source: Square,
    target: Square,
    value: fn(Piece) -> i32,
}

impl<B: Board> See<'_, B> {
    fn exchange_value(&self) -> i32 {
        let board = self.board;
        let knights = self.locs(Piece::WN) | self.locs(Piece::BN);
        let first_attacker = board.piece_at(self.source).unwrap();
        let first_victim = board.piece_at(self.target).unwrap();
        let mut d = 0;
        let mut gain: [i32; 32] = [0; 32];
        gain[d] = (self.value)(first_victim);

        let mut attacker = first_attacker;
        let mut active = first_attacker.side();
        let mut src = self.source.lift();
        let (mut attadef, mut xray) = self.pieces_involved();
        loop {
            d += 1;
            active = active.reflect();
            gain[d] = (self.value)(attacker) - gain[d - 1];
            // TODO Can add this optimization in if we only want to know is exchange is good
            //if cmp::max(-gain[d - 1], gain[d]) < 0 {
            //    break;
            //}
            attadef ^= src;
            if !src.intersects(knights) {
                let (new_attadef, new_xray) = self.update_xrays(attadef, xray);
                attadef = new_attadef;
                xray = new_xray;
            }
            src = self.least_valuable_piece(attadef, active);
            if src.is_empty() {
                break;
            } else {
                attacker = board.piece_at(src.first().unwrap()).unwrap();
            }
        }
        d -= 1;
        while d > 0 {
            gain[d - 1] = -cmp::max(-gain[d - 1], gain[d]);
            d -= 1;
        }
        gain[0]
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.board.piece_locations(piece)
    }

    /// Get (direct attadef, xray attadef) involved.
    fn pieces_involved(&self) -> (BitBoard, BitBoard) {
        let (board, target) = (self.board, self.target);
        let (whites, blacks) = board.whites_blacks();
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
            let (whites, blacks) = self.board.whites_blacks();
            let (mut new_attadef, mut new_xray) = (attadef, xray);
            sliders()
                .iter()
                .map(|&p| (p, self.locs(p) & xray))
                .flat_map(|(p, locs)| locs.iter().map(move |loc| (p, loc)))
                .filter(|(p, loc)| p.control(*loc, whites, blacks).contains(self.target))
                .for_each(|(_, loc)| {
                    new_xray ^= loc;
                    new_attadef ^= loc;
                });
            (new_attadef, new_xray)
        }
    }

    fn least_valuable_piece(&self, options: BitBoard, side: Side) -> BitBoard {
        Piece::on_side(side)
            .map(|p| self.locs(p))
            .find(|locs| locs.intersects(options))
            .map_or(BitBoard::EMPTY, |locs| locs.least_set_bit())
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
