pub struct Dir {
    pub dr: i8,
    pub df: i8,
}

pub const N: Dir = Dir { dr:  1, df:  0 };
pub const E: Dir = Dir { dr:  0, df: -1 };
pub const S: Dir = Dir { dr: -1, df:  0 };
pub const W: Dir = Dir { dr:  0, df:  1 };

pub const NE: Dir = Dir { dr:  1, df: -1 };
pub const SE: Dir = Dir { dr: -1, df: -1 };
// Finishe the rest
