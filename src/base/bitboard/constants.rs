use crate::base::bitboard::BitBoard;
use crate::base::square::constants;

pub const H1: BitBoard = constants::H1.lift();
pub const G1: BitBoard = constants::G1.lift();
pub const F1: BitBoard = constants::F1.lift();
pub const E1: BitBoard = constants::E1.lift();
pub const D1: BitBoard = constants::D1.lift();
pub const C1: BitBoard = constants::C1.lift();
pub const B1: BitBoard = constants::B1.lift();
pub const A1: BitBoard = constants::A1.lift();

pub const H2: BitBoard = constants::H2.lift();
pub const G2: BitBoard = constants::G2.lift();
pub const F2: BitBoard = constants::F2.lift();
pub const E2: BitBoard = constants::E2.lift();
pub const D2: BitBoard = constants::D2.lift();
pub const C2: BitBoard = constants::C2.lift();
pub const B2: BitBoard = constants::B2.lift();
pub const A2: BitBoard = constants::A2.lift();

pub const H3: BitBoard = constants::H3.lift();
pub const G3: BitBoard = constants::G3.lift();
pub const F3: BitBoard = constants::F3.lift();
pub const E3: BitBoard = constants::E3.lift();
pub const D3: BitBoard = constants::D3.lift();
pub const C3: BitBoard = constants::C3.lift();
pub const B3: BitBoard = constants::B3.lift();
pub const A3: BitBoard = constants::A3.lift();

pub const H4: BitBoard = constants::H4.lift();
pub const G4: BitBoard = constants::G4.lift();
pub const F4: BitBoard = constants::F4.lift();
pub const E4: BitBoard = constants::E4.lift();
pub const D4: BitBoard = constants::D4.lift();
pub const C4: BitBoard = constants::C4.lift();
pub const B4: BitBoard = constants::B4.lift();
pub const A4: BitBoard = constants::A4.lift();

pub const H5: BitBoard = constants::H5.lift();
pub const G5: BitBoard = constants::G5.lift();
pub const F5: BitBoard = constants::F5.lift();
pub const E5: BitBoard = constants::E5.lift();
pub const D5: BitBoard = constants::D5.lift();
pub const C5: BitBoard = constants::C5.lift();
pub const B5: BitBoard = constants::B5.lift();
pub const A5: BitBoard = constants::A5.lift();

pub const H6: BitBoard = constants::H6.lift();
pub const G6: BitBoard = constants::G6.lift();
pub const F6: BitBoard = constants::F6.lift();
pub const E6: BitBoard = constants::E6.lift();
pub const D6: BitBoard = constants::D6.lift();
pub const C6: BitBoard = constants::C6.lift();
pub const B6: BitBoard = constants::B6.lift();
pub const A6: BitBoard = constants::A6.lift();

pub const H7: BitBoard = constants::H7.lift();
pub const G7: BitBoard = constants::G7.lift();
pub const F7: BitBoard = constants::F7.lift();
pub const E7: BitBoard = constants::E7.lift();
pub const D7: BitBoard = constants::D7.lift();
pub const C7: BitBoard = constants::C7.lift();
pub const B7: BitBoard = constants::B7.lift();
pub const A7: BitBoard = constants::A7.lift();

pub const H8: BitBoard = constants::H8.lift();
pub const G8: BitBoard = constants::G8.lift();
pub const F8: BitBoard = constants::F8.lift();
pub const E8: BitBoard = constants::E8.lift();
pub const D8: BitBoard = constants::D8.lift();
pub const C8: BitBoard = constants::C8.lift();
pub const B8: BitBoard = constants::B8.lift();
pub const A8: BitBoard = constants::A8.lift();

pub static SQUARES: [BitBoard; 64] = [
    H1, G1, F1, E1, D1, C1, B1, A1, H2, G2, F2, E2, D2, C2, B2, A2, H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4, H5, G5, F5, E5, D5, C5, B5, A5, H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7, H8, G8, F8, E8, D8, C8, B8, A8,
];
