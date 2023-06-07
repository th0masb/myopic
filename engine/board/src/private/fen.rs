use myopic_core::*;

use crate::{Board, FenPart};

pub(crate) fn to_fen_impl(board: &Board, parts: &[FenPart]) -> String {
    let mut dest = String::new();
    for cmp in parts {
        let encoded_cmp = match *cmp {
            FenPart::Board => to_fen_board(board),
            FenPart::Active => to_fen_side(board),
            FenPart::CastlingRights => to_fen_castling_rights(board),
            FenPart::Enpassant => to_fen_enpassant(board),
            FenPart::HalfMoveCount => to_fen_half_move_count(board),
            FenPart::MoveCount => to_fen_move_count(board),
        };
        dest.push_str(encoded_cmp.as_str());
        dest.push(' ');
    }
    if !dest.is_empty() {
        dest.remove(dest.len() - 1);
    }
    dest
}

fn to_fen_board(board: &Board) -> String {
    let mut dest = String::new();
    let mut empty_count = 0;
    for i in 0..64 {
        match board.piece((63 - i).into()) {
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
    dest
}

fn to_fen_side(board: &Board) -> String {
    match board.active() {
        Side::W => "w".to_string(),
        Side::B => "b".to_string(),
    }
}

fn to_fen_castling_rights(board: &Board) -> String {
    let rights = board
        .remaining_rights()
        .into_iter()
        .flat_map(|(s, fs)| fs.into_iter().map(move |f| Corner(s, f)))
        .map(corner_to_fen)
        .collect::<String>();
    if rights.is_empty() {
        format!("-")
    } else {
        rights
    }
}

fn to_fen_enpassant(board: &Board) -> String {
    match board.enpassant() {
        None => format!("-"),
        Some(s) => format!("{}", s).to_lowercase(),
    }
}

fn to_fen_half_move_count(board: &Board) -> String {
    board.half_move_clock().to_string()
}

fn to_fen_move_count(board: &Board) -> String {
    (board.position_count() / 2 + 1).to_string()
}

fn corner_to_fen(Corner(side, flank): Corner) -> &'static str {
    match (side, flank) {
        (Side::W, Flank::K) => "K",
        (Side::B, Flank::K) => "k",
        (Side::W, Flank::Q) => "Q",
        (Side::B, Flank::Q) => "q",
    }
}

fn piece_to_fen(Piece(side, class): Piece) -> &'static str {
    match (side, class) {
        (Side::W, Class::P) => "P",
        (Side::B, Class::P) => "p",
        (Side::W, Class::N) => "N",
        (Side::B, Class::N) => "n",
        (Side::W, Class::B) => "B",
        (Side::B, Class::B) => "b",
        (Side::W, Class::R) => "R",
        (Side::B, Class::R) => "r",
        (Side::W, Class::Q) => "Q",
        (Side::B, Class::Q) => "q",
        (Side::W, Class::K) => "K",
        (Side::B, Class::K) => "k",
    }
}

#[cfg(test)]
mod test {
    use myopic_core::anyhow::Result;

    use crate::{Board, FenPart};

    use super::to_fen_impl;

