use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::square::Square;
use crate::base::square;
use crate::base::Side;
use crate::board::castletracker::CastleTracker;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::board::Board;
use crate::board::Move;
use crate::pieces::Piece;
use crate::pieces;

fn reflect_square(input: Square) -> Square {
    let (fi, ri) = (input.file_index(), input.rank_index());
    square::constants::SQUARES[(8 * (7 - ri) + fi) as usize]
}

fn reflect_zone(input: CastleZone) -> CastleZone {
    CastleZone::ALL[(input.i() + 2) % 4]
}

fn reflect_piece(input: Piece) -> Piece {
    pieces::ALL[(input.index() + 6) % 12]
}

pub fn reflect_move(input: &Move) -> Move {
    let (rs, rp) = (reflect_square, reflect_piece);
    match input {
        Move::Castle(zone) => Move::castle(reflect_zone(*zone)),
        Move::Enpassant(square) => Move::Enpassant(rs(*square)),
        Move::Standard(p, s, t) => Move::Standard(rp(*p), rs(*s), rs(*t)),
        Move::Promotion(s, t, p) => Move::Promotion(rs(*s), rs(*t), rp(*p)),
    }
}

pub fn reflect_moves(input: &Vec<Move>) -> Vec<Move> {
    input.into_iter().map(reflect_move).collect()
}

pub fn reflect_bitboard(input: BitBoard) -> BitBoard {
    input.into_iter().map(reflect_square).collect()
}

pub fn reflect_bitboards(input: &Vec<BitBoard>) -> Vec<BitBoard> {
    input.into_iter().map(|&set| reflect_bitboard(set)).collect()
}

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

impl TestBoard {

    pub fn reflect(&self) -> TestBoard {
        TestBoard {
            whites: reflect_bitboards(&self.blacks),
            blacks: reflect_bitboards(&self.whites),
            castle_rights: self.castle_rights.iter().map(reflect_zone).collect(),
            white_status: self.black_status.map(reflect_zone),
            black_status: self.white_status.map(reflect_zone),
            active: self.active.other(),
            clock: self.clock,
            enpassant: self.enpassant.map(reflect_square),
            hash_offset: self.hash_offset
        }
    }

    pub fn to_board(self) -> Board {
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
        let mut result = Board {
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
