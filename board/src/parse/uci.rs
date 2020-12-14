use crate::parse::patterns::uci_move;
use crate::{Move, MoveComputeType, MutBoard};
use myopic_core::{Piece, Square};

/// Extracts the moves encoded in standard pgn format contained in
/// a single string.
pub fn uci(moves: &str) -> Result<Vec<Move>, String> {
    return partial_uci(&crate::start_position(), moves);
}

/// Extracts the moves encoded in standard pgn format starting at
/// a custom board position.
pub fn partial_uci<B: MutBoard>(start: &B, moves: &str) -> Result<Vec<Move>, String> {
    let mut mutator_board = start.clone();
    let mut dest: Vec<Move> = Vec::new();
    for evolve in uci_move().find_iter(moves) {
        match parse_single_move(&mut mutator_board, evolve.as_str()) {
            Ok(result) => dest.push(result.clone()),
            Err(_) => return Err(format!("Failed at {} in: {}", evolve.as_str(), moves)),
        };
        mutator_board.evolve(dest.last().unwrap());
    }
    Ok(dest)
}

fn parse_single_move<B: MutBoard>(start: &mut B, uci_move: &str) -> Result<Move, String> {
    let (src, dest, promoting) = extract_uci_component(uci_move)?;
    start
        .compute_moves(MoveComputeType::All)
        .into_iter()
        .find(|mv| match mv {
            &Move::Standard(_, s, d) => s == src && d == dest,
            &Move::Enpassant(s, d) => s == src && d == dest,
            &Move::Promotion(s, d, p) => {
                s == src && d == dest && promoting.map(|c| piece_char(p) == c).unwrap_or(false)
            }
            &Move::Castle(zone) => {
                let (_, king_src, king_dest) = zone.king_data();
                src == king_src && dest == king_dest
            }
        })
        .ok_or(format!("No moves matching {}", uci_move))
}

fn extract_uci_component(pgn_move: &str) -> Result<(Square, Square, Option<char>), String> {
    let src = Square::from_string(pgn_move.chars().take(2).collect::<String>().as_str())?;
    let dest = Square::from_string(pgn_move.chars().skip(2).take(2).collect::<String>().as_str())?;
    Ok((src, dest, pgn_move.chars().skip(4).next()))
}

fn piece_char(piece: Piece) -> char {
    match piece {
        Piece::WQ | Piece::BQ => 'q',
        Piece::WR | Piece::BR => 'r',
        Piece::WB | Piece::BB => 'b',
        Piece::WN | Piece::BN => 'n',
        _ => 'x',
    }
}
#[cfg(test)]
mod test {
    fn execute_success_test(expected_finish: &'static str, uci: &'static str) {
        let finish = crate::fen_position(expected_finish).unwrap();
        let mut board = crate::start_position();
        for evolve in super::partial_uci(&board, &String::from(uci)).unwrap() {
            board.evolve(&evolve);
        }
        assert_eq!(finish, board);
    }

