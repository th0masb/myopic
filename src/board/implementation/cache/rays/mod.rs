use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::BoardImpl;
use crate::pieces::Piece;

pub const WHITE_SLIDERS: [Piece; 3] = [Piece::WB, Piece::WR, Piece::WQ];
pub const BLACK_SLIDERS: [Piece; 3] = [Piece::BB, Piece::BR, Piece::BQ];

pub mod pinning;
pub mod discovery;

/// A pinned set consisted of the locations of all the pieces which are pinned
/// alongside a vector containing the constraint area for each of these pinned
/// pieces.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RaySet {
    pub ray_points: BitBoard,
    // TODO Replace vec with fixed length array
    pub rays: Vec<(Square, BitBoard)>,
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
