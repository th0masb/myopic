use crate::base::square::constants::A1;
use crate::base::square::constants::A8;
use crate::base::square::constants::B1;
use crate::base::square::constants::B8;
use crate::base::square::constants::C1;
use crate::base::square::constants::C8;
use crate::base::square::constants::E1;
use crate::base::square::constants::E8;
use crate::base::square::constants::F1;
use crate::base::square::constants::F8;
use crate::base::square::constants::G1;
use crate::base::square::constants::G8;
use crate::base::square::constants::H1;
use crate::base::square::constants::H8;
use crate::base::square::Square;

pub struct CastleZone {
    i: usize,
    king_source: Square,
    king_target: Square,
    rook_source: Square,
    rook_target: Square,
}

impl CastleZone {
    pub fn i(&self) -> usize {
        self.i
    }

    pub const WK: CastleZone = CastleZone {
        i: 0,
        king_source: E1,
        king_target: G1,
        rook_source: H1,
        rook_target: F1,
    };

    pub const WQ: CastleZone = CastleZone {
        i: 1,
        king_source: E1,
        king_target: B1,
        rook_source: A1,
        rook_target: C1,
    };

    pub const BK: CastleZone = CastleZone {
        i: 2,
        king_source: E8,
        king_target: G8,
        rook_source: H8,
        rook_target: F8,
    };

    pub const BQ: CastleZone = CastleZone {
        i: 3,
        king_source: E8,
        king_target: B8,
        rook_source: A8,
        rook_target: C8,
    };

    pub const ALL: [&'static CastleZone; 4] = [
        &CastleZone::WK,
        &CastleZone::WQ,
        &CastleZone::BK,
        &CastleZone::BQ,
    ];
}
