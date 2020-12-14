use myopic_core::*;
use std::collections::BTreeSet;

pub const WHITE_SLIDERS: [Piece; 3] = [Piece::WB, Piece::WR, Piece::WQ];
pub const BLACK_SLIDERS: [Piece; 3] = [Piece::BB, Piece::BR, Piece::BQ];

pub mod discovery;
pub mod pinning;

/// A pinned set consisted of the locations of all the pieces which are pinned
/// alongside a vector containing the constraint area for each of these pinned
/// pieces.
#[derive(Debug, Clone, PartialOrd, Hash)]
pub struct RaySet {
    pub ray_points: BitBoard,
    // TODO Replace vec with fixed length array
    pub rays: Vec<(Square, BitBoard)>,
}

impl PartialEq<RaySet> for RaySet {
    fn eq(&self, other: &RaySet) -> bool {
        self.ray_points == other.ray_points && {
            let this_rayset = self.rays.iter().collect::<BTreeSet<_>>();
            let other_rayset = other.rays.iter().collect::<BTreeSet<_>>();
            this_rayset == other_rayset
        }
    }
}

impl RaySet {
    pub fn ray(&self, loc: Square) -> Option<BitBoard> {
        if self.ray_points.contains(loc) {
            self.rays.iter().find(|(sq, _)| *sq == loc).map(|(_, c)| *c)
        } else {
            None
        }
    }
}

impl Reflectable for RaySet {
    fn reflect(&self) -> Self {
        RaySet { ray_points: self.ray_points.reflect(), rays: self.rays.reflect() }
    }
}
