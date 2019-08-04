use crate::board::implementation::castletracker::CastleTracker;
use crate::board::implementation::hashcache::HashCache;
use crate::board::implementation::piecetracker::PieceTracker;
use crate::board::implementation::BoardImpl;
use crate::board::test_board::TestBoard;
use crate::base::bitboard::BitBoard;

impl BoardImpl {
    pub fn from(test_board: TestBoard) -> BoardImpl {
        let pieces = PieceTracker::new(
            vec![test_board.whites, test_board.blacks]
                .iter()
                .flat_map(|x| x.into_iter())
                .map(|&x| x)
                .collect::<Vec<BitBoard>>().as_slice(),
        );
        let castling = CastleTracker::new(
            test_board.castle_rights,
            test_board.white_status,
            test_board.black_status,
        );
        let mut hashes = HashCache::new(0u64, 0);
        for i in 0..test_board.hash_offset {
            hashes.push_head(i as u64);
        }
        let mut result = BoardImpl {
            hashes,
            pieces,
            castling,
            active: test_board.active,
            enpassant: test_board.enpassant,
            clock: test_board.clock,
        };
        result.update_hash();
        result
    }
}
