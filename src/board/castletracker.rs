use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Side;
use crate::base::bitboard::BitBoard;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleTracker {
    remaining_rights: CastleZoneSet,
    white_status: Option<CastleZone>,
    black_status: Option<CastleZone>,
}

impl CastleTracker {
    fn compute_rights_removed(move_components: BitBoard) -> CastleZoneSet {
        CastleZone::ALL.iter()
            .filter(|&x| move_components.intersects(x.source_squares()))
            .collect()
    }

    pub fn set_status(&mut self, side: Side, zone: CastleZone) -> CastleZoneSet {
        let side_rights = match side {
            Side::White => CastleZoneSet::white(),
            Side::Black => CastleZoneSet::black(),
        };
        let rights_removed = self.remaining_rights & side_rights;
        self.remaining_rights -= rights_removed;
        if side == Side::White {
            self.white_status = Some(zone);
        } else {
            self.black_status = Some(zone);
        }
        rights_removed
    }

    pub fn clear_status(&mut self, side: Side) {
        match side {
            Side::White => self.white_status = None,
            Side::Black => self.black_status = None,
        }
    }

    pub fn remove_rights(&mut self, move_components: BitBoard) -> CastleZoneSet {
        let to_remove = CastleTracker::compute_rights_removed(move_components);
        let removed = self.remaining_rights & to_remove;
        self.remaining_rights = self.remaining_rights - removed;
        removed
    }

    pub fn add_rights(&mut self, rights: CastleZoneSet) {
        self.remaining_rights = self.remaining_rights | rights;
    }

    pub fn hash(&self) -> u64 {
        self.remaining_rights.hash()
    }
}

