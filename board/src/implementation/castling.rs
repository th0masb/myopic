use anyhow::{anyhow, Error, Result};
use myopic_core::*;
use std::str::FromStr;

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

impl FromStr for Castling {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !crate::parse::patterns::fen_rights().is_match(s) {
            Err(anyhow!("{}", s))
        } else {
            let rights: CastleZoneSet = CastleZone::iter()
                .zip(vec!["K", "Q", "k", "q"].into_iter())
                .filter(|(_, pat)| s.contains(pat))
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
}

fn compute_rights_removed(move_components: BitBoard) -> CastleZoneSet {
    CastleZone::iter()
        .filter(|x| move_components.intersects(x.source_squares()))
        .collect()
}

impl Castling {
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

    pub fn set_status(&mut self, zone: CastleZone) {
        match zone.side() {
            Side::White => {
                self.white_status = Some(zone);
                self.remaining_rights -= CastleZoneSet::WHITE;
            }
            Side::Black => {
                self.black_status = Some(zone);
                self.remaining_rights -= CastleZoneSet::BLACK;
            }
        }
    }

    pub fn clear_status(&mut self, side: Side) {
        match side {
            Side::White => self.white_status = None,
            Side::Black => self.black_status = None,
        }
    }

    pub fn remove_rights(&mut self, move_components: BitBoard) {
        self.remaining_rights = self.remaining_rights - compute_rights_removed(move_components);
    }

    pub fn set_rights(&mut self, rights: CastleZoneSet) {
        self.remaining_rights = rights;
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
