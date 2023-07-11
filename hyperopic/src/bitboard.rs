use itertools::iterate;
use crate::{BitBoard, Dir, Square};
use crate::square::{lift, next};


pub const RANKS: [BitBoard; 8] = [
    255, // 1
    65280,
    16711680,
    4278190080,
    1095216660480,
    280375465082880,
    71776119061217280,
    18374686479671623680, // 8
];

pub const FILES: [BitBoard; 8] = [
    72340172838076673, // H
    144680345676153346,
    289360691352306692,
    578721382704613384,
    1157442765409226768,
    2314885530818453536,
    4629771061636907072,
    9259542123273814144, // A
];

pub const RIM: BitBoard = 72340172838076673 | 9259542123273814144 | 255 | 18374686479671623680;

pub const fn contains(board: BitBoard, square: Square) -> bool {
    board & lift(square) != 0
}

pub const fn covers(left: BitBoard, right: BitBoard) -> bool {
    (left & right) == right
}

pub const fn intersect(left: BitBoard, right: BitBoard) -> bool {
    (left & right) != 0
}

pub fn rays(source: Square, dirs: &[Dir]) -> BitBoard {
    dirs.iter()
        .flat_map(|&d| iterate(Some(source), move |op| op.and_then(|sq| next(sq, d))).take_while(|op| op.is_some()))
        .filter_map(|x| x)
        .fold(0u64, |a, n| a | lift(n))
}