    #[test]
    fn start_position_board() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::Board],)
        )
    }

    #[test]
    fn start_position_active() {
        assert_eq!(
            "w",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::Active],)
        )
    }

    #[test]
    fn start_position_castling_rights() {
        assert_eq!(
            "KQkq",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::CastlingRights],)
        )
    }

    #[test]
    fn start_position_enpassant() {
        assert_eq!(
            "-",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::Enpassant],)
        )
    }

    #[test]
    fn start_position_half_move_count() {
        assert_eq!(
            "0",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::HalfMoveCount],)
        )
    }

    #[test]
    fn start_position_move_count() {
        assert_eq!(
            "1",
            to_fen_impl(&crate::START_FEN.parse::<Board>().unwrap(), &[FenPart::MoveCount],)
        )
    }

    #[test]
    fn start_position_all() -> Result<()> {
        assert_eq!(
            crate::START_FEN,
            to_fen_impl(
                &crate::START_FEN.parse::<Board>()?,
                &[
                    FenPart::Board,
                    FenPart::Active,
                    FenPart::CastlingRights,
                    FenPart::Enpassant,
                    FenPart::HalfMoveCount,
                    FenPart::MoveCount,
                ],
            )
        );
        assert_eq!(crate::START_FEN, crate::START_FEN.parse::<Board>()?.to_fen());
        Ok(())
    }

    fn position_1() -> Board {
        let mut board = crate::start();
        board.play_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5").unwrap();
        board
    }

    #[test]
    fn position_1_board() {
        assert_eq!(
            "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1",
            to_fen_impl(&position_1(), &[FenPart::Board])
        )
    }

    #[test]
    fn position_1_active() {
        assert_eq!("w", to_fen_impl(&position_1(), &[FenPart::Active]))
    }

    #[test]
    fn position_1_castling_rights() {
        assert_eq!("Qq", to_fen_impl(&position_1(), &[FenPart::CastlingRights]))
    }

    #[test]
    fn position_1_enpassant() {
        assert_eq!("d6", to_fen_impl(&position_1(), &[FenPart::Enpassant]))
    }

    #[test]
    fn position_1_half_move_count() {
        assert_eq!("0", to_fen_impl(&position_1(), &[FenPart::HalfMoveCount]))
    }

    #[test]
    fn position_1_move_count() {
        assert_eq!("5", to_fen_impl(&position_1(), &[FenPart::MoveCount]))
    }

    #[test]
    fn position_1_all() {
        let expected = "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1 w Qq d6 0 5";
        assert_eq!(
            expected,
            to_fen_impl(
                &position_1(),
                &[
                    FenPart::Board,
                    FenPart::Active,
                    FenPart::CastlingRights,
                    FenPart::Enpassant,
                    FenPart::HalfMoveCount,
                    FenPart::MoveCount,
                ],
            )
        );
        assert_eq!(expected, position_1().to_fen());
    }

    fn position_2() -> Board {
        let mut board = crate::start();
        board.play_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5 5. Ke2 Kd7 6. Rh1").unwrap();
        board
    }

    #[test]
    fn position_2_board() {
        assert_eq!(
            "rnbq1br1/pppkppp1/5n1p/3pP3/8/5N2/PPPPKPPP/RNBQ1B1R",
            to_fen_impl(&position_2(), &[FenPart::Board])
        )
    }

    #[test]
    fn position_2_active() {
        assert_eq!("b", to_fen_impl(&position_2(), &[FenPart::Active]))
    }

    #[test]
    fn position_2_castling_rights() {
        assert_eq!("-", to_fen_impl(&position_2(), &[FenPart::CastlingRights]))
    }

    #[test]
    fn position_2_enpassant() {
        assert_eq!("-", to_fen_impl(&position_2(), &[FenPart::Enpassant]))
    }

    #[test]
    fn position_2_half_move_count() {
        assert_eq!("3", to_fen_impl(&position_2(), &[FenPart::HalfMoveCount]))
    }

    #[test]
    fn position_2_move_count() {
        assert_eq!("6", to_fen_impl(&position_2(), &[FenPart::MoveCount]))
    }

    #[test]
    fn position_2_all() {
        let expected = "rnbq1br1/pppkppp1/5n1p/3pP3/8/5N2/PPPPKPPP/RNBQ1B1R b - - 3 6";
        assert_eq!(
            expected,
            to_fen_impl(
                &position_2(),
                &[
                    FenPart::Board,
                    FenPart::Active,
                    FenPart::CastlingRights,
                    FenPart::Enpassant,
                    FenPart::HalfMoveCount,
                    FenPart::MoveCount,
                ],
            )
        );
        assert_eq!(expected, position_2().to_fen());
    }
}