use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::pieces::Piece;
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;
use std::iter;

const SEED: u64 = 0x110894u64;
const N_FEATURES: usize = 64 * 12 + 8 + 4 + 1;

/// O(n^2) complexity but hey ho.
pub fn gen_unique(count: usize) -> Vec<u64> {
    let mut prng = Mcg128Xsl64::seed_from_u64(SEED);
    let mut dest: Vec<u64> = Vec::with_capacity(count);
    while dest.len() < count {
        let attempt = prng.gen();
        if !dest.contains(&attempt) {
            dest.push(attempt);
        }
    }
    dest
}

lazy_static! {
    static ref FEATURES: Vec<u64> = gen_unique(N_FEATURES);
}

pub fn piece_feature(piece: &dyn Piece, square: Square) -> u64 {
    FEATURES[piece.index() * 64 + square.i as usize]
}

pub fn black_feature() -> u64 {
    FEATURES[N_FEATURES - 1]
}

pub fn enpassant_feature(square: Square) -> u64 {
    FEATURES[N_FEATURES - 6 - square.file()]
}

pub fn castle_feature(zone: CastleZone) -> u64 {
    FEATURES[N_FEATURES - 2 - zone.i()]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base::square::constants::SQUARES;
    use crate::pieces;

    #[test]
    fn test_uniqueness() {
        let mut dest: Vec<u64> = Vec::new();
        // add piece-square features
        for &piece in pieces::ALL.iter() {
            for &square in SQUARES.iter() {
                unique_add(&mut dest, piece_feature(piece, square));
            }
        }
        for &zone in CastleZone::ALL.iter() {
            unique_add(&mut dest, castle_feature(zone));
        }
        for &square in &SQUARES[..8] {
            unique_add(&mut dest, enpassant_feature(square));
        }
        unique_add(&mut dest, black_feature());
    }

    fn unique_add(dest: &mut Vec<u64>, next: u64) {
        assert!(!dest.contains(&next));
        dest.push(next);
    }
}
