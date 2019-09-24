use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::pieces::Piece;
use std::collections::BTreeSet;

pub const WHITE_SLIDERS: [Piece; 3] = [Piece::WB, Piece::WR, Piece::WQ];
pub const BLACK_SLIDERS: [Piece; 3] = [Piece::BB, Piece::BR, Piece::BQ];

pub mod discovery;
pub mod pinning;

pub const MAX_SIZE: usize = 5;
pub type RayPairs = [Option<(Square, BitBoard)>; MAX_SIZE];

/// A pinned set consisted of the locations of all the pieces which are pinned
/// alongside a vector containing the constraint area for each of these pinned
/// pieces.
#[derive(Debug, Clone, PartialOrd, Hash)]
pub struct RaySet {
    pub ray_points: BitBoard,
    // TODO Replace vec with fixed length array
    //pub rays: Vec<(Square, BitBoard)>,
    pub rays: RayPairs,

}

const fn empty_ray_pairs() -> RayPairs {
    [None; MAX_SIZE]
}

impl Reflectable for [Option<(Square, BitBoard)>; MAX_SIZE] {
    fn reflect(&self) -> Self {
        let mut dest = empty_ray_pairs();
        for i in 0..MAX_SIZE {
            dest[i] = match self[i] {
                Some(x) => Some(x.reflect()),
                _ => None,
            }
        }
        dest
    }
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
            self.rays.iter().filter_map(|x| x.as_ref()).find(|(sq, _)| *sq == loc).map(|(_, c)| *c)
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
