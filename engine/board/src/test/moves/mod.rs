use std::collections::btree_set::BTreeSet;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

use itertools::Itertools;
use MoveFacet::Attacking;

use myopic_core::anyhow::{anyhow, Result};
use myopic_core::Corner;
use myopic_core::*;

use crate::anyhow::Error;
use crate::parse::parse_option;
use crate::{Board, MoveFacet, Moves};
use crate::Move;
use crate::MoveFacet::Checking;

mod misc;
mod szukstra_tal;

type MoveSet = BTreeSet<Move>;

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard { moving, from, dest, capture } => {
                let formatted_capture = capture.map_or("-".to_string(), |p| p.to_string());
                write!(f, "s{}{}{}{}", moving, from, dest, formatted_capture)
            }
            Move::Promotion { from, dest, promoted, capture } => {
                let formatted_capture = capture.map_or("-".to_string(), |p| p.to_string());
                write!(f, "p{}{}{}{}", from, dest, promoted, formatted_capture)
            }
            Move::Enpassant { side, from, dest, capture } => {
                write!(f, "e{}{}{}{}", side, from, dest, capture)
            }
            Move::Castle { corner, .. } => write!(f, "c{}{:?}", corner.0, corner.1),
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Move> {
        match s.chars().next() {
            None => Err(anyhow!("Cannot parse move from empty string!")),
            Some(t) => match t {
                's' => Ok(Move::Standard {
                    moving: slice(s, 1, 2).parse()?,
                    from: slice(s, 3, 2).parse()?,
                    dest: slice(s, 5, 2).parse()?,
                    capture: parse_option(slice(s, 7, 2).as_str())?,
                }),
                'e' => Ok(Move::Enpassant {
                    side: slice(s, 1, 1).parse()?,
                    from: slice(s, 2, 2).parse()?,
                    dest: slice(s, 4, 2).parse()?,
                    capture: slice(s, 6, 2).parse()?,
                }),
                'p' => Ok(Move::Promotion {
                    from: slice(s, 1, 2).parse()?,
                    dest: slice(s, 3, 2).parse()?,
                    promoted: slice(s, 5, 2).parse()?,
                    capture: parse_option(slice(s, 7, 2).as_str())?,
                }),
                'c' => Ok(Move::Castle {
                    corner: Corner(slice(s, 1, 1).parse()?, slice(s, 2, 1).parse()?),
                }),
                _ => Err(anyhow!("Cannot parse {} as a move", s)),
            },
        }
    }
}

fn slice(s: &str, skip: usize, take: usize) -> String {
    s.chars().skip(skip).take(take).collect()
}

#[derive(Debug, Copy, Clone)]
enum MoveType { All, Attacks, AttacksChecks }

type ExpectedMoves = Vec<(MoveType, MoveSet)>;

#[derive(Debug, Clone)]
struct TestCase {
    board: &'static str,
    expected_all: Vec<&'static str>,
    expected_attacks_checks: Vec<&'static str>,
    expected_attacks: Vec<&'static str>,
}

fn parse_moves(encoded: &Vec<&str>) -> Result<BTreeSet<Move>> {
    let mut dest = BTreeSet::new();
    for &s in encoded {
        dest.insert(Move::from_str(s)?);
    }
    Ok(dest)
}

fn execute_test(case: TestCase) -> Result<()> {
    let board = case.board.parse::<Board>()?;

    let expected = vec![
        (MoveType::All, parse_moves(&case.expected_all)?),
        (MoveType::Attacks, parse_moves(&case.expected_attacks)?),
        (MoveType::AttacksChecks, parse_moves(&case.expected_attacks_checks)?),
    ];

    let ref_board = board.reflect();
    let ref_moves = expected
        .iter()
        .map(|(t, mvs)| (*t, mvs.into_iter().map(|m| m.reflect()).collect::<BTreeSet<_>>()))
        .collect::<Vec<_>>();

    execute_test_impl(board, expected);
    execute_test_impl(ref_board, ref_moves);
    Ok(())
}

fn execute_test_impl(board: Board, moves: ExpectedMoves) {
    for (computation_type, expected_moves) in moves.into_iter() {
        let under_test: MoveSet = match computation_type {
            MoveType::All => board.moves(Moves::All).into_iter().collect(),
            MoveType::Attacks => board.moves(Moves::Are(Attacking)).into_iter().collect(),
            MoveType::AttacksChecks => board.moves(Moves::AreAny(&[Attacking, Checking])).into_iter().collect(),
        };
        assert_eq!(
            expected_moves.clone(),
            under_test.clone(),
            "Differences for {:?} are: {}",
            computation_type,
            format_difference(expected_moves, under_test)
        );
    }
}

fn format_difference(expected: MoveSet, actual: MoveSet) -> String {
    let left_sub_right = expected.clone().difference(&actual).map(|m| m.to_string()).collect_vec();
    let right_sub_left = actual.clone().difference(&expected).map(|m| m.to_string()).collect_vec();
    format!("E - A: {:?}, A - E: {:?}", left_sub_right, right_sub_left)
}

mod parsing_formatting_test {
    use std::str::FromStr;

    use myopic_core::{Class, Corner, Flank, Side};
    use Square::*;

    use crate::anyhow::Result;
    use crate::Move;
    use crate::{Piece, Square};

    #[test]
    fn standard() -> Result<()> {
        assert_eq!(
            Move::Standard { moving: Piece(Side::W, Class::P), from: E2, dest: E4, capture: None },
            Move::from_str("swpe2e4-")?
        );
        assert_eq!(
            Move::Standard {
                moving: Piece(Side::B, Class::R),
                from: C4,
                dest: C2,
                capture: Some(Piece(Side::W, Class::P)),
            },
            Move::from_str("sbrc4c2wp")?
        );
        Ok(())
    }

    #[test]
    fn promotion() -> Result<()> {
        assert_eq!(
            Move::Promotion {
                from: E7,
                dest: E8,
                promoted: Piece(Side::W, Class::Q),
                capture: None,
            },
            Move::from_str("pe7e8wq-")?
        );
        assert_eq!(
            Move::Promotion {
                from: E7,
                dest: D8,
                promoted: Piece(Side::W, Class::Q),
                capture: Some(Piece(Side::B, Class::B)),
            },
            Move::from_str("pe7d8wqbb")?
        );
        Ok(())
    }

    #[test]
    fn enpassant() -> Result<()> {
        assert_eq!(
            Move::Enpassant { side: Side::B, from: D4, dest: C3, capture: C4 },
            Move::from_str("ebd4c3c4")?
        );
        Ok(())
    }

    #[test]
    fn castle() -> Result<()> {
        assert_eq!(Move::Castle { corner: Corner(Side::B, Flank::K) }, Move::from_str("cbk")?);
        Ok(())
    }
}
