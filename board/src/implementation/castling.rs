use myopic_core::*;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Castling {
    remaining_rights: CastleZoneSet,
    // TODO Do we actually need to keep these fields?
    white_status: Option<CastleZone>,
    black_status: Option<CastleZone>,
}

impl Reflectable for Castling {
    fn reflect(&self) -> Self {
        Castling {
            remaining_rights: self.remaining_rights.reflect(),
            white_status: self.white_status.reflect(),
            black_status: self.black_status.reflect(),
        }
    }
}

fn compute_rights_removed(move_components: BitBoard) -> CastleZoneSet {
    CastleZone::iter()
        .filter(|x| move_components.intersects(x.source_squares()))
        .collect()
}

impl Castling {
    pub fn from_fen(fen_string: String) -> Result<Castling, String> {
        if !crate::parse::patterns::fen_rights().is_match(&fen_string) {
            Err(fen_string)
        } else {
            let rights: CastleZoneSet = CastleZone::iter()
                .zip(vec!["K", "Q", "k", "q"].into_iter())
                .filter(|(_, pat)| fen_string.contains(pat))
                .map(|(z, _)| z)
                .collect();
            let white_status = if rights.intersects(CastleZoneSet::WHITE) {
                None
            } else {
                Some(CastleZone::WK)
            };
            let black_status = if rights.intersects(CastleZoneSet::BLACK) {
                None
            } else {
                Some(CastleZone::BK)
            };
            Ok(Castling {
                remaining_rights: rights,
                white_status,
                black_status,
            })
        }
    }

    #[cfg(test)]
    pub fn new(
        rights: CastleZoneSet,
        white_status: Option<CastleZone>,
        black_status: Option<CastleZone>,
    ) -> Castling {
        Castling {
            remaining_rights: rights,
            black_status,
            white_status,
        }
    }

    pub fn set_status(&mut self, side: Side, zone: CastleZone) -> CastleZoneSet {
        match side {
            Side::White => self.white_status = Some(zone),
            Side::Black => self.black_status = Some(zone),
        };
        let rights_removed = self.remaining_rights
            & match side {
                Side::White => CastleZoneSet::WHITE,
                Side::Black => CastleZoneSet::BLACK,
            };
        self.remaining_rights -= rights_removed;
        rights_removed
    }

    pub fn clear_status(&mut self, side: Side) {
        match side {
            Side::White => self.white_status = None,
            Side::Black => self.black_status = None,
        }
    }

    pub fn remove_rights(&mut self, move_components: BitBoard) -> CastleZoneSet {
        let to_remove = compute_rights_removed(move_components);
        let removed = self.remaining_rights & to_remove;
        self.remaining_rights = self.remaining_rights - removed;
        removed
    }

    pub fn add_rights(&mut self, rights: CastleZoneSet) {
        self.remaining_rights = self.remaining_rights | rights;
    }

    pub fn hash(&self) -> u64 {
        hash::zones(self.remaining_rights)
    }

    pub fn rights(&self) -> CastleZoneSet {
        self.remaining_rights
    }

    pub fn status(&self, side: Side) -> Option<CastleZone> {
        match side {
            Side::White => self.white_status,
            Side::Black => self.black_status,
        }
    }
}
