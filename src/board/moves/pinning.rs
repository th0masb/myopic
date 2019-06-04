use crate::base::bitboard::constants::*;
use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::Board;
use crate::pieces::Piece;

use super::{BLACK_SLIDERS, WHITE_SLIDERS};

/// A pinned set consisted of the locations of all the pieces which are pinned
/// alongside a vector containing the constraint area for each of these pinned
/// pieces.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct PinnedSet {
    pub pinned_locations: BitBoard,
    pub constraint_areas: Vec<(Square, BitBoard)>,
}

impl PinnedSet {
    /// Computes the constraint area for a piece at the given square taking into
    /// account some existing constraint set. If the piece is pinned we perform
    /// a linear search to find the constraint area before returning the intersection
    /// of the two constraints. Otherwise we just return the existing constraint.
    pub(super) fn compute_constraint_area(&self, loc: Square, existing: BitBoard) -> BitBoard {
        existing
            & if self.pinned_locations.contains(loc) {
                (&self.constraint_areas)
                    .into_iter()
                    .find(|(sq, _)| *sq == loc)
                    .unwrap()
                    .1
            } else {
                BitBoard::ALL
            }
    }
}

impl Board {
    /// Computes the set of all active pieces which are pinned to the king,
    /// i.e have their movement areas constrained so that they do not move
    /// and leave the king in check.
    ///
    pub(super) fn compute_pinned(&self) -> PinnedSet {
        let locs = |side: Side| self.pieces.side_locations(side);
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
        PinnedSet {
            pinned_locations,
            constraint_areas,
        }
    }

    fn compute_potential_pinners(&self, king_loc: Square) -> BitBoard {
        let passive_sliders = match self.active {
            Side::White => BLACK_SLIDERS,
            Side::Black => WHITE_SLIDERS,
        };
        let locs = |p: Piece| self.pieces.locations(p);
        passive_sliders
            .iter()
            .flat_map(|&p| locs(p) & p.control(king_loc, BitBoard::EMPTY, BitBoard::EMPTY))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::board::testutils::TestBoard;

    use super::*;

    struct TestCase {
        input: TestBoard,
        expected: PinnedSet,
    }

    fn execute_test(case: TestCase) {
        assert_eq!(case.expected, case.input.to_board().compute_pinned());
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
                hash_offset: 10,
            },

            expected: PinnedSet {
                pinned_locations: C5 | D5 | E4,
                constraint_areas: vec![
                    (Square::E4, D4 | E4 | F4 | G4),
                    (Square::C5, B6 | C5 | D4),
                    (Square::D5, D4 | D5 | D6 | D7 | D8),
                ],
            },
        })
    }
}
