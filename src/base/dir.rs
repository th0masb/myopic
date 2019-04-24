#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dir {
    pub dr: i8,
    pub df: i8,
}
impl Dir {
    pub fn opposite(self) -> Dir {
        Dir {dr: -self.dr, df: -self.df}
    }
}

pub const N: Dir = Dir { dr: 1, df: 0 };
pub const E: Dir = Dir { dr: 0, df: -1 };
pub const S: Dir = Dir { dr: -1, df: 0 };
pub const W: Dir = Dir { dr: 0, df: 1 };

pub const NE: Dir = Dir { dr: 1, df: -1 };
pub const SE: Dir = Dir { dr: -1, df: -1 };
pub const SW: Dir = Dir { dr: -1, df: 1 };
pub const NW: Dir = Dir { dr: 1, df: 1 };

pub const NNE: Dir = Dir { dr: 2, df: -1 };
pub const NEE: Dir = Dir { dr: 1, df: -2 };
pub const SEE: Dir = Dir { dr: -1, df: -2 };
pub const SSE: Dir = Dir { dr: -2, df: -1 };
pub const SSW: Dir = Dir { dr: -2, df: 1 };
pub const SWW: Dir = Dir { dr: -1, df: 2 };
pub const NWW: Dir = Dir { dr: 1, df: 2 };
pub const NNW: Dir = Dir { dr: 2, df: 1 };
