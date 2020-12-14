use crate::implementation::cache::CalculationCache;
use crate::implementation::castling::Castling;
use crate::implementation::history::History;
use crate::implementation::positions::Positions;
use crate::implementation::MutBoardImpl;
use myopic_core::*;

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

impl MutBoardImpl {
    pub fn from(test_board: TestBoard) -> MutBoardImpl {
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
        MutBoardImpl {
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
