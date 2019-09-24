use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::cache::rays::RaySet;
use crate::board::implementation::BoardImpl;
use crate::board::Board;
use crate::pieces::Piece;

use super::{BLACK_SLIDERS, WHITE_SLIDERS};

impl BoardImpl {
    pub fn pinned_set_impl(&mut self) -> RaySet {
        match &self.cache.pinned_set {
            Some(x) => x.clone(),
            None => {
                let result = self.compute_pinned();
                self.cache.pinned_set = Some(result.clone());
                result
            }
        }
    }

    /// Computes the set of all active pieces which are pinned to the king,
    /// i.e have their movement areas constrained so that they do not move
    /// and leave the king in check.
    fn compute_pinned(&self) -> RaySet {
        let locs = |side: Side| self.side(side);
        let (active, passive) = (locs(self.active), locs(self.active.reflect()));
        let king_loc = self.pieces.king_location(self.active);
        let (mut constraint_areas, mut i) = (super::empty_ray_pairs(), 0);
        let mut pinned_locations = BitBoard::EMPTY;
        for potential_pinner in self.compute_potential_pinners(king_loc) {
            let cord = BitBoard::cord(king_loc, potential_pinner);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 && i < super::MAX_SIZE {
                let pinned_loc = ((cord & active) - king_loc).into_iter().next().unwrap();
                constraint_areas[i] = Some((pinned_loc, cord));
                i += 1;
                pinned_locations |= pinned_loc;
            }
        }
        RaySet { ray_points: pinned_locations, rays: constraint_areas }
    }

    fn compute_potential_pinners(&self, king_loc: Square) -> BitBoard {
        let passive_sliders = match self.active {
            Side::White => BLACK_SLIDERS,
            Side::Black => WHITE_SLIDERS,
        };
        let locs = |p: Piece| self.pieces.locs_impl(p);
        passive_sliders.iter().flat_map(|&p| locs(p) & p.empty_control(king_loc)).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::base::bitboard::constants::*;

    use super::*;
    use super::super::empty_ray_pairs;

    fn execute_test(fen: &'static str, expected_pinned: RaySet) {
        let board = crate::board::from_fen(fen).unwrap();
        assert_eq!(expected_pinned.reflect(), board.reflect().compute_pinned());
        assert_eq!(expected_pinned, board.compute_pinned());
    }

    #[test]
    fn case_one() {
        let fen = "K2Q4/7p/1B4n1/2bq4/2rkp1R1/4p3/5br1/6B1 b KQkq - 5 10";
        let mut rays = empty_ray_pairs();
        rays[0] = Some((Square::E4, D4 | E4 | F4 | G4));
        rays[1] = Some((Square::C5, B6 | C5 | D4));
        rays[2] = Some((Square::D5, D4 | D5 | D6 | D7 | D8));
        let expected_pinned = RaySet {
            ray_points: C5 | D5 | E4,
            rays,
        };
        execute_test(fen, expected_pinned);
    }
}
