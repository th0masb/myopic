use std::str::FromStr;
use anyhow::{anyhow, Error};
use myopic_board::FenComponent;
use serde_derive::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum PositionFormat {
    #[serde(rename = "uci")]
    UciSequence,
    #[serde(rename = "fen")]
    Fen {
        #[serde_as(as = "DisplayFromStr")]
        format: FenFormat,
    },
}

impl FromStr for PositionFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(Error::from)
    }
}

impl Default for PositionFormat {
    fn default() -> Self {
        PositionFormat::UciSequence
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FenFormat(pub Vec<FenComponent>);

impl FromStr for FenFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .chars()
            .flat_map(|c| match c {
                'b' => vec![FenComponent::Board],
                'a' => vec![FenComponent::Active],
                'c' => vec![FenComponent::CastlingRights],
                'e' => vec![FenComponent::Enpassant],
                'h' => vec![FenComponent::HalfMoveCount],
                'm' => vec![FenComponent::MoveCount],
                _ => vec![],
            })
            .collect::<Vec<_>>();
        if parsed.is_empty() {
            Err(anyhow!("No FEN component extracted from {}", s))
        } else {
            Ok(FenFormat(parsed))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::position::{PositionFormat, FenFormat};
    use myopic_board::FenComponent;

    #[test]
    fn deserialize() {
        assert_eq!(
            PositionFormat::UciSequence,
            serde_json::from_str::<PositionFormat>(r#"{"type":"uci"}"#).unwrap()
        );
        assert_eq!(
            PositionFormat::Fen { format: FenFormat(vec![FenComponent::Board, FenComponent::Active, FenComponent::CastlingRights]) },
            serde_json::from_str::<PositionFormat>(r#"{"type":"fen","format":"bac"}"#).unwrap()
        );
    }
}
