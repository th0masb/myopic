use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;

#[derive(Debug, Clone)]
pub struct TestBoard {
    pub whites: Vec<BitBoard>,
    pub blacks: Vec<BitBoard>,
    pub castle_rights: CastleZoneSet,
    pub white_status: Option<CastleZone>,
    pub black_status: Option<CastleZone>,
    pub active: Side,
    pub clock: usize,
    pub enpassant: Option<Square>,
    pub history_count: usize,
}

impl TestBoard {
    pub fn positions(whites: Vec<BitBoard>, blacks: Vec<BitBoard>) -> TestBoard {
        TestBoard {
            whites,
            blacks,
            castle_rights: CastleZoneSet::NONE,
            white_status: None,
            black_status: None,
            active: Side::White,
            clock: 5,
            enpassant: None,
            history_count: 20,
        }
    }
}

impl Reflectable for TestBoard {
    fn reflect(&self) -> Self {
        TestBoard {
            whites: (&self.blacks).reflect(),
            blacks: (&self.whites).reflect(),
            castle_rights: self.castle_rights.reflect(),
            white_status: self.black_status.reflect(),
            black_status: self.white_status.reflect(),
            active: self.active.reflect(),
            clock: self.clock,
            enpassant: self.enpassant.reflect(),
            history_count: self.history_count,
        }
    }
}

