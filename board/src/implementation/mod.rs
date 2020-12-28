use std::cmp::max;

use crate::implementation::cache::CalculationCache;
use crate::implementation::castling::Castling;
use crate::implementation::history::History;
use crate::implementation::positions::Positions;
use crate::mv::{parse_op, Move};
use crate::parse::patterns;
use crate::ChessBoard;
use crate::FenComponent;
use crate::MoveComputeType;
use crate::Termination;
use anyhow::{anyhow, Error, Result};
use myopic_core::*;
use std::str::FromStr;

mod cache;
mod castling;
mod evolve;
mod fen;
mod history;
mod moves;
mod positions;
#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Board {
    history: History,
    pieces: Positions,
    castling: Castling,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    cache: CalculationCache,
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        if patterns::fen().is_match(&fen) {
            let space_split: Vec<_> = patterns::space()
                .split(&fen)
                .map(|s| s.trim().to_owned())
                .collect();
            let active = space_split[1].parse::<Side>()?;
            let curr_move = space_split[5].parse::<usize>()?;
            Ok(Board {
                pieces: space_split[0].parse()?,
                active: space_split[1].parse()?,
                castling: space_split[2].parse()?,
                enpassant: parse_op(space_split[3].as_str())?,
                clock: space_split[4].parse::<usize>()?,
                history: History::new(2 * (max(curr_move, 1) - 1) + (active as usize)),
                cache: CalculationCache::default(),
            })
        } else {
            Err(anyhow!("Cannot parse FEN {}", fen))
        }
    }
}

fn hash(pt: &Positions, ct: &Castling, active: Side, ep: Option<Square>) -> u64 {
    pt.hash()
        ^ ct.hash()
        ^ crate::hash::side(active)
        ^ ep.map_or(0u64, |x| crate::hash::enpassant(x))
}

#[cfg(test)]
mod fen_test {
    use crate::implementation::test::TestBoard;
    use crate::Board;
    use anyhow::Result;
    use myopic_core::{constants::*, CastleZone, CastleZoneSet, Side, Square};

    fn test(expected: TestBoard, fen_string: String) -> Result<()> {
        assert_eq!(Board::from(expected), fen_string.parse::<Board>()?);
        Ok(())
    }

    #[test]
    fn fen_to_board_case_1() -> Result<()> {
        let fen = "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19";
        let board = TestBoard {
            whites: vec![
                A3 | B2 | C4 | E4 | F2 | G2 | H2,
                C3 | D4,
                E2,
                D1 | F1,
                D2,
                G1,
            ],
            blacks: vec![
                A6 | B7 | D6 | E6 | F7 | G6 | H6,
                E7,
                C8 | G7,
                A8 | D8,
                C7,
                G8,
            ],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 3,
            active: Side::White,
            enpassant: None,
            history_count: 36,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_2() -> Result<()> {
        let fen = "rnb2rk1/ppp2ppp/4pq2/8/2PP4/5N2/PP3PPP/R2QKB1R w KQ - 2 9";
        let board = TestBoard {
            whites: vec![A2 | B2 | C4 | D4 | F2 | G2 | H2, F3, F1, A1 | H1, D1, E1],
            blacks: vec![A7 | B7 | C7 | E6 | F7 | G7 | H7, B8, C8, A8 | F8, F6, G8],
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::White,
            enpassant: None,
            history_count: 16,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_3() -> Result<()> {
        let fen = "r1bqkbnr/ppp1pppp/n7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let board = TestBoard {
            whites: vec![
                A2 | B2 | C2 | D2 | E5 | F2 | G2 | H2,
                B1 | G1,
                C1 | F1,
                A1 | H1,
                D1,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C7 | D5 | E7 | F7 | G7 | H7,
                A6 | G8,
                C8 | F8,
                A8 | H8,
                D8,
                E8,
            ],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            clock: 0,
            active: Side::White,
            enpassant: Some(Square::D6),
            history_count: 4,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_4() -> Result<()> {
        let fen = "r6k/p5pp/p1b2qnN/8/3Q4/2P1B3/PP4PP/R5K1 b - - 2 21";
        let board = TestBoard {
            whites: vec![A2 | B2 | C3 | G2 | H2, H6, E3, A1, D4, G1],
            blacks: vec![A7 | A6 | G7 | H7, G6, C6, A8, F6, H8],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::Black,
            enpassant: None,
            history_count: 41,
        };
        test(board, String::from(fen))
    }
}

// Trait implementations
impl Reflectable for Board {
    fn reflect(&self) -> Self {
        let pieces = self.pieces.reflect();
        let castling = self.castling.reflect();
        let active = self.active.reflect();
        let enpassant = self.enpassant.reflect();
        let hash = hash(&pieces, &castling, active, enpassant);
        Board {
            history: self.history.reflect_for(hash),
            clock: self.clock,
            pieces,
            castling,
            active,
            enpassant,
            cache: CalculationCache::default(),
        }
    }
}

impl PartialEq<Board> for Board {
    fn eq(&self, other: &Board) -> bool {
        self.pieces == other.pieces
            && self.castling.rights() == other.castling.rights()
            && self.enpassant == other.enpassant
            && self.active == other.active
            && self.half_move_clock() == other.half_move_clock()
    }
}

impl ChessBoard for Board {
    fn make(&mut self, mv: Move) -> Result<()> {
        self.evolve_impl(mv)
    }

    fn unmake(&mut self) -> Result<Move> {
        self.devolve_impl()
    }

    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn termination_status(&mut self) -> Option<Termination> {
        self.termination_status_impl()
    }

    fn in_check(&mut self) -> bool {
        self.passive_control_impl().contains(self.king(self.active))
    }

    fn side(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.pieces.whites(),
            Side::Black => self.pieces.blacks(),
        }
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        (
            self.pieces.side_locations(Side::White),
            self.pieces.side_locations(Side::Black),
        )
    }

    fn hash(&self) -> u64 {
        hash(&self.pieces, &self.castling, self.active, self.enpassant)
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn locs(&self, pieces: &[Piece]) -> BitBoard {
        pieces
            .into_iter()
            .map(|&p| self.pieces.locs(p))
            .fold(BitBoard::EMPTY, |l, r| l | r)
    }

    fn king(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn position_count(&self) -> usize {
        self.history.position_count()
    }

    fn remaining_rights(&self) -> CastleZoneSet {
        self.castling.rights()
    }

    fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in crate::parse::pgn::moves(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in crate::parse::uci::moves(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    fn to_partial_fen(&self, cmps: &[FenComponent]) -> String {
        fen::to_fen_impl(self, cmps)
    }
}
