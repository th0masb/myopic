use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Side;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleTracker {
    remaining_rights: CastleZoneSet,
    white_status: Option<&'static CastleZone>,
    black_status: Option<&'static CastleZone>,
}

impl CastleTracker {
    pub fn remove_rights(&mut self, rights: CastleZoneSet) {
        self.remaining_rights = self.remaining_rights - rights;
    }

    pub fn add_rights(&mut self, rights: CastleZoneSet) {
        self.remaining_rights = self.remaining_rights | rights;
    }

    pub fn set_status(&mut self, side: Side, status: &'static CastleZone) {
        match side {
            Side::White => self.white_status = Some(status),
            Side::Black => self.black_status = Some(status)
        }
    }

    pub fn remove_status(&mut self, side: Side) {
        match side {
            Side::White => self.white_status = None,
            Side::Black => self.black_status = None
        }
    }

    pub fn get_status(&self, side: Side) -> Option<&'static CastleZone> {
        match side {
            Side::White => self.white_status,
            Side::Black => self.black_status
        }
    }
}

