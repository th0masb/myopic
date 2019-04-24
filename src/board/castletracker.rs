use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::Side;
use crate::base::bitboard::BitBoard;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleTracker {
    remaining_rights: CastleZoneSet,
    white_status: Option<&'static CastleZone>,
    black_status: Option<&'static CastleZone>,
}

impl CastleTracker {
    fn compute_rights_removed(move_components: BitBoard) -> CastleZoneSet {
        CastleZone::ALL.iter()
            .filter(|&x| move_components.intersects(x.source_squares()))
            .collect()
    }

    pub fn remove_rights(&mut self, move_components: BitBoard) -> CastleZoneSet {
        let to_remove = CastleTracker::compute_rights_removed(move_components);
        let removed = self.remaining_rights & to_remove;
        self.remaining_rights = self.remaining_rights - removed;
        removed
    }

    pub fn hash(&self) -> u64 {
        self.remaining_rights.hash()
    }
}

