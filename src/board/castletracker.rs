use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;

#[derive(Debug, PartialEq, Clone)]
struct CastleTracker {
    remaining_rights: CastleZoneSet,
    white_status: Option<&'static CastleZone>,
    black_status: Option<&'static CastleZone>,
}

impl CastleTracker {
    pub fn remaining_rights(&self) -> CastleZoneSet {
        self.remaining_rights
    }

    pub fn white_status(&self) -> Option<&'static CastleZone> {
        self.white_status
    }

    pub fn black_status(&self) -> Option<&'static CastleZone> {
        self.black_status
    }
}

