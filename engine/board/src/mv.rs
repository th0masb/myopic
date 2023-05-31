use std::fmt::{Display, Formatter};
use std::str::FromStr;

use myopic_core::{
    anyhow::{Error, Result},
    Reflectable, Side,
};

use crate::{CastleZone, Piece, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    Standard {
        source: u64,
        moving: Piece,
        from: Square,
        dest: Square,
        capture: Option<Piece>,
    },
    Enpassant {
        source: u64,
        side: Side,
        from: Square,
        dest: Square,
        capture: Square,
    },
    Promotion {
        source: u64,
        from: Square,
        dest: Square,
        promoted: Piece,
        capture: Option<Piece>,
    },
    Castle {
        source: u64,
        zone: CastleZone,
    },
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => write!(
                f,
                "s{}{}{}{}",
                moving,
                from,
                dest,
                match capture {
                    None => "-".to_string(),
                    Some(p) => p.to_string(),
                }
            ),
            Move::Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => write!(
                f,
                "p{}{}{}{}",
                from,
                dest,
                promoted,
                match capture {
                    None => "-".to_string(),
                    Some(p) => p.to_string(),
                }
            ),
            Move::Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => write!(f, "e{}{}{}{}", side, from, dest, capture),
            Move::Castle { zone, .. } => write!(f, "c{}", zone),
        }
    }
}

#[cfg(test)]
impl Move {
    pub fn from(s: &str, source: u64) -> Result<Move> {
        use myopic_core::anyhow::anyhow;
        match s.chars().next() {
            None => Err(anyhow!("Cannot parse move from empty string!")),
            Some(t) => match t {
                's' => Ok(Move::Standard {
                    source,
                    moving: slice(s, 1, 2).parse()?,
                    from: slice(s, 3, 2).parse()?,
                    dest: slice(s, 5, 2).parse()?,
                    capture: parse_op(slice(s, 7, 2).as_str())?,
                }),
                'e' => Ok(Move::Enpassant {
                    source,
                    side: slice(s, 1, 1).parse()?,
                    from: slice(s, 2, 2).parse()?,
                    dest: slice(s, 4, 2).parse()?,
                    capture: slice(s, 6, 2).parse()?,
                }),
                'p' => Ok(Move::Promotion {
                    source,
                    from: slice(s, 1, 2).parse()?,
                    dest: slice(s, 3, 2).parse()?,
                    promoted: slice(s, 5, 2).parse()?,
                    capture: parse_op(slice(s, 7, 2).as_str())?,
                }),
                'c' => Ok(Move::Castle {
                    source,
                    zone: slice(s, 1, 2).parse()?,
                }),
                _ => Err(anyhow!("Cannot parse {} as a move", s)),
            },
        }
    }
}

#[cfg(test)]
fn slice(s: &str, skip: usize, take: usize) -> String {
    s.chars().skip(skip).take(take).collect()
}

pub fn parse_op<F>(s: &str) -> Result<Option<F>>
where
    F: FromStr<Err = Error>,
{
    match s {
        "-" => Ok(None),
        _ => Ok(Some(FromStr::from_str(s)?)),
    }
}

#[cfg(test)]
mod test {
    use crate::mv::Move;
    use crate::{CastleZone, Piece, Square};

    use super::*;

    #[test]
    fn standard() -> Result<()> {
        assert_eq!(
            Move::Standard {
                source: 0u64,
                moving: Piece::WP,
                from: Square::E2,
                dest: Square::E4,
                capture: None,
            },
            Move::from("swpe2e4-", 0u64)?
        );
        assert_eq!(
            Move::Standard {
                source: 1u64,
                moving: Piece::BR,
                from: Square::C4,
                dest: Square::C2,
                capture: Some(Piece::WP),
            },
            Move::from("sbrc4c2wp", 1u64)?
        );
        Ok(())
    }

    #[test]
    fn promotion() -> Result<()> {
        assert_eq!(
            Move::Promotion {
                source: 0u64,
                from: Square::E7,
                dest: Square::E8,
                promoted: Piece::WQ,
                capture: None,
            },
            Move::from("pe7e8wq-", 0u64)?
        );
        assert_eq!(
            Move::Promotion {
                source: 1u64,
                from: Square::E7,
                dest: Square::D8,
                promoted: Piece::WQ,
                capture: Some(Piece::BB),
            },
            Move::from("pe7d8wqbb", 1u64)?
        );
        Ok(())
    }

    #[test]
    fn enpassant() -> Result<()> {
        assert_eq!(
            Move::Enpassant {
                source: 0,
                side: Side::Black,
                from: Square::D4,
                dest: Square::C3,
                capture: Square::C4,
            },
            Move::from("ebd4c3c4", 0u64)?
        );
        Ok(())
    }

    #[test]
    fn castle() -> Result<()> {
        assert_eq!(
            Move::Castle {
                source: 0,
                zone: CastleZone::BK,
            },
            Move::from("cbk", 0u64)?
        );
        Ok(())
    }
}

impl Move {
    pub fn moving_side(&self) -> Side {
        match self {
            &Move::Standard { moving, .. } => moving.side(),
            &Move::Enpassant { side, .. } => side,
            &Move::Promotion { promoted, .. } => promoted.side(),
            &Move::Castle { zone, .. } => zone.side(),
        }
    }

    pub fn source(&self) -> u64 {
        *match self {
            Move::Standard { source, .. } => source,
            Move::Enpassant { source, .. } => source,
            Move::Promotion { source, .. } => source,
            Move::Castle { source, .. } => source,
        }
    }

    /// Convert this move into a human readable uci long format string.
    pub fn uci_format(&self) -> String {
        match self {
            Move::Standard { from, dest, .. } => format!("{}{}", from, dest),
            Move::Enpassant { from, dest, .. } => format!("{}{}", from, dest),
            Move::Castle { zone, .. } => {
                let (_, src, dest) = zone.king_data();
                format!("{}{}", src, dest)
            }
            Move::Promotion {
                from,
                dest,
                promoted,
                ..
            } => format!(
                "{}{}{}",
                from,
                dest,
                match promoted {
                    Piece::WQ | Piece::BQ => "q",
                    Piece::WR | Piece::BR => "r",
                    Piece::WB | Piece::BB => "b",
                    Piece::WN | Piece::BN => "n",
                    _ => "",
                }
            ),
        }
    }

    // TODO move me somewhere better
    pub(crate) fn reflect_for(&self, new_source: u64) -> Move {
        match self {
            &Move::Standard {
                moving,
                dest,
                from,
                capture,
                ..
            } => Move::Standard {
                source: new_source,
                moving: moving.reflect(),
                dest: dest.reflect(),
                from: from.reflect(),
                capture: capture.reflect(),
            },
            &Move::Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => Move::Promotion {
                source: new_source,
                from: from.reflect(),
                dest: dest.reflect(),
                promoted: promoted.reflect(),
                capture: capture.reflect(),
            },
            &Move::Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => Move::Enpassant {
                source: new_source,
                side: side.reflect(),
                from: from.reflect(),
                dest: dest.reflect(),
                capture: capture.reflect(),
            },
            &Move::Castle { zone, .. } => Move::Castle {
                source: new_source,
                zone: zone.reflect(),
            },
        }
    }
}
