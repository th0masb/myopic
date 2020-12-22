use crate::{ChessBoard, FenComponent};
use myopic_core::*;

pub(super) fn to_fen_impl<B: ChessBoard>(board: &B, cmps: &[FenComponent]) -> String {
    let mut dest = String::new();
    for cmp in cmps {
        let encoded_cmp = match *cmp {
            FenComponent::Board => to_fen_board(board),
            FenComponent::Active => to_fen_side(board),
            FenComponent::CastlingRights => to_fen_castling_rights(board),
            FenComponent::Enpassant => to_fen_enpassant(board),
            FenComponent::HalfMoveCount => to_fen_half_move_count(board),
            FenComponent::MoveCount => to_fen_move_count(board),
        };
        dest.push_str(encoded_cmp.as_str());
        dest.push(' ');
    }
    if !dest.is_empty() {
        dest.remove(dest.len() - 1);
    }
    dest
}

fn to_fen_board<B: ChessBoard>(board: &B) -> String {
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
    dest
}

fn to_fen_side<B: ChessBoard>(board: &B) -> String {
    match board.active() {
        Side::White => "w".to_string(),
        Side::Black => "b".to_string(),
    }
}

fn to_fen_castling_rights<B: ChessBoard>(board: &B) -> String {
    let rights = board
        .remaining_rights()
        .iter()
        .map(castlezone_to_fen)
        .collect::<String>();
    if rights.is_empty() {
        format!("-")
    } else {
        rights
    }
}

fn to_fen_enpassant<B: ChessBoard>(board: &B) -> String {
    match board.enpassant() {
        None => format!("-"),
        Some(s) => format!("{}", s).to_lowercase(),
    }
}

fn to_fen_half_move_count<B: ChessBoard>(board: &B) -> String {
    board.half_move_clock().to_string()
}

fn to_fen_move_count<B: ChessBoard>(board: &B) -> String {
    (board.position_count() / 2 + 1).to_string()
}

fn castlezone_to_fen(zone: CastleZone) -> &'static str {
    match zone {
        CastleZone::WK => "K",
        CastleZone::BK => "k",
        CastleZone::WQ => "Q",
        CastleZone::BQ => "q",
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
    use super::to_fen_impl;
    use crate::{parse, Board, ChessBoard, FenComponent};
    use anyhow::Result;

    #[test]
    fn start_position_board() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::Board]
            )
        )
    }

    #[test]
    fn start_position_active() {
        assert_eq!(
            "w",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::Active]
            )
        )
    }

    #[test]
    fn start_position_castling_rights() {
        assert_eq!(
            "KQkq",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::CastlingRights]
            )
        )
    }

    #[test]
    fn start_position_enpassant() {
        assert_eq!(
            "-",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::Enpassant]
            )
        )
    }

    #[test]
    fn start_position_half_move_count() {
        assert_eq!(
            "0",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::HalfMoveCount]
            )
        )
    }

    #[test]
    fn start_position_move_count() {
        assert_eq!(
            "1",
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>().unwrap(),
                &[FenComponent::MoveCount]
            )
        )
    }

    #[test]
    fn start_position_all() -> Result<()> {
        assert_eq!(
            crate::STARTPOS_FEN,
            to_fen_impl(
                &crate::STARTPOS_FEN.parse::<Board>()?,
                &[
                    FenComponent::Board,
                    FenComponent::Active,
                    FenComponent::CastlingRights,
                    FenComponent::Enpassant,
                    FenComponent::HalfMoveCount,
                    FenComponent::MoveCount,
                ]
            )
        );
        assert_eq!(
            crate::STARTPOS_FEN,
            crate::STARTPOS_FEN.parse::<Board>()?.to_fen()
        );
        Ok(())
    }

    fn position_1() -> Board {
        parse::position_from_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5").unwrap()
    }

    #[test]
    fn position_1_board() {
        assert_eq!(
            "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1",
            to_fen_impl(&position_1(), &[FenComponent::Board])
        )
    }

    #[test]
    fn position_1_active() {
        assert_eq!("w", to_fen_impl(&position_1(), &[FenComponent::Active]))
    }

    #[test]
    fn position_1_castling_rights() {
        assert_eq!(
            "Qq",
            to_fen_impl(&position_1(), &[FenComponent::CastlingRights])
        )
    }

    #[test]
    fn position_1_enpassant() {
        assert_eq!("d6", to_fen_impl(&position_1(), &[FenComponent::Enpassant]))
    }

    #[test]
    fn position_1_half_move_count() {
        assert_eq!(
            "0",
            to_fen_impl(&position_1(), &[FenComponent::HalfMoveCount])
        )
    }

    #[test]
    fn position_1_move_count() {
        assert_eq!("5", to_fen_impl(&position_1(), &[FenComponent::MoveCount]))
    }

    #[test]
    fn position_1_all() {
        let expected = "rnbqkbr1/ppp1ppp1/5n1p/3pP3/8/5N2/PPPP1PPP/RNBQKBR1 w Qq d6 0 5";
        assert_eq!(
            expected,
            to_fen_impl(
                &position_1(),
                &[
                    FenComponent::Board,
                    FenComponent::Active,
                    FenComponent::CastlingRights,
                    FenComponent::Enpassant,
                    FenComponent::HalfMoveCount,
                    FenComponent::MoveCount,
                ]
            )
        );
        assert_eq!(expected, position_1().to_fen());
    }

    fn position_2() -> Board {
        parse::position_from_pgn("1. e4 Nf6 2. Nf3 Rg8 3. Rg1 h6 4. e5 d5 5. Ke2 Kd7 6. Rh1")
            .unwrap()
    }

    #[test]
    fn position_2_board() {
        assert_eq!(
            "rnbq1br1/pppkppp1/5n1p/3pP3/8/5N2/PPPPKPPP/RNBQ1B1R",
            to_fen_impl(&position_2(), &[FenComponent::Board])
        )
    }

    #[test]
    fn position_2_active() {
        assert_eq!("b", to_fen_impl(&position_2(), &[FenComponent::Active]))
    }

    #[test]
    fn position_2_castling_rights() {
        assert_eq!(
            "-",
            to_fen_impl(&position_2(), &[FenComponent::CastlingRights])
        )
    }

    #[test]
    fn position_2_enpassant() {
        assert_eq!("-", to_fen_impl(&position_2(), &[FenComponent::Enpassant]))
    }

    #[test]
    fn position_2_half_move_count() {
        assert_eq!(
            "3",
            to_fen_impl(&position_2(), &[FenComponent::HalfMoveCount])
        )
    }

    #[test]
    fn position_2_move_count() {
        assert_eq!("6", to_fen_impl(&position_2(), &[FenComponent::MoveCount]))
    }

    #[test]
    fn position_2_all() {
        let expected = "rnbq1br1/pppkppp1/5n1p/3pP3/8/5N2/PPPPKPPP/RNBQ1B1R b - - 3 6";
        assert_eq!(
            expected,
            to_fen_impl(
                &position_2(),
                &[
                    FenComponent::Board,
                    FenComponent::Active,
                    FenComponent::CastlingRights,
                    FenComponent::Enpassant,
                    FenComponent::HalfMoveCount,
                    FenComponent::MoveCount,
                ]
            )
        );
        assert_eq!(expected, position_2().to_fen());
    }
}
