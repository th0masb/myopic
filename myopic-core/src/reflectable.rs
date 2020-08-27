use crate::Side;
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
