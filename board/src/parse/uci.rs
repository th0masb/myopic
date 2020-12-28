use crate::parse::patterns::uci_move;
use crate::{ChessBoard, Move, MoveComputeType};
use anyhow::{anyhow, Result};
use myopic_core::{Piece, Square};

/// Extracts the moves encoded in standard pgn format starting at
/// a custom board position.
pub fn moves<B: ChessBoard>(start: &B, encoded: &str) -> Result<Vec<Move>> {
    let (mut mutator_board, mut dest) = (start.clone(), vec![]);
    for evolve in uci_move().find_iter(encoded) {
        match parse_single_move(&mut mutator_board, evolve.as_str()) {
            Ok(result) => {
                dest.push(result.clone());
                mutator_board.make(result)?
            }
            Err(_) => return Err(anyhow!("Failed at {} in: {}", evolve.as_str(), encoded)),
        };
    }
    Ok(dest)
}

fn parse_single_move<B: ChessBoard>(start: &mut B, uci_move: &str) -> Result<Move> {
    let (f, d, promoting) = extract_uci_component(uci_move)?;
    start
        .compute_moves(MoveComputeType::All)
        .into_iter()
        .find(|mv| match mv {
            &Move::Standard { from, dest, .. } => from == f && dest == d,
            &Move::Enpassant { from, dest, .. } => from == f && dest == d,
            &Move::Castle { zone, .. } => {
                let (_, king_src, king_dest) = zone.king_data();
                f == king_src && d == king_dest
            }
            &Move::Promotion {
                from,
                dest,
                promoted,
                ..
            } => {
                from == f
                    && dest == d
                    && promoting
                        .map(|c| piece_char(promoted) == c)
                        .unwrap_or(false)
            }
        })
        .ok_or(anyhow!("No moves matching {}", uci_move))
}

fn extract_uci_component(pgn_move: &str) -> Result<(Square, Square, Option<char>)> {
    let from = pgn_move
        .chars()
        .take(2)
        .collect::<String>()
        .as_str()
        .parse::<Square>()?;
    let dest = pgn_move
        .chars()
        .skip(2)
        .take(2)
        .collect::<String>()
        .parse::<Square>()?;

    Ok((from, dest, pgn_move.chars().skip(4).next()))
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
    use crate::{Board, ChessBoard};
    use anyhow::Result;

    fn execute_success_test(expected_finish: &'static str, uci: &'static str) -> Result<()> {
        let finish = expected_finish.parse::<Board>()?;
        let mut board = crate::STARTPOS_FEN.parse::<Board>()?;
        for evolve in super::moves(&board, uci)? {
            board.make(evolve)?;
        }
        assert_eq!(finish, board);
        Ok(())
    }

    #[test]
    fn case_zero() -> Result<()> {
        execute_success_test(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "",
        )
    }

    #[test]
    fn case_one() -> Result<()> {
        execute_success_test(
            "r2qkb1r/pp1n1pp1/2p1pnp1/8/3PN3/P3P1P1/1P3PBP/1nBQK2R w Kkq - 0 13",
            "1. d2d4 d7d5 2. c2c4 c7c6 3. g1f3 g8f6 4. e2e3 c8f5 5. b1c3 e7e6 6. f3h4 f5g6
             7. h4g6 h7g6 8. g2g3 b8d7 9. f1g2 d5c4 10. c3e4 c4c3 11. a2a3 c3c2 12. a1b1 c2b1n",
        )
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

    fn execute_success_test(
        expected: &'static str,
        start_fen: &'static str,
        uci: &'static str,
    ) -> Result<()> {
        let mut board = start_fen.parse::<Board>()?;
        let parsed_expected = Move::from(expected, board.hash())?;
        let uci_parse = parse_single_move(&mut board, uci)?;
        assert_eq!(parsed_expected, uci_parse);
        Ok(())
    }

    #[test]
    fn case_one() -> Result<()> {
        execute_success_test(
            "sbbg4f3wn",
            "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 4",
            "g4f3",
        )
    }

    #[test]
    fn case_two() -> Result<()> {
        execute_success_test(
            "ewe5f6f5",
            "r2qkbnr/pp1np1pp/2p5/3pPp2/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQkq f6 0 7",
            "e5f6",
        )
    }

    #[test]
    fn case_three() -> Result<()> {
        execute_success_test(
            "pf7g8wnbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "f7g8n",
        )
    }

    #[test]
    fn case_four() -> Result<()> {
        execute_success_test(
            "pf7g8wqbn",
            "r2q1bnr/pp1nkPpp/2p1p3/3p4/8/2N2Q1P/PPPP1PP1/R1B1KB1R w KQ - 1 9",
            "f7g8q",
        )
    }

    #[test]
    fn case_five() -> Result<()> {
        execute_success_test(
            "sbra8e8-",
            "r5r1/ppqkb1pp/2p1pn2/3p2B1/3P4/2NB1Q1P/PPP2PP1/4RRK1 b - - 8 14",
            "a8e8",
        )
    }

    #[test]
    fn case_six() -> Result<()> {
        execute_success_test(
            "swre1e2-",
            "4rr2/ppqkb1p1/2p1p2p/3p4/3Pn2B/2NBRQ1P/PPP2PP1/4R1K1 w - - 2 18",
            "e1e2",
        )
    }

    #[test]
    fn case_seven() -> Result<()> {
        execute_success_test(
            "sbrf3f6wb",
            "5r2/ppqkb1p1/2p1pB1p/3p4/3Pn2P/2NBRr2/PPP1RPP1/6K1 b - - 0 20",
            "f3f6",
        )
    }

    #[test]
    fn case_eight() -> Result<()> {
        execute_success_test(
            "sbne4f2wp",
            "5r2/ppqkb1p1/2p1pr1p/3p4/3Pn2P/2NBR3/PPP1RPP1/7K b - - 1 21",
            "e4f2",
        )
    }

    #[test]
    fn case_nine() -> Result<()> {
        execute_success_test(
            "sbrf8f1wb",
            "5r2/ppqkb1p1/2p1p2p/3p4/P2P3P/2N1R3/1PP3P1/5B1K b - - 0 24",
            "f8f1",
        )
    }

    #[test]
    fn case_ten() -> Result<()> {
        execute_success_test(
            "cwk",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R3K2R w KQkq - 1 13",
            "e1g1",
        )
    }
    #[test]
    fn case_eleven() -> Result<()> {
        execute_success_test(
            "cbq",
            "r3k2r/pp1q1ppp/n1p2n2/4p3/3pP2P/3P1QP1/PPPN1PB1/R4RK1 b kq - 2 13",
            "e8c8",
        )
    }
}
