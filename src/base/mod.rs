pub mod bitboard;
pub mod square;
pub mod dir;
pub mod castlezone;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Side {
    White,
    Black,
}

