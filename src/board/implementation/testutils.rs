use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::implementation::{
    castletracker::CastleTracker, hashcache::HashCache, piecetracker::PieceTracker, BoardImpl,
};

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
    pub hash_offset: usize,
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
            hash_offset: self.hash_offset,
        }
    }
}

impl TestBoard {
    pub fn to_board(self) -> BoardImpl {
        let pieces = PieceTracker::new(
            vec![self.whites, self.blacks]
                .iter()
                .flat_map(|x| x.into_iter())
                .map(|&x| x)
                .collect(),
        );
        let castling = CastleTracker::new(self.castle_rights, self.white_status, self.black_status);
        let mut hashes = HashCache::new(0u64);
        for i in 0..self.hash_offset {
            hashes.push_head(i as u64);
        }
        let mut result = BoardImpl {
            hashes,
            pieces,
            castling,
            active: self.active,
            enpassant: self.enpassant,
            clock: self.clock,
        };
        result.update_hash();
        result
    }
}
