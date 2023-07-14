mod misc;
mod szukstra_tal;

use std::collections::BTreeSet;

use crate::moves::MoveFacet::{Attacking, Checking};
use crate::moves::{Move, Moves};
use crate::parse::StringIndexMap;
use crate::position::Position;
use crate::Symmetric;
use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

type MoveSet = BTreeSet<Move>;

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Move> {
        let squares = StringIndexMap::squares();
        let pieces = StringIndexMap::pieces();
        let sides = StringIndexMap::sides();
        let corners = StringIndexMap::corners();
        match s.chars().next() {
            None => Err(anyhow!("Cannot parse move from empty string!")),
            Some(t) => match t {
                's' => Ok(Move::Normal {
                    moving: pieces.get(slice(s, 1, 2)),
                    from: squares.get(slice(s, 3, 2)),
                    dest: squares.get(slice(s, 5, 2)),
                    capture: pieces.get_op(slice(s, 7, 2)),
                }),
                'e' => Ok(Move::Enpassant {
                    side: sides.get(slice(s, 1, 1)),
                    from: squares.get(slice(s, 2, 2)),
                    dest: squares.get(slice(s, 4, 2)),
                    capture: squares.get(slice(s, 6, 2)),
                }),
                'p' => Ok(Move::Promote {
                    from: squares.get(slice(s, 1, 2)),
                    dest: squares.get(slice(s, 3, 2)),
                    promoted: pieces.get(slice(s, 5, 2)),
                    capture: pieces.get_op(slice(s, 7, 2)),
                }),
                'c' => Ok(Move::Castle { corner: corners.get(slice(s, 1, 2)) }),
                _ => Err(anyhow!("Cannot parse {} as a move", s)),
            },
        }
    }
}

fn slice(s: &str, skip: usize, take: usize) -> String {
    s.chars().skip(skip).take(take).collect::<String>()
}

#[derive(Debug, Copy, Clone)]
enum MoveType {
    All,
    Attacks,
    AttacksChecks,
}

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
    let board = case.board.parse::<Position>()?;

    let expected = vec![
        (MoveType::All, parse_moves(&case.expected_all)?),
        //(MoveType::Attacks, parse_moves(&case.expected_attacks)?),
        //(MoveType::AttacksChecks, parse_moves(&case.expected_attacks_checks)?),
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

fn execute_test_impl(board: Position, moves: ExpectedMoves) {
    for (computation_type, expected_moves) in moves.into_iter() {
        let under_test: MoveSet = match computation_type {
            MoveType::All => board.moves(Moves::All).into_iter().collect(),
            MoveType::Attacks => board.moves(Moves::AreAny(&[Attacking])).into_iter().collect(),
            MoveType::AttacksChecks => {
                board.moves(Moves::AreAny(&[Attacking, Checking])).into_iter().collect()
            }
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
    let left_sub_right: Vec<_> =
        expected.clone().difference(&actual).map(|m| format!("{:?}", m)).collect();
    let right_sub_left: Vec<_> =
        actual.clone().difference(&expected).map(|m| format!("{:?}", m)).collect();
    format!("E - A: {:?}, A - E: {:?}", left_sub_right, right_sub_left)
}

mod parsing_formatting_test {
    use std::str::FromStr;

    use crate::constants::square::*;
    use crate::constants::{class, corner, side};

    use crate::create_piece;
    use crate::moves::Move;

    #[test]
    fn standard() {
        assert_eq!(
            Move::Normal {
                moving: create_piece(side::W, class::P),
                from: E2,
                dest: E4,
                capture: None
            },
            Move::from_str("swpe2e4-").unwrap()
        );
        assert_eq!(
            Move::Normal {
                moving: create_piece(side::B, class::R),
                from: C4,
                dest: C2,
                capture: Some(create_piece(side::W, class::P)),
            },
            Move::from_str("sbrc4c2wp").unwrap()
        );
    }

    #[test]
    fn promotion() {
        assert_eq!(
            Move::Promote {
                from: E7,
                dest: E8,
                promoted: create_piece(side::W, class::Q),
                capture: None,
            },
            Move::from_str("pe7e8wq-").unwrap()
        );
        assert_eq!(
            Move::Promote {
                from: E7,
                dest: D8,
                promoted: create_piece(side::W, class::Q),
                capture: Some(create_piece(side::B, class::B)),
            },
            Move::from_str("pe7d8wqbb").unwrap()
        );
    }

    #[test]
    fn enpassant() {
        assert_eq!(
            Move::Enpassant { side: side::B, from: D4, dest: C3, capture: C4 },
            Move::from_str("ebd4c3c4").unwrap()
        );
    }

    #[test]
    fn castle() {
        assert_eq!(Move::Castle { corner: corner::BK }, Move::from_str("cbk").unwrap());
    }
}
