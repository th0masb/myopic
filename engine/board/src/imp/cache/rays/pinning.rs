use myopic_core::*;

use crate::imp::cache::rays::RaySet;
use crate::imp::Board;
use crate::ChessBoard;

impl Board {
    pub fn pinned_set(&self) -> RaySet {
        let mut cache = self.cache.borrow_mut();
        match &cache.pinned_set {
            Some(x) => x.clone(),
            None => {
                let result = self.compute_pinned();
                cache.pinned_set = Some(result.clone());
                result
            }
        }
    }

    /// Computes the set of all active pieces which are pinned to the king,
    /// i.e have their movement areas constrained so that they do not move
    /// and leave the king in check.
    fn compute_pinned(&self) -> RaySet {
        let active = self.side(self.active);
        let passive = self.side(self.active.reflect());
        let king_loc = self.pieces.locs(Piece(self.active, Class::K)).into_iter().next().unwrap();

        self.compute_xrays(king_loc)
            .iter()
            .map(|square| BitBoard::cord(king_loc, square))
            .filter(|&cord| (cord & active).size() == 2 && (cord & passive).size() == 1)
            .map(|cord| {
                let pinned_loc = ((cord & active) - king_loc).first().unwrap();
                (pinned_loc, cord)
            })
            .collect()
    }

    fn compute_xrays(&self, king_loc: Square) -> BitBoard {
        [Class::B, Class::R, Class::Q]
            .iter()
            .map(|&class| Piece(self.active.reflect(), class))
            .map(|piece| self.pieces.locs(piece) & piece.empty_control(king_loc))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use myopic_core::Square::*;

    use super::*;

    fn execute_test(fen: &'static str, expected_pinned: RaySet) {
        let board = fen.parse::<Board>().unwrap();
        assert_eq!(expected_pinned.reflect(), board.reflect().compute_pinned());
        assert_eq!(expected_pinned, board.compute_pinned());
    }

    #[test]
    fn case_one() {
        let fen = "K2Q4/7p/1B4n1/2bq4/2rkp1R1/4p3/5br1/6B1 b KQkq - 5 10";
        let expected_pinned =
            vec![(E4, D4 | E4 | F4 | G4), (C5, B6 | C5 | D4), (D5, D4 | D5 | D6 | D7 | D8)]
                .into_iter()
                .collect();

        execute_test(fen, expected_pinned);
    }
}
