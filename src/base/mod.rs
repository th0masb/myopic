pub mod bitboard;
pub mod square;
pub mod dir;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

