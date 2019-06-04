use crate::base::bitboard::BitBoard;
use crate::base::square::Square;

pub const H1: BitBoard = Square::H1.lift();
pub const G1: BitBoard = Square::G1.lift();
pub const F1: BitBoard = Square::F1.lift();
pub const E1: BitBoard = Square::E1.lift();
pub const D1: BitBoard = Square::D1.lift();
pub const C1: BitBoard = Square::C1.lift();
pub const B1: BitBoard = Square::B1.lift();
pub const A1: BitBoard = Square::A1.lift();

pub const H2: BitBoard = Square::H2.lift();
pub const G2: BitBoard = Square::G2.lift();
pub const F2: BitBoard = Square::F2.lift();
pub const E2: BitBoard = Square::E2.lift();
pub const D2: BitBoard = Square::D2.lift();
pub const C2: BitBoard = Square::C2.lift();
pub const B2: BitBoard = Square::B2.lift();
pub const A2: BitBoard = Square::A2.lift();

pub const H3: BitBoard = Square::H3.lift();
pub const G3: BitBoard = Square::G3.lift();
pub const F3: BitBoard = Square::F3.lift();
pub const E3: BitBoard = Square::E3.lift();
pub const D3: BitBoard = Square::D3.lift();
pub const C3: BitBoard = Square::C3.lift();
pub const B3: BitBoard = Square::B3.lift();
pub const A3: BitBoard = Square::A3.lift();

pub const H4: BitBoard = Square::H4.lift();
pub const G4: BitBoard = Square::G4.lift();
pub const F4: BitBoard = Square::F4.lift();
pub const E4: BitBoard = Square::E4.lift();
pub const D4: BitBoard = Square::D4.lift();
pub const C4: BitBoard = Square::C4.lift();
pub const B4: BitBoard = Square::B4.lift();
pub const A4: BitBoard = Square::A4.lift();

pub const H5: BitBoard = Square::H5.lift();
pub const G5: BitBoard = Square::G5.lift();
pub const F5: BitBoard = Square::F5.lift();
pub const E5: BitBoard = Square::E5.lift();
pub const D5: BitBoard = Square::D5.lift();
pub const C5: BitBoard = Square::C5.lift();
pub const B5: BitBoard = Square::B5.lift();
pub const A5: BitBoard = Square::A5.lift();

pub const H6: BitBoard = Square::H6.lift();
pub const G6: BitBoard = Square::G6.lift();
pub const F6: BitBoard = Square::F6.lift();
pub const E6: BitBoard = Square::E6.lift();
pub const D6: BitBoard = Square::D6.lift();
pub const C6: BitBoard = Square::C6.lift();
pub const B6: BitBoard = Square::B6.lift();
pub const A6: BitBoard = Square::A6.lift();

pub const H7: BitBoard = Square::H7.lift();
pub const G7: BitBoard = Square::G7.lift();
pub const F7: BitBoard = Square::F7.lift();
pub const E7: BitBoard = Square::E7.lift();
pub const D7: BitBoard = Square::D7.lift();
pub const C7: BitBoard = Square::C7.lift();
pub const B7: BitBoard = Square::B7.lift();
pub const A7: BitBoard = Square::A7.lift();

pub const H8: BitBoard = Square::H8.lift();
pub const G8: BitBoard = Square::G8.lift();
pub const F8: BitBoard = Square::F8.lift();
pub const E8: BitBoard = Square::E8.lift();
pub const D8: BitBoard = Square::D8.lift();
pub const C8: BitBoard = Square::C8.lift();
pub const B8: BitBoard = Square::B8.lift();
pub const A8: BitBoard = Square::A8.lift();

pub static SQUARES: [BitBoard; 64] = [
    H1, G1, F1, E1, D1, C1, B1, A1, H2, G2, F2, E2, D2, C2, B2, A2, H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4, H5, G5, F5, E5, D5, C5, B5, A5, H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7, H8, G8, F8, E8, D8, C8, B8, A8,
];
