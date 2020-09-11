use crate::{Side, Dir};
use std::collections::BTreeSet;

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
        match self {
            Some(t) => Some(t.reflect()),
            _ => None,
        }
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