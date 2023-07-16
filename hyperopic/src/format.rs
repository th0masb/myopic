use crate::moves::Move;
use crate::parse::StringIndexMap;
use crate::piece_class;
use crate::position::{Position, CASTLING_DETAILS};
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        lazy_static! {
            static ref SQUARES: StringIndexMap = StringIndexMap::squares();
            static ref PIECES: StringIndexMap = StringIndexMap::uci_pieces();
        }
        write!(
            f,
            "{}",
            match self {
                Move::Null => "null".to_owned(),
                &Move::Normal { from, dest, .. } =>
                    format!("{}{}", SQUARES.format(from), SQUARES.format(dest)),
                &Move::Enpassant { from, dest, .. } =>
                    format!("{}{}", SQUARES.format(from), SQUARES.format(dest)),
                &Move::Castle { corner, .. } => {
                    let details = &CASTLING_DETAILS[corner];
                    let (from, dest) = details.king_line;
                    format!("{}{}", SQUARES.format(from), SQUARES.format(dest))
                }
                &Move::Promote { from, dest, promoted, .. } => {
                    let promote_class = PIECES.format(piece_class(promoted));
                    format!("{}{}{}", SQUARES.format(from), SQUARES.format(dest), promote_class)
                }
            }
        )
    }
}
