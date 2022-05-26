use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use myopic_core::*;

use crate::enumset::EnumSet;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Rights(pub EnumSet<CastleZone>);

impl Reflectable for Rights {
    fn reflect(&self) -> Self {
        Rights(self.0.reflect())
    }
}

impl FromStr for Rights {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !crate::parse::patterns::fen_rights().is_match(s) {
            Err(anyhow!("{}", s))
        } else {
            let rights = CastleZone::iter()
                .zip(vec!["K", "Q", "k", "q"].into_iter())
                .filter(|(_, pat)| s.contains(pat))
                .map(|(z, _)| z)
                .collect();
            Ok(Rights(rights))
        }
    }
}

fn compute_rights_removed(srcdest: BitBoard) -> EnumSet<CastleZone> {
    CastleZone::iter()
        .filter(|x| srcdest.intersects(x.source_squares()))
        .collect()
}

impl Rights {
    pub fn apply_castling(self, side: Side) -> Rights {
        Rights(
            self.0
                - match side {
                    Side::White => CastleZone::WK | CastleZone::WQ,
                    Side::Black => CastleZone::BK | CastleZone::BQ,
                },
        )
    }

    pub fn remove_rights(self, srcdest: BitBoard) -> Rights {
        Rights(self.0 - compute_rights_removed(srcdest))
    }
}
