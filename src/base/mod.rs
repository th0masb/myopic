pub mod bitboard;
pub mod square;
pub mod dir;
pub mod castlezone;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

