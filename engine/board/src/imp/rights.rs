use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use myopic_core::*;

use enum_map::{enum_map, EnumMap};
use enumset::EnumSet;
#[cfg(test)]
use enumset::enum_set;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Default)]
pub struct Rights(pub EnumMap<Side, EnumSet<Flank>>);

impl Reflectable for Rights {
    fn reflect(&self) -> Self {
        Rights(enum_map! {
            Side::W => self.0[Side::B].clone(),
            Side::B => self.0[Side::W].clone(),
        })
    }
}

impl FromStr for Rights {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !crate::parse::patterns::fen_rights().is_match(s) {
            Err(anyhow!("{}", s))
        } else {
            let mut rights = Rights::default();
            for &p in &["K", "Q", "k", "q"] {
                if s.contains(p) {
                    let (side, flank) = match p {
                        "K" => (Side::W, Flank::K),
                        "Q" => (Side::W, Flank::Q),
                        "k" => (Side::B, Flank::K),
                        "q" => (Side::B, Flank::Q),
                        _ => panic!()
                    };
                    rights.0[side].insert(flank);
                };
            }
            Ok(rights)
        }
    }
}

impl Rights {
    pub fn corners(&self) -> impl Iterator<Item = Corner> + '_ {
        self.0.iter().flat_map(|(s, flanks)| flanks.iter().map(move |f| Corner(s, f)))
    }

    pub fn apply_castling(&mut self, side: Side) {
        self.0[side] = EnumSet::empty();
    }

    pub fn remove_rights(&mut self, srcdest: BitBoard) {
        srcdest.iter().for_each(|square| {
            match square {
                Square::E1 => self.0[Side::W] = EnumSet::empty(),
                Square::E8 => self.0[Side::B] = EnumSet::empty(),
                Square::A1 => { self.0[Side::W].remove(Flank::Q); },
                Square::A8 => { self.0[Side::B].remove(Flank::Q); },
                Square::H1 => { self.0[Side::W].remove(Flank::K); },
                Square::H8 => { self.0[Side::B].remove(Flank::K); },
                _ => {}
            }
        });
    }
}

#[cfg(test)]
impl Rights {
    pub fn empty() -> Rights {
        Rights(enum_map! { Side::W => EnumSet::empty(), Side::B => EnumSet::empty() })
    }

    pub fn all() -> Rights {
        Rights(enum_map! { Side::W => EnumSet::all(), Side::B => EnumSet::all() })
    }

    pub fn flank(flank: Flank) -> Rights {
        Rights(enum_map! { Side::W => enum_set!(flank), Side::B => enum_set!(flank) })
    }

    pub fn side(side: Side) -> Rights {
        let mut rights = Rights::empty();
        rights.0[side] = EnumSet::all();
        rights
    }
}
