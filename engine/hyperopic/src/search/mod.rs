use std::time::{Duration, Instant};

use serde::ser::SerializeStruct;
use serde::Serializer;

use anyhow::{anyhow, Result};
use end::SearchEnd;

use crate::moves::{Move, Moves};
use crate::node;
use crate::node::SearchNode;
use crate::position::{Position, TerminalState};
use crate::search::negascout::{Context, Scout, SearchResponse};
use crate::search::pv::PrincipleVariation;
pub use crate::search::transpositions::{Transpositions, TranspositionsImpl, TreeNode};

pub mod end;
mod moves;
pub mod negascout;
mod pv;
pub mod quiescent;
mod transpositions;

const DEPTH_UPPER_BOUND: usize = 20;

/// API function for executing search on the calling thread, we pass a root
/// state and a terminator and compute the best move we can make from this
/// state within the duration constraints implied by the terminator.
pub fn search<E: SearchEnd, T: Transpositions>(
    node: SearchNode,
    parameters: SearchParameters<E, T>,
) -> Result<SearchOutcome> {
    Search { node, end: parameters.end, transpositions: parameters.table }.search()
}

pub struct SearchParameters<'a, E: SearchEnd, T: Transpositions> {
    pub end: E,
    pub table: &'a mut T,
}

/// Data class composing information/result about/of a best move search.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOutcome {
    pub best_move: Move,
    /// Larger +ve score better for side to move
    pub relative_eval: i32,
    pub depth: u8,
    pub time: Duration,
    pub optimal_path: Vec<Move>,
}

impl serde::Serialize for SearchOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SearchOutcome", 4)?;
        state.serialize_field("bestMove", &self.best_move.to_string())?;
        state.serialize_field("positionEval", &self.relative_eval)?;
        state.serialize_field("depthSearched", &self.depth)?;
        state.serialize_field("searchDurationMillis", &self.time.as_millis())?;
        state.serialize_field(
            "optimalPath",
            &self.optimal_path.iter().map(|m| m.to_string()).collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod searchoutcome_serialize_test {
    use std::time::Duration;

    use serde_json;

    use crate::constants::create_piece;
    use crate::constants::{class, corner, side, square};
    use crate::moves::Move;

    use super::SearchOutcome;

    #[test]
    fn test_json_serialize() {
        let search_outcome = SearchOutcome {
            best_move: Move::Castle { corner: corner::WK },
            relative_eval: -125,
            depth: 2,
            time: Duration::from_millis(3000),
            optimal_path: vec![
                Move::Castle { corner: corner::WK },
                Move::Normal {
                    moving: create_piece(side::B, class::P),
                    from: square::D7,
                    dest: square::D5,
                    capture: None,
                },
            ],
        };
        assert_eq!(
            r#"{"bestMove":"e1g1","positionEval":-125,"depthSearched":2,"searchDurationMillis":3000,"optimalPath":["e1g1","d7d5"]}"#,
            serde_json::to_string(&search_outcome).expect("Serialization failed")
        );
    }
}

struct Search<'a, E: SearchEnd, T: Transpositions> {
    node: SearchNode,
    end: E,
    transpositions: &'a mut T,
}

struct BestMoveResponse {
    eval: i32,
    best_move: Move,
    path: Vec<Move>,
    depth: u8,
}

impl<E: SearchEnd, T: Transpositions> Search<'_, E, T> {
    pub fn search(&mut self) -> Result<SearchOutcome> {
        let search_start = Instant::now();
        let mut break_err = anyhow!("Terminated before search began");
        let mut pv = PrincipleVariation::default();
        let mut best_response = None;
        let risks_draw = contains_draw(&mut self.node.position().clone(), 2)?;

        for i in 1..DEPTH_UPPER_BOUND {
            match self.best_move(i as u8, search_start, &pv, risks_draw) {
                Err(message) => {
                    break_err = anyhow!("{}", message);
                    break;
                }
                Ok(response) => {
                    pv.set(response.path.as_slice());
                    best_response = Some(response);
                }
            }
        }

        best_response.ok_or(break_err).map(|response| SearchOutcome {
            best_move: response.best_move,
            relative_eval: response.eval,
            depth: response.depth,
            time: search_start.elapsed(),
            optimal_path: response.path,
        })
    }

    fn best_move(
        &mut self,
        depth: u8,
        search_start: Instant,
        pv: &PrincipleVariation,
        risks_draw: bool,
    ) -> Result<BestMoveResponse> {
        if depth < 1 {
            return Err(anyhow!("Cannot iteratively deepen with depth 0"));
        }

        // TODO If any move in the current position leads to a draw by repetition then disable the
        //  transposition table early break?
        let SearchResponse { eval, path } =
            Scout { end: &self.end, transpositions: self.transpositions, moves: pv.into() }
                .search(
                    &mut self.node,
                    Context {
                        depth,
                        start: search_start,
                        alpha: -node::INFTY,
                        beta: node::INFTY,
                        precursors: vec![],
                        // If there is potential for the position to be drawn based on the move we choose
                        // then disable early breaking using the transposition table
                        early_break_enabled: !risks_draw,
                    },
                )?;

        // If the path returned is empty then there must be no legal moves in this position
        if path.is_empty() {
            Err(anyhow!("No moves for position {} at depth {}", self.node.position(), depth))
        } else {
            Ok(BestMoveResponse { best_move: path.get(0).unwrap().clone(), eval, path, depth })
        }
    }
}

