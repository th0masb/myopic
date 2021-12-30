use std::iter::FromIterator;

use myopic_core::*;
use myopic_core::enum_map::EnumMap;

pub const WHITE_SLIDERS: [Piece; 3] = [Piece::WB, Piece::WR, Piece::WQ];
pub const BLACK_SLIDERS: [Piece; 3] = [Piece::BB, Piece::BR, Piece::BQ];

pub mod discovery;
pub mod pinning;

/// A pinned set consisted of the locations of all the pieces which are pinned
/// alongside a vector containing the constraint area for each of these pinned
/// pieces.
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct RaySet {
    points: BitBoard,
    contents: EnumMap<Square, BitBoard>,
}

impl FromIterator<(Square, BitBoard)> for RaySet {
    fn from_iter<T: IntoIterator<Item=(Square, BitBoard)>>(iter: T) -> Self {
        let mut points = BitBoard::EMPTY;
        let mut contents = EnumMap::default();
        for (square, board) in iter {
            contents[square] = board;
            points |= square;
        }
        RaySet { points, contents }
    }
}

impl RaySet {
    pub fn points(&self) -> BitBoard {
        self.points
    }

    pub fn ray(&self, loc: Square) -> Option<BitBoard> {
        if self.points.contains(loc) {
            Some(self.contents[loc])
        } else {
            None
        }
    }
}

impl Reflectable for RaySet {
    fn reflect(&self) -> Self {
        let mut contents = EnumMap::default();
        for point in self.points {
            contents[point.reflect()] = self.contents[point].reflect();
        }
        RaySet { points: self.points.reflect(), contents }
    }
}
