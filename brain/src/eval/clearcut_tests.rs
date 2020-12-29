use crate::EvalBoard;
use anyhow::Result;
use myopic_board::ChessBoard;

#[test]
fn case_1() -> Result<()> {
    let uci_sequence = "e2e4 g8f6 b1c3 d7d5 e4e5 f6d7 d2d4 e7e6 g1f3 c7c5 f1d3 c5d4 f3d4 \
     f8c5 d4f3 b8c6 e1g1 d7e5 f3e5 c6e5 d3b5";
    let mut state = EvalBoard::start();
    state.play_uci(uci_sequence)?;
    let search_outcome = crate::search(state, 4)?;
    assert_eq!("c8d7", search_outcome.best_move.uci_format().as_str());
    Ok(())
}
