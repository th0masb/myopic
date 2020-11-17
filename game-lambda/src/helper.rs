use myopic_board::Move;
use myopic_core::pieces::Piece;
use std::time::Duration;

pub struct ThinkingTimeParams {
    pub expected_half_move_count: u32,
    pub half_moves_played: u32,
    pub initial: Duration,
    pub increment: Duration,
}

pub fn compute_thinking_time(params: ThinkingTimeParams) -> Duration {
    // Lets say we predict a chess game will last n moves before the game. Lets say the
    // time limit for the whole game is T seconds. Then for the first n / 2 moves allocate
    // T / n seconds. For the next n / 2 moves allocate T / 2n seconds and so on.
    //
    // So for move m, define i = m // (n / 2) and we allocate t_m = T / (i + 1)n seconds
    //
    // This can be modified to make the decrease in thinking time less sharp
    let i = params.half_moves_played / (params.expected_half_move_count / 2);
    params.increment + (params.initial / ((i + 1) * params.expected_half_move_count))
}

#[cfg(test)]
mod thinking_time_test {
    use super::{ThinkingTimeParams, compute_thinking_time};
    use std::time::Duration;

    #[test]
    fn test_first_move() {
        let params = ThinkingTimeParams {
            expected_half_move_count: 60,
            half_moves_played: 0,
            increment: Duration::from_secs(2),
            initial: Duration::from_secs(600),
        };
        assert_eq!(Duration::from_secs(12), compute_thinking_time(params))
    }
}

pub fn move_to_uci(mv: &Move) -> String {
    match mv {
        &Move::Standard(_, src, dest) => format!("{}{}", src, dest),
        &Move::Enpassant(src, dest) => format!("{}{}", src, dest),
        &Move::Promotion(src, dest, piece) => format!(
            "{}{}{}",
            src,
            dest,
            match piece {
                Piece::WQ | Piece::BQ => "q",
                Piece::WR | Piece::BR => "r",
                Piece::WB | Piece::BB => "b",
                Piece::WN | Piece::BN => "n",
                _ => "",
            }
        ),
        &Move::Castle(zone) => {
            let (_, src, dest) = zone.king_data();
            format!("{}{}", src, dest)
        }
    }
    .to_lowercase()
    .to_owned()
}

#[cfg(test)]
mod uci_conversion_test {
    use super::move_to_uci;
    use myopic_board::Move;
    use myopic_core::{pieces::Piece, Square};
    use myopic_core::castlezone::CastleZone;

    #[test]
    fn test_pawn_standard_conversion() {
        assert_eq!(
            "e2e4",
            move_to_uci(&Move::Standard(Piece::WP, Square::E2, Square::E4)).as_str()
        );
    }

    #[test]
    fn test_rook_standard_conversion() {
        assert_eq!(
            "h1h7",
            move_to_uci(&Move::Standard(Piece::BR, Square::H1, Square::H7)).as_str()
        );
    }

    #[test]
    fn test_castling_conversion() {
        assert_eq!("e1g1", move_to_uci(&Move::Castle(CastleZone::WK)).as_str());
        assert_eq!("e1c1", move_to_uci(&Move::Castle(CastleZone::WQ)).as_str());
        assert_eq!("e8g8", move_to_uci(&Move::Castle(CastleZone::BK)).as_str());
        assert_eq!("e8c8", move_to_uci(&Move::Castle(CastleZone::BQ)).as_str());
    }

    #[test]
    fn test_promotion_conversion() {
        assert_eq!(
            "e7d8q",
            move_to_uci(&Move::Promotion(Square::E7, Square::D8, Piece::WQ))
        )
    }

    #[test]
    fn test_enpassant_conversion() {
        assert_eq!(
            "e5d6",
            move_to_uci(&Move::Enpassant(Square::E5, Square::D6))
        )
    }
}
