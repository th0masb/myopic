use std::collections::BTreeSet;

use enumset::EnumSet;

use crate::{BitBoard, CastleZone, Dir, Piece, Side, Square};

/// Chess is a symmetric game and this trait represents a component of
/// the game which can be reflected to it's symmetric opposite component.
pub trait Reflectable {
    fn reflect(&self) -> Self;
}

impl Reflectable for Side {
    fn reflect(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

impl Reflectable for Dir {
    fn reflect(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::NE => Dir::SW,
            Dir::SE => Dir::NW,
            Dir::SW => Dir::NE,
            Dir::NW => Dir::SE,
            Dir::NNE => Dir::SSW,
            Dir::NEE => Dir::SWW,
            Dir::SEE => Dir::NWW,
            Dir::SSE => Dir::NNW,
            Dir::SSW => Dir::NNE,
            Dir::SWW => Dir::NEE,
            Dir::NWW => Dir::SEE,
            Dir::NNW => Dir::SSE,
        }
    }
}

impl Reflectable for BitBoard {
    fn reflect(&self) -> Self {
        self.into_iter().map(|sq| sq.reflect()).collect()
    }
}

impl Reflectable for Square {
    fn reflect(&self) -> Self {
        let (fi, ri) = (self.file_index(), self.rank_index());
        Square::from_index((8 * (7 - ri) + fi) as usize)
    }
}

/// We reflect a piece to it's correspondent on the opposite side.
impl Reflectable for Piece {
    fn reflect(&self) -> Self {
        match self {
            Piece::WP => Piece::BP,
            Piece::WN => Piece::BN,
            Piece::WB => Piece::BB,
            Piece::WR => Piece::BR,
            Piece::WQ => Piece::BQ,
            Piece::WK => Piece::BK,
            Piece::BP => Piece::WP,
            Piece::BN => Piece::WN,
            Piece::BB => Piece::WB,
            Piece::BR => Piece::WR,
            Piece::BQ => Piece::WQ,
            Piece::BK => Piece::WK,
        }
    }
}

/// A castle is reflected by it's side, i.e.
///  - WK <==> BK
///  - WQ <==> BQ
impl Reflectable for CastleZone {
    fn reflect(&self) -> Self {
        match self {
            CastleZone::WK => CastleZone::BK,
            CastleZone::WQ => CastleZone::BQ,
            CastleZone::BK => CastleZone::WK,
            CastleZone::BQ => CastleZone::WQ,
        }
    }
}

impl Reflectable for i32 {
    fn reflect(&self) -> Self {
        -(*self)
    }
}

impl<T: Reflectable> Reflectable for Vec<T> {
    fn reflect(&self) -> Self {
        self.into_iter().map(|t| t.reflect()).collect()
    }
}

impl<T: Reflectable> Reflectable for Option<T> {
    fn reflect(&self) -> Self {
        self.as_ref().map(|t| t.reflect())
    }
}

impl<T1, T2> Reflectable for (T1, T2)
    where
        T1: Reflectable,
        T2: Reflectable,
{
    fn reflect(&self) -> Self {
        (self.0.reflect(), self.1.reflect())
    }
}

impl<T1, T2, T3> Reflectable for (T1, T2, T3)
    where
        T1: Reflectable,
        T2: Reflectable,
        T3: Reflectable,
{
    fn reflect(&self) -> Self {
        (self.0.reflect(), self.1.reflect(), self.2.reflect())
    }
}

impl<T: Reflectable + Ord> Reflectable for BTreeSet<T> {
    fn reflect(&self) -> Self {
        self.iter().map(|x| x.reflect()).collect()
    }
}

impl<T: Reflectable + enumset::EnumSetType> Reflectable for EnumSet<T> {
    fn reflect(&self) -> Self {
        self.iter().map(|z| z.reflect()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direction_reflection() {
        assert_eq!(Dir::N, Dir::S.reflect());
        assert_eq!(Dir::E, Dir::W.reflect());
        assert_eq!(Dir::S, Dir::N.reflect());
        assert_eq!(Dir::W, Dir::E.reflect());
        assert_eq!(Dir::NE, Dir::SW.reflect());
        assert_eq!(Dir::SE, Dir::NW.reflect());
        assert_eq!(Dir::SW, Dir::NE.reflect());
        assert_eq!(Dir::NW, Dir::SE.reflect());
        assert_eq!(Dir::NNE, Dir::SSW.reflect());
        assert_eq!(Dir::NEE, Dir::SWW.reflect());
        assert_eq!(Dir::SEE, Dir::NWW.reflect());
        assert_eq!(Dir::SSE, Dir::NNW.reflect());
        assert_eq!(Dir::SSW, Dir::NNE.reflect());
        assert_eq!(Dir::SWW, Dir::NEE.reflect());
        assert_eq!(Dir::NWW, Dir::SEE.reflect());
        assert_eq!(Dir::NNW, Dir::SSE.reflect());
    }
}
