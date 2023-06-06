use std::cell::RefCell;
use std::cmp::max;
use std::str::FromStr;

use myopic_core::anyhow::{anyhow, Error, Result};
use myopic_core::*;

use crate::moves::parse_op;
use crate::parse::patterns;
use crate::private::cache::CalculationCache;
use crate::private::history::History;
use crate::private::positions::Positions;
use crate::private::rights::Rights;
use crate::Board;

pub(crate) mod cache;
pub(crate) mod evolve;
pub(crate) mod fen;
pub(crate) mod history;
pub(crate) mod moves;
pub(crate) mod positions;
pub(crate) mod rights;
#[cfg(test)]
mod test;

impl FromStr for Board {
    type Err = Error;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        if patterns::fen().is_match(&fen) {
            let space_split: Vec<_> =
                patterns::space().split(&fen).map(|s| s.trim().to_owned()).collect();
            let active = space_split[1].parse::<Side>()?;
            let curr_move = space_split[5].parse::<usize>()?;
            Ok(Board {
                pieces: space_split[0].parse()?,
                active: space_split[1].parse()?,
                rights: space_split[2].parse()?,
                enpassant: parse_op(space_split[3].as_str())?,
                clock: space_split[4].parse::<usize>()?,
                history: History::new(2 * (max(curr_move, 1) - 1) + (active as usize)),
                cache: RefCell::new(CalculationCache::default()),
            })
        } else {
            Err(anyhow!("Cannot parse FEN {}", fen))
        }
    }
}

// TODO move
pub(crate) fn hash(pos: &Positions, rights: &Rights, active: Side, ep: Option<Square>) -> u64 {
    let mut result = pos.hash() ^ hash::side(active) ^ ep.map_or(0u64, |x| hash::enpassant(x));
    rights.corners().for_each(|c| result ^= hash::zone(c));
    result
}

#[cfg(test)]
mod fen_test {

    use crate::private::test::TestBoard;
    use crate::Board;
    use crate::{Side, Square::*};

    use super::*;

    fn test(expected: TestBoard, fen_string: String) -> Result<()> {
        assert_eq!(Board::from(expected), fen_string.parse::<Board>()?);
        Ok(())
    }

    #[test]
    fn fen_to_board_case_1() -> Result<()> {
        let fen = "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19";
        let board = TestBoard {
            whites: vec![A3 | B2 | C4 | E4 | F2 | G2 | H2, C3 | D4, !!E2, D1 | F1, !!D2, !!G1],
            blacks: vec![A6 | B7 | D6 | E6 | F7 | G6 | H6, !!E7, C8 | G7, A8 | D8, !!C7, !!G8],
            castle_rights: Rights::empty(),
            clock: 3,
            active: Side::W,
            enpassant: None,
            history_count: 36,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_2() -> Result<()> {
        let fen = "rnb2rk1/ppp2ppp/4pq2/8/2PP4/5N2/PP3PPP/R2QKB1R w KQ - 2 9";
        let board = TestBoard {
            whites: vec![A2 | B2 | C4 | D4 | F2 | G2 | H2, !!F3, !!F1, A1 | H1, !!D1, !!E1],
            blacks: vec![A7 | B7 | C7 | E6 | F7 | G7 | H7, !!B8, !!C8, A8 | F8, !!F6, !!G8],
            castle_rights: Rights::side(Side::W),
            clock: 2,
            active: Side::W,
            enpassant: None,
            history_count: 16,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_3() -> Result<()> {
        let fen = "r1bqkbnr/ppp1pppp/n7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let board = TestBoard {
            whites: vec![
                A2 | B2 | C2 | D2 | E5 | F2 | G2 | H2,
                B1 | G1,
                C1 | F1,
                A1 | H1,
                !!D1,
                !!E1,
            ],
            blacks: vec![
                A7 | B7 | C7 | D5 | E7 | F7 | G7 | H7,
                A6 | G8,
                C8 | F8,
                A8 | H8,
                !!D8,
                !!E8,
            ],
            castle_rights: Rights::all(),
            clock: 0,
            active: Side::W,
            enpassant: Some(D6),
            history_count: 4,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_4() -> Result<()> {
        let fen = "r6k/p5pp/p1b2qnN/8/3Q4/2P1B3/PP4PP/R5K1 b - - 2 21";
        let board = TestBoard {
            whites: vec![A2 | B2 | C3 | G2 | H2, !!H6, !!E3, !!A1, !!D4, !!G1],
            blacks: vec![A7 | A6 | G7 | H7, !!G6, !!C6, !!A8, !!F6, !!H8],
            castle_rights: Rights::empty(),
            clock: 2,
            active: Side::B,
            enpassant: None,
            history_count: 41,
        };
        test(board, String::from(fen))
    }
}

impl Reflectable for Board {
    fn reflect(&self) -> Self {
        let start_hash = Board::default().hash();
        if self.history.historical_positions().next() == Some(start_hash) {
            // If we played from the start position we can reflect properly
            let mut reflected = Board::default();
            reflected.active = Side::B;
            for m in self.history.historical_moves() {
                reflected.make(m).unwrap()
            }
            reflected
        } else {
            // Otherwise we started from some intermediate position and we cannot keep our history
            let pieces = self.pieces.reflect();
            let rights = self.rights.reflect();
            let active = self.active.reflect();
            let enpassant = self.enpassant.reflect();
            //let hash = hash(&pieces, &rights, active, enpassant);
            Board {
                history: History::default(),
                clock: self.clock,
                pieces,
                rights,
                active,
                enpassant,
                cache: RefCell::new(CalculationCache::default()),
            }
        }
    }
}
