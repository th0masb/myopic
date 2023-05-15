use std::collections::btree_set::BTreeSet;

use anyhow::Result;
use itertools::Itertools;

use myopic_core::*;

use crate::imp::Board;
use crate::mv::Move;
use crate::ChessBoard;
use crate::MoveComputeType;

type MoveSet = BTreeSet<Move>;

#[cfg(test)]
mod misc;
#[cfg(test)]
mod szukstra_tal;

type ExpectedMoves = Vec<(MoveComputeType, MoveSet)>;

#[derive(Debug, Clone)]
struct TestCase {
    board: &'static str,
    expected_all: Vec<&'static str>,
    expected_attacks_checks: Vec<&'static str>,
    expected_attacks: Vec<&'static str>,
}

fn parse_moves(source: u64, encoded: &Vec<&str>) -> Result<BTreeSet<Move>> {
    let mut dest = BTreeSet::new();
    for &s in encoded {
        dest.insert(Move::from(s, source)?);
    }
    Ok(dest)
}

fn execute_test(case: TestCase) -> Result<()> {
    let board = case.board.parse::<Board>()?;
    let board_hash = board.hash();
    let expected = vec![
        (
            MoveComputeType::All,
            parse_moves(board_hash, &case.expected_all)?,
        ),
        (
            MoveComputeType::Attacks,
            parse_moves(board_hash, &case.expected_attacks)?,
        ),
        (
            MoveComputeType::AttacksChecks,
            parse_moves(board_hash, &case.expected_attacks_checks)?,
        ),
    ];

    let ref_board = board.reflect();
    let ref_hash = ref_board.hash();
    let ref_moves = expected
        .iter()
        .map(|(t, mvs)| {
            (
                *t,
                mvs.into_iter()
                    .map(|m| m.reflect_for(ref_hash))
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<Vec<_>>();

    execute_test_impl(board, expected);
    execute_test_impl(ref_board, ref_moves);
    Ok(())
}

fn execute_test_impl(board: Board, moves: ExpectedMoves) {
    for (computation_type, expected_moves) in moves.into_iter() {
        let actual_moves: MoveSet = board.compute_moves(computation_type).into_iter().collect();
        assert_eq!(
            expected_moves.clone(),
            actual_moves.clone(),
            "Differences for {:?} are: {}",
            computation_type,
            format_difference(expected_moves, actual_moves)
        );
    }
}

fn format_difference(expected: MoveSet, actual: MoveSet) -> String {
    let left_sub_right = expected
        .clone()
        .difference(&actual)
        .map(|m| m.to_string())
        .collect_vec();
    let right_sub_left = actual
        .clone()
        .difference(&expected)
        .map(|m| m.to_string())
        .collect_vec();
    format!("E - A: {:?}, A - E: {:?}", left_sub_right, right_sub_left)
}
