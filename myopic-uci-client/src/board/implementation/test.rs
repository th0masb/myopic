use crate::base::bitboard::BitBoard;
use crate::board::implementation::cache::CalculationCache;
use crate::board::implementation::castling::Castling;
use crate::board::implementation::history::History;
use crate::board::implementation::positions::Positions;
use crate::board::implementation::BoardImpl;
use crate::base::castlezone::{CastleZoneSet, CastleZone};
use crate::base::{Side, Reflectable};
use crate::base::square::Square;

#[derive(Debug, Clone)]
pub struct TestBoard {
    pub whites: Vec<BitBoard>,
    pub blacks: Vec<BitBoard>,
    pub castle_rights: CastleZoneSet,
    pub white_status: Option<CastleZone>,
    pub black_status: Option<CastleZone>,
    pub active: Side,
    pub clock: usize,
    pub enpassant: Option<Square>,
    pub history_count: usize,
}

impl Reflectable for TestBoard {
    fn reflect(&self) -> Self {
        TestBoard {
            whites: (&self.blacks).reflect(),
            blacks: (&self.whites).reflect(),
            castle_rights: self.castle_rights.reflect(),
            white_status: self.black_status.reflect(),
            black_status: self.white_status.reflect(),
            active: self.active.reflect(),
            clock: self.clock,
            enpassant: self.enpassant.reflect(),
            history_count: self.history_count,
        }
    }
}

impl BoardImpl {
    pub fn from(test_board: TestBoard) -> BoardImpl {
        let pieces = Positions::new(
            vec![test_board.whites, test_board.blacks]
                .iter()
                .flat_map(|x| x.into_iter())
                .map(|&x| x)
                .collect::<Vec<BitBoard>>()
                .as_slice(),
        );
        let castling = Castling::new(
            test_board.castle_rights,
            test_board.white_status,
            test_board.black_status,
        );
        let hash = super::hash(&pieces, &castling, test_board.active, test_board.enpassant);
        BoardImpl {
            history: History::new(hash, test_board.history_count),
            pieces,
            castling,
            active: test_board.active,
            enpassant: test_board.enpassant,
            clock: test_board.clock,
            cache: CalculationCache::empty(),
        }
    }
}
