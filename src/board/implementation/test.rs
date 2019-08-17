use crate::base::bitboard::BitBoard;
use crate::board::implementation::castling::Castling;
use crate::board::implementation::history::History;
use crate::board::implementation::positions::Positions;
use crate::board::implementation::BoardImpl;
use crate::board::test_board::TestBoard;
use crate::board::implementation::cache::CalculationCache;

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