    #[test]
    fn case_zero() {
        execute_success_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "")
    }

    #[test]
    fn case_one() {
        execute_success_test(
            "r2qkb1r/pp1n1pp1/2p1pnp1/8/3PN3/P3P1P1/1P3PBP/1nBQK2R w Kkq - 0 13",
            "1. d2d4 d7d5 2. c2c4 c7c6 3. g1f3 g8f6 4. e2e3 c8f5 5. b1c3 e7e6 6. f3h4 f5g6
             7. h4g6 h7g6 8. g2g3 b8d7 9. f1g2 d5c4 10. c3e4 c4c3 11. a2a3 c3c2 12. a1b1 c2b1n",
        );
    }

    //    #[test]
    //    fn case_two() {
    //        execute_success_test(
    //            "5rk1/pp2p3/3p2pb/2pP4/2q5/3b1B1P/PPn2Q2/R1NK2R1 w - - 0 28",
    //            "
    //            [Event \"F/S Return Match\"]
    //            [Site \"Belgrade, Serbia JUG\"]
    //            [Date \"1992.11.04\"]
    //            [Round \"29\"]
    //            [White \"Fischer, Robert J.\"]
    //            [Black \"Spassky, Boris V.\"]
    //            [Result \"1/2-1/2\"]
    //
    //            1.d4 Nf6 2.c4 g6 3.Nc3 Bg7 4.e4 d6 5.f3 O-O 6.Be3 Nbd7 7.Qd2
    //            c5 8.d5 Ne5 9.h3 Nh5 10.Bf2 f5 11.exf5 Rxf5 12.g4 Rxf3 13.gxh5
    //            Qf8 14.Ne4 Bh6 15.Qc2 Qf4 16.Ne2 Rxf2 17.Nxf2 Nf3+ 18.Kd1 Qh4
    //            19.Nd3 Bf5 20.Nec1 Nd2 21.hxg6 hxg6 22.Bg2 Nxc4 23.Qf2 Ne3+
    //            24.Ke2 Qc4 25.Bf3 Rf8 26.Rg1 Nc2 27.Kd1 Bxd3 0-1
    //            ",
    //        );
    //    }
}

#[cfg(test)]
mod test_single_move {
    use super::*;
    use myopic_core::{CastleZone, Square::*};

    fn execute_success_test(expected: Move, start_fen: &'static str, uci: &'static str) {
        let mut board = crate::fen_position(start_fen).unwrap();
        let uci_parse = parse_single_move(&mut board, &String::from(uci)).unwrap();
        assert_eq!(expected, uci_parse);
    }

    #[test]
    fn case_one() {
        execute_success_test(
            Move::Standard(Piece::BB, G4, F3),
            "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 4",
            "g4f3",
        )
    }

    #[test]
    fn case_two() {
        execute_success_test(
            Move::Enpassant(E5, Square::F6),
            "r2qkbnr/pp1np1pp/2p5/3pPp2/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQkq f6 0 7",
            "e5f6",
        )
    }

    #[test]
    fn case_three() {
        execute_success_test(
            Move::Promotion(F7, G8, Piece::WN),
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "f7g8n",
        )
    }

    #[test]
    fn case_four() {
        execute_success_test(
            Move::Promotion(F7, G8, Piece::WQ),
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "f7g8q",
        )
    }

    #[test]
    fn case_five() {
        execute_success_test(
            Move::Standard(Piece::BR, A8, E8),
            "r5r1/ppqkb1pp/2p1pn2/3p2B1/3P4/2NB1Q1P/PPP2PP1/4RRK1 b - - 8 14",
            "a8e8",
        )
    }

    #[test]
    fn case_six() {
        execute_success_test(
            Move::Standard(Piece::WR, E1, E2),
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "e1e2",
        )
    }

    #[test]
    fn case_seven() {
        execute_success_test(
            Move::Standard(Piece::BR, F3, F6),
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "f3f6",
        )
    }

    #[test]
    fn case_eight() {
        execute_success_test(
            Move::Standard(Piece::BN, E4, F2),
            "5r2/ppqkb1p1/2p1pr1p/3p4/3Pn2P/2NBR3/PPP1RPP1/7K b - - 1 21",
            "e4f2",
        )
    }

    #[test]
    fn case_nine() {
        execute_success_test(
            Move::Standard(Piece::BR, F8, F1),
            "5r2/ppqkb1p1/2p1p2p/3p4/P2P3P/2N1R3/1PP3P1/5B1K b - - 0 24",
            "f8f1",
        )
    }

    #[test]
    fn case_ten() {
        execute_success_test(
            Move::Castle(CastleZone::WK),
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R3K2R w KQkq - 1 13",
            "e1g1",
        )
    }
    #[test]
    fn case_eleven() {
        execute_success_test(
            Move::Castle(CastleZone::BQ),
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R4RK1 b kq - 2 13",
            "e8c8",
        )
    }
}
