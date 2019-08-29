use crate::board::{BoardImpl, Board};
use crate::board::implementation::cache::rays::RaySet;
use crate::base::{Side, Reflectable};
use crate::base::square::Square;
use crate::base::bitboard::BitBoard;
use crate::pieces::Piece;

impl BoardImpl {
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
        RaySet {ray_points: discovers, rays: discovery_rays}
    }

    fn compute_xrayers(&self, king_loc: Square) -> BitBoard {
        let active_sliders = match self.active {
            Side::White => super::WHITE_SLIDERS,
            Side::Black => super::BLACK_SLIDERS
        };
        let locs = |p: Piece| self.locs(p);
        active_sliders.iter().flat_map(|&p| locs(p) & p.empty_control(king_loc)).collect()
    }
}