use itertools::Itertools;

use myopic_board::anyhow::Result;

use crate::search::SearchParameters;
use crate::{Evaluator, TranspositionsImpl};

#[test]
fn case_1() -> Result<()> {
    let uci_sequence = "e2e4 g8f6 b1c3 d7d5 e4e5 f6d7 d2d4 e7e6 g1f3 c7c5 f1d3 c5d4 f3d4 \
     f8c5 d4f3 b8c6 e1g1 d7e5 f3e5 c6e5 d3b5";
    let mut state = Evaluator::default();
    state.play_uci(uci_sequence)?;
    let (depth, table_size) = (4, 10000);
    let search_outcome =
        crate::search(state, SearchParameters { terminator: depth, table: &mut TranspositionsImpl::new(table_size) })?;
    assert_eq!("c8d7", search_outcome.best_move.uci_format().as_str());
    Ok(())
}

#[test]
fn check_pv_length_is_depth() -> Result<()> {
    let uci_sequence = "e2e4 g7g6 d2d4 f8g7 c2c4 d7d6 b1c3 g8f6 g1f3 e8g8 f1d3"; // e7e5 f3d2 e5d4 d2b1 d4c3
    let mut state = Evaluator::default();
    state.play_uci(uci_sequence)?;
    let (depth, table_size) = (4, 10000);
    let search_outcome =
        crate::search(state, SearchParameters { terminator: depth, table: &mut TranspositionsImpl::new(table_size) })?;
    let path = search_outcome.optimal_path.iter().map(|m| m.uci_format()).collect_vec();
    assert_eq!(depth, path.len(), "{:?}", path);
    Ok(())
}
