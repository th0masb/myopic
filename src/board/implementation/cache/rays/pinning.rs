use super::{BLACK_SLIDERS, WHITE_SLIDERS};
use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::cache::rays::RaySet;
use crate::board::implementation::BoardImpl;
use crate::board::Board;
use crate::pieces::Piece;

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
        let mut constraint_areas: Vec<(Square, BitBoard)> = Vec::with_capacity(2);
        let mut pinned_locations = BitBoard::EMPTY;
        for potential_pinner in self.compute_potential_pinners(king_loc) {
            let cord = BitBoard::cord(king_loc, potential_pinner);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 {
                let pinned_loc = ((cord & active) - king_loc).into_iter().next().unwrap();
                constraint_areas.push((pinned_loc, cord));
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
    use crate::base::castlezone::CastleZoneSet;

    use super::*;
    use crate::board::test_board::TestBoard;

    struct TestCase {
        input: TestBoard,
        expected: RaySet,
    }

    fn execute_test(case: TestCase) {
        assert_eq!(case.expected, BoardImpl::from(case.input).compute_pinned());
    }

    const EMPTY: BitBoard = BitBoard::EMPTY;

    #[test]
    fn case_one() {
        execute_test(TestCase {
            input: TestBoard {
                active: Side::Black,
                whites: vec![EMPTY, EMPTY, G1 | B6, G4, D8, EMPTY],
                blacks: vec![E3 | E4 | H8, G6, C5 | F2, C4 | G2, D5, D4],
                castle_rights: CastleZoneSet::ALL,
                enpassant: None,
                white_status: None,
                black_status: None,
                clock: 10,
                history_count: 10,
            },

            expected: RaySet {
                ray_points: C5 | D5 | E4,
                rays: vec![
                    (Square::E4, D4 | E4 | F4 | G4),
                    (Square::C5, B6 | C5 | D4),
                    (Square::D5, D4 | D5 | D6 | D7 | D8),
                ],
            },
        })
    }
}
