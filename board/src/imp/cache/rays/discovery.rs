use myopic_core::*;

use crate::{Board, ChessBoard};
use crate::imp::cache::rays::RaySet;

impl Board {
    pub fn compute_discoveries(&self) -> RaySet {
        let locs = |side: Side| self.side(side);
        let (active, passive) = (locs(self.active), locs(self.active.reflect()));
        let king_loc = self.pieces.king_location(self.active.reflect());
        let mut discovery_rays: Vec<(Square, BitBoard)> = Vec::with_capacity(2);
        let mut discovers = BitBoard::EMPTY;
        for xrayer in self.compute_xrayers(king_loc) {
            let cord = BitBoard::cord(king_loc, xrayer);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 {
                let discov_loc = ((cord & active) - xrayer).first().unwrap();
                discovery_rays.push((discov_loc, cord));
                discovers |= discov_loc;
            }
        }
        RaySet {
            ray_points: discovers,
            rays: discovery_rays,
        }
    }

    fn compute_xrayers(&self, king_loc: Square) -> BitBoard {
        let active_sliders = match self.active {
            Side::White => super::WHITE_SLIDERS,
            Side::Black => super::BLACK_SLIDERS,
        };
        let locs = |p: Piece| self.locs(&[p]);
        active_sliders
            .iter()
            .flat_map(|&p| locs(p) & p.empty_control(king_loc))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use myopic_core::constants::*;

    use super::*;

    fn execute_test(fen: &'static str, expected_discoveries: RaySet) {
        let board = fen.parse::<Board>().unwrap();
        assert_eq!(
            expected_discoveries.reflect(),
            board.reflect().compute_discoveries()
        );
        assert_eq!(expected_discoveries, board.compute_discoveries());
    }

    #[test]
    fn case_one() {
        let fen = "6r1/5p1k/4pP2/4N3/3PN3/6P1/2B3PK/7R w - - 1 10";
        let expected_pinned = RaySet {
            ray_points: E4 | H2,
            rays: vec![
                (Square::E4, C2 | D3 | E4 | F5 | G6 | H7),
                (Square::H2, H1 | H2 | H3 | H4 | H5 | H6 | H7),
            ],
        };
        execute_test(fen, expected_pinned);
    }
}
