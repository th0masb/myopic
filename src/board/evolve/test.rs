use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZoneSet;
use crate::base::castlezone::CastleZone;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::piecetracker::PieceTracker;
use std::iter;
use crate::board::castletracker::CastleTracker;
use crate::board::hashcache::HashCache;
use crate::board::Move;

#[derive(Debug, Clone)]
struct TestBoard {
    whites: Vec<BitBoard>,
    blacks: Vec<BitBoard>,
    castle_rights: CastleZoneSet,
    white_status: Option<CastleZone>,
    black_status: Option<CastleZone>,
    active: Side,
    clock: usize,
    enpassant: Option<Square>,
    hash_offset: usize,
}

impl TestBoard {
    fn to_board(self) -> Board {
        let pieces = PieceTracker::new(vec![self.whites, self.blacks].iter()
            .flat_map(|x| x.into_iter()).map(|&x| x).collect());
        let castling = CastleTracker::new(self.castle_rights, self.white_status, self.black_status);
        let mut hashes = HashCache::new(0u64);
        for i in 0..self.hash_offset {
            hashes.push_head(i as u64);
        }
        Board {
            hashes,
            pieces,
            castling,
            active: self.active,
            enpassant: self.enpassant,
            clock: self.clock,
        }
    }
}

#[derive(Debug, Clone)]
struct TestCase {
    action : Move,
    start: TestBoard,
    end: TestBoard,
}


fn check_case(test_case: &TestCase) {
    let action = test_case.action.clone();
    let start = test_case.start.clone().to_board();
    let end = test_case.end.clone().to_board();

    let mut subject = start.clone();
    let rev_data = subject.evolve(&action);
    assert_eq!(end, subject);
    subject.devolve(&action, rev_data);
    assert_eq!(start, subject);
}
