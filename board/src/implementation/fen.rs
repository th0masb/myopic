use crate::MutBoard;
use myopic_core::castlezone::CastleZone;
use myopic_core::pieces::Piece;
use myopic_core::{Side, Square};

/// Convert a board to a FEN string minus the two time suffix components.
pub(super) fn to_timeless_impl<B: MutBoard>(board: &B) -> String {
    let mut dest = String::new();
    let mut empty_count = 0;
    for i in 0..64 {
        match board.piece(Square::from_index(63 - i)) {
            None => {
                empty_count += 1;
            }
            Some(piece) => {
                if empty_count > 0 {
                    dest.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                dest.push_str(piece_to_fen(piece));
            }
        }
        if i != 0 && ((i % 8) == 7) {
            if empty_count > 0 {
                dest.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            dest.push('/');
        }
    }
    dest.remove(dest.len() - 1);

    dest.push_str(&format!(" {}", side_to_fen(board.active())));
    let castle_rights = board.remaining_rights().iter().map(castlezone_to_fen).collect::<String>();
    if castle_rights.is_empty() {
        dest.push_str(&format!(" -"));
    } else {
        dest.push_str(&format!(" {}", castle_rights));
    }
    match board.enpassant() {
        None => dest.push_str(&format!(" -")),
        Some(s) => dest.push_str(&format!(" {}", s).to_lowercase()),
    }
    dest
}

fn castlezone_to_fen(zone: CastleZone) -> &'static str {
    match zone {
        CastleZone::WK => "K",
        CastleZone::BK => "k",
        CastleZone::WQ => "Q",
        CastleZone::BQ => "q",
    }
}

fn side_to_fen(side: Side) -> &'static str {
    match side {
        Side::White => "w",
        Side::Black => "b",
    }
}

fn piece_to_fen(piece: Piece) -> &'static str {
    match piece {
        Piece::WP => "P",
        Piece::BP => "p",
        Piece::WN => "N",
        Piece::BN => "n",
        Piece::WB => "B",
        Piece::BB => "b",
        Piece::WR => "R",
        Piece::BR => "r",
        Piece::WQ => "Q",
        Piece::BQ => "q",
        Piece::WK => "K",
        Piece::BK => "k",
    }
}

#[cfg(test)]
mod test {
    use super::to_timeless_impl;
    use crate::{parse, MutBoard};

    #[test]
    fn start_position() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
            to_timeless_impl(&crate::start_position())
        )
    }

    #[test]
    fn position_1() {
        assert_eq!(
            "r1bqk1nr/2pp1ppp/p1n5/1pb1p3/4P3/1B3N2/PPPP1PPP/RNBQ1RK1 b kq -",
            to_timeless_impl(
                &parse::position_from_pgn(
                    "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 b55. Bb3 Bc5 6. O-O"
                )
                .expect("pgn parse failed")
            )
        )
    }

    #[test]
    fn position_2_timeless() {
        assert_eq!(
            "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1 w Qq d6",
            to_timeless_impl(
                &parse::position_from_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5")
                    .expect("pgn parse failed")
            )
        )
    }

    #[test]
    fn position_2() {
        assert_eq!(
            "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1 w Qq d6 0 5",
            &parse::position_from_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5")
                .expect("pgn parse failed").to_fen()
        )
    }
}
