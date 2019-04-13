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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct CastleZoneSet {
    data: usize,
}

impl CastleZoneSet {
    pub fn all() -> CastleZoneSet {
        CastleZoneSet {data: 0b1111}
    }

    pub fn none() -> CastleZoneSet {
        CastleZoneSet {data: 0}
    }

    pub fn contains(self, zone: &'static CastleZone) -> bool {
        (1usize << zone.i) & self.data != 0
    }

    pub fn add(&mut self, zone: &'static CastleZone) {
        self.data |= (1usize << zone.i)
    }

    pub fn remove(&mut self, zone: &'static CastleZone) {
        self.data &= !(1usize << zone.i)
    }
}

#[cfg(test)]
mod set_test {
    use super::*;

    #[test]
    fn test_all() {
        let all = CastleZoneSet::all();
        for &zone in &CastleZone::ALL {
            assert!(all.contains(zone));
        }
    }

    #[test]
    fn test_none() {
        let none = CastleZoneSet::none();
        for &zone in &CastleZone::ALL {
            assert!(!none.contains(zone));
        }
    }

    #[test]
    fn test_add() {
        let mut set = CastleZoneSet::none();
        assert!(!set.contains(&CastleZone::BK));
        set.add(&CastleZone::BK);
        assert!(set.contains(&CastleZone::BK));
    }

    #[test]
    fn test_remove() {
        let mut set = CastleZoneSet::all();
        assert!(set.contains(&CastleZone::WQ));
        set.remove(&CastleZone::WQ);
        assert!(!set.contains(&CastleZone::WQ));
    }
}