fn contains_draw(position: &mut Position, depth: usize) -> Result<bool> {
    Ok(match position.compute_terminal_state() {
        Some(TerminalState::Draw) => true,
        Some(TerminalState::Loss) => false,
        None => {
            if depth == 0 {
                false
            } else {
                for m in position.moves(&Moves::All) {
                    position.make(m)?;
                    let recursive = contains_draw(position, depth - 1)?;
                    position.unmake()?;
                    if recursive {
                        return Ok(true);
                    }
                }
                false
            }
        }
    })
}

#[cfg(test)]
mod test_contains_draw {
    use crate::position::Position;

    #[test]
    fn has_draw_0() {
        let pgn = "1. e4 c5 2. Nf3 d6 3. d4 cxd4 4. Nxd4 Nf6 5. Nc3 a6 6. f3 e5 7. Nf5 Bxf5 \
        8. exf5 Nc6 9. Bd3 Qb6 10. Na4 Qa5+ 11. Nc3 Be7 12. O-O Qb6+ 13. Rf2 d5 14. Na4 Qa5 \
        15. c4 b5 16. cxd5 bxa4 17. dxc6 Bc5 18. Qe2 Bxf2+ 19. Kxf2 O-O 20. Be3 Nd5 21. Rb1 Qc7 \
        22. Be4 Qxc6 23. Rc1 Qb7 24. Rc5 Rad8 25. a3 Rfe8 26. Kg3 Qb3 27. Bc1 Nf6 28. Bc2 Nh5+ \
        29. Kh3 Qb6 30. Rxe5 Rxe5 31. Qxe5 Qc6 32. Qc3 Qb5 33. Be3 Nf6 34. Qc7 Rf8 35. Bd4 h6 \
        36. Bxf6 gxf6 37. Qc3 Kg7 38. Qd4 Rb8 39. Bd3 Qxb2 40. Qg4+ Kh7 41. Qxa4 Rg8 42. g3 Rg5 \
        43. Qd7 Kg7 44. Bc4 Rh5+ 45. Kg4 Rg5+ 46. Kh3 Rh5+ 47. Kg4";
        let mut board: Position = pgn.parse().unwrap();
        assert_eq!(false, super::contains_draw(&mut board, 0).unwrap());
        assert_eq!(false, super::contains_draw(&mut board, 1).unwrap());
        assert_eq!(true, super::contains_draw(&mut board, 2).unwrap());
    }

    #[test]
    fn has_no_draw_0() {
        let pgn = "1. b3 d6 2. Bb2 e5 3. d4 Nd7 4. dxe5 dxe5 5. a4 Ngf6 6. Nf3 Bb4+ \
        7. c3 Bc5 8. Ba3 Bxa3 9. Rxa3 O-O 10. b4 e4 11. Nfd2 e3 12. fxe3 Ne5 13. Qc2 Nd5 \
        14. Qe4 Re8 15. b5 Nxe3 16. b6 Bf5 17. bxc7 Qxc7 18. Qxe3 Nd3+ 19. Qxd3 Bxd3 20. Rb3 Qc6 \
        21. Kd1 Bxe2+ 22. Bxe2 Rxe2 23. Kxe2 Qxg2+ 24. Kd3 Qxh1 25. Rb4 Qxh2 26. Rxb7 Qd6+ \
        27. Kc2 Qd5 28. Rb4 Qa2+ 29. Kd3 h6 30. Ke2 a5 31. Rd4 Rb8 32. Ke3 Re8+ 33. Kf3 Qa1 \
        34. Kg3 g5 35. Kg2 Rb8 36. Kf2 Kg7 37. Kf3 Rxb1 38. Nxb1 Qxb1 39. Ke3 Kg6 40. Rc4 f6 \
        41. Rc8 Qe1+ 42. Kd3 Qd1+ 43. Ke3 Qe1+ 44. Kd3 Qf1+ 45. Kd2 Qf4+ 46. Ke1 Qxa4 47. Rg8+ Kh5 \
        48. Rc8 Qf4 49. c4 a4";
        let mut board: Position = pgn.parse().unwrap();
        assert_eq!(false, super::contains_draw(&mut board, 0).unwrap());
        assert_eq!(false, super::contains_draw(&mut board, 1).unwrap());
        assert_eq!(false, super::contains_draw(&mut board, 2).unwrap());
    }
}
