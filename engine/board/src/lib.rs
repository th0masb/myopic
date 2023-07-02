use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Debug;
use std::str::FromStr;

use enum_map::{Enum, EnumMap};
use enumset::EnumSet;

use cache::CalculationCache;
use history::History;
use Move::Promotion;
use myopic_core::anyhow::Result;
pub use myopic_core::*;
use parse::patterns;
pub use parse::uci::UciMove;
use positions::Positions;
use rights::Rights;

use crate::anyhow::{anyhow, Error};
use crate::cache::RaySet;

mod cache;
mod evolve;
mod fen;
mod history;
mod moves;
mod parse;
mod positions;
mod rights;
#[cfg(test)]
mod test;

/// The start position of a chess game encoded in FEN format
pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Return the start position of a standard game
pub fn start() -> Board {
    START_FEN.parse().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    Standard { moving: Piece, from: Square, dest: Square, capture: Option<Piece> },
    Enpassant { side: Side, from: Square, dest: Square, capture: Square },
    Promotion { from: Square, dest: Square, promoted: Piece, capture: Option<Piece> },
    Castle { corner: Corner },
}

impl Move {
    pub fn is_attack(&self) -> bool {
        match self {
            Move::Standard { capture, .. } => capture.is_some(),
            Promotion { capture, .. } => capture.is_some(),
            Move::Enpassant { .. } => true,
            Move::Castle { .. } => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Moves<'a> {
    All,
    Are(MoveFacet),
    AreAny(&'a [MoveFacet]),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Enum)]
pub enum MoveFacet {
    Checking,
    Attacking,
    Promoting,
}

/// Represents the possible ways a game can be terminated, we only
/// consider a game to be terminated when a side has no legal moves
/// to make or if a special draw condition is met like position
/// repetition. If a side has no legal moves and is currently in check
/// then the game is lost, if it is not in check then the game is
/// drawn.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TerminalState {
    Draw,
    Loss,
}

/// Represents the individual components which make up a board position
/// encoded as a FEN string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum FenPart {
    Board,
    Active,
    CastlingRights,
    Enpassant,
    HalfMoveCount,
    MoveCount,
}

/// Chessboard representation on which a game can be played
#[derive(Debug, Clone)]
pub struct Board {
    history: History,
    pieces: Positions,
    rights: Rights,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    cache: RefCell<CalculationCache>,
}

impl Board {
    /// Evolves the position by making the given move. If the source hash
    /// of the move does not match the hash of this position (prior to making
    /// the move) then an error will be returned. If the hash matches but
    /// the move is illegal in this position (e.g if you manually start
    /// creating moves) then the results are undefined.
    pub fn make(&mut self, mv: Move) -> Result<()> {
        self.make_impl(mv)
    }

    /// Reverses and returns the move which was made last. If no move has
    /// been made yet then an error is returned.
    pub fn unmake(&mut self) -> Result<Move> {
        self.unmake_impl()
    }

    /// Parse the given string as a sequence of pgn encoded moves
    /// starting from the current position. The moves are then
    /// made one by one. The sequence of moves which were made
    /// are returned in a Vec.
    pub fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in parse::pgn::moves(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    /// Parse the given string as a sequence of uci encoded moves
    /// starting from the current position. The moves are then
    /// made one by one.The sequence of moves which were made
    /// are returned in a Vec.
    pub fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut dest = vec![];
        for mv in parse::uci::move_sequence(self, moves)? {
            dest.push(mv.clone());
            self.make(mv)?;
        }
        Ok(dest)
    }

    /// Compute a vector of all the legal moves in this position for the
    /// given computation type. Note there is no particular ordering to the
    /// move vector. If we are in check then the type is ignored and all
    /// legal moves are returned.
    pub fn moves(&self, moves: Moves) -> Vec<Move> {
        let all = self.compute_moves_impl();
        match moves {
            Moves::All => all,
            Moves::Are(facet) => self.moves_impl(all, &[facet]).collect(),
            Moves::AreAny(facets) => self.moves_impl(all, facets).collect(),
        }
    }

    fn moves_impl<'a>(
        &'a self,
        all: Vec<Move>,
        facets: &'a [MoveFacet]
    ) -> impl Iterator<Item=Move> + 'a {
        let discoveries = self.compute_discoveries();
        let occupied = self.side(Side::W) | self.side(Side::B);
        let king_loc = self.king(self.active.reflect());
        all.into_iter().filter(move |m| {
            facets.iter().any(|&f| {
                match f {
                    MoveFacet::Attacking => m.is_attack(),
                    MoveFacet::Promoting => if let Promotion { .. } = m { true } else { false },
                    MoveFacet::Checking => {
                        match m {
                            Move::Castle { .. } => false,
                            Move::Enpassant { side, from, dest, capture } => {
                                discoveries.constraints[*from].contains(*dest) ||
                                    Piece(*side, Class::P)
                                        .control(*dest, occupied - *from - *capture)
                                        .contains(king_loc)
                            },
                            Promotion { from, dest, promoted, .. } => {
                                discoveries.constraints[*from].contains(*dest) ||
                                    promoted
                                        .control(*dest, occupied - *from)
                                        .contains(king_loc)
                            },
                            Move::Standard { moving, from, dest, .. } => {
                                discoveries.constraints[*from].contains(*dest) ||
                                    moving
                                        .control(*dest, occupied - *from)
                                        .contains(king_loc)
                            },
                        }
                    },
                }
            })
        })
    }

    /// Compute the termination state of this node. If it is not terminal
    /// nothing is returned, if it is then the manner of termination is
    /// returned wrapped inside an Option. The termination can be only a
    /// draw or a loss since a side only loses when it runs out of moves,
    /// i.e. you don't play a winning move, you just fail to have a legal
    /// move.
    pub fn terminal_state(&self) -> Option<TerminalState> {
        self.terminal_state_impl()
    }

    /// Determines whether the active side is in a state of check.
    pub fn in_check(&self) -> bool {
        self.passive_control().contains(self.king(self.active))
    }

    /// Return the locations of all pieces on the given side.
    pub fn side(&self, side: Side) -> BitBoard {
        match side {
            Side::W => self.pieces.whites(),
            Side::B => self.pieces.blacks(),
        }
    }

    /// Return the locations of all white and black pieces.
    pub fn sides(&self) -> (BitBoard, BitBoard) {
        (self.pieces.side_locations(Side::W), self.pieces.side_locations(Side::B))
    }

    /// Returns the Zobrist hash of this position.
    pub fn hash(&self) -> u64 {
        hash(&self.pieces, &self.rights, self.active, self.enpassant)
    }

    /// Return the active side in this position, i.e. the one whose turn it is.
    pub fn active(&self) -> Side {
        self.active
    }

    /// Return the enpassant target square in this position.
    pub fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    /// Return the locations of the given pieces.
    pub fn locs(&self, pieces: &[Piece]) -> BitBoard {
        pieces.into_iter().map(|&p| self.pieces.locs(p)).fold(BitBoard::EMPTY, |l, r| l | r)
    }

    /// Return the location of the king for the given side.
    pub fn king(&self, side: Side) -> Square {
        self.pieces.locs(Piece(side, Class::K)).into_iter().next().unwrap()
    }

    /// Return the piece occupying the given location.
    pub fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    /// Return the half move clock value at this position.
    pub fn half_move_clock(&self) -> usize {
        self.clock
    }

    /// Return the number of previous positions for this board.
    pub fn position_count(&self) -> usize {
        self.history.position_count()
    }

    /// Return the remaining castling rights from this position.
    pub fn remaining_rights(&self) -> EnumMap<Side, EnumSet<Flank>> {
        self.rights.0.clone()
    }

    /// Given a uci encoded move this method will attempt to match
    /// it to the unique matching legal move in this position if it
    /// exist. An error is returned if no matching move exists in
    /// this position.
    pub fn parse_uci(&self, uci_move: &str) -> Result<Move> {
        parse::uci::single_move(self, uci_move)
    }

    /// Return the specified components of the FEN encoding of this position
    /// in the given order with components separated by a space.
    pub fn to_fen_parts(&self, parts: &[FenPart]) -> String {
        fen::to_fen_impl(self, parts)
    }

    /// Return the complete FEN representation of this position.
    pub fn to_fen(&self) -> String {
        self.to_fen_parts(&[
            FenPart::Board,
            FenPart::Active,
            FenPart::CastlingRights,
            FenPart::Enpassant,
            FenPart::HalfMoveCount,
            FenPart::MoveCount,
        ])
    }

    /// Returns the locations of all pieces on the board.
    pub fn all_pieces(&self) -> BitBoard {
        let (w, b) = self.sides();
        w | b
    }
}

impl Default for Board {
    fn default() -> Self {
        START_FEN.parse().unwrap()
    }
}

impl PartialEq<Board> for Board {
    fn eq(&self, other: &Board) -> bool {
        self.pieces == other.pieces
            && self.rights == other.rights
            && self.enpassant == other.enpassant
            && self.active == other.active
            && self.half_move_clock() == other.half_move_clock()
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        if patterns::fen().is_match(&fen) {
            let space_split: Vec<_> =
                patterns::space().split(&fen).map(|s| s.trim().to_owned()).collect();
            let active = space_split[1].parse::<Side>()?;
            let curr_move = space_split[5].parse::<usize>()?;
            Ok(Board {
                pieces: space_split[0].parse()?,
                active: space_split[1].parse()?,
                rights: space_split[2].parse()?,
                enpassant: parse::parse_option(space_split[3].as_str())?,
                clock: space_split[4].parse::<usize>()?,
                history: History::new(2 * (max(curr_move, 1) - 1) + (active as usize)),
                cache: RefCell::new(CalculationCache::default()),
            })
        } else {
            Err(anyhow!("Cannot parse FEN {}", fen))
        }
    }
}

pub(crate) fn hash(pos: &Positions, rights: &Rights, active: Side, ep: Option<Square>) -> u64 {
    let mut result = pos.hash() ^ hash::side(active) ^ ep.map_or(0u64, |x| hash::enpassant(x));
    rights.corners().for_each(|c| result ^= hash::zone(c));
    result
}

#[cfg(test)]
mod fen_test {
    use crate::test::TestBoard;
    use crate::Board;
    use crate::{Side, Square::*};

    use super::*;

    fn test(expected: TestBoard, fen_string: String) -> Result<()> {
        assert_eq!(Board::from(expected), fen_string.parse::<Board>()?);
        Ok(())
    }

    #[test]
    fn fen_to_board_case_1() -> Result<()> {
        let fen = "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19";
        let board = TestBoard {
            whites: vec![A3 | B2 | C4 | E4 | F2 | G2 | H2, C3 | D4, !!E2, D1 | F1, !!D2, !!G1],
            blacks: vec![A6 | B7 | D6 | E6 | F7 | G6 | H6, !!E7, C8 | G7, A8 | D8, !!C7, !!G8],
            castle_rights: Rights::empty(),
            clock: 3,
            active: Side::W,
            enpassant: None,
            history_count: 36,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_2() -> Result<()> {
        let fen = "rnb2rk1/ppp2ppp/4pq2/8/2PP4/5N2/PP3PPP/R2QKB1R w KQ - 2 9";
        let board = TestBoard {
            whites: vec![A2 | B2 | C4 | D4 | F2 | G2 | H2, !!F3, !!F1, A1 | H1, !!D1, !!E1],
            blacks: vec![A7 | B7 | C7 | E6 | F7 | G7 | H7, !!B8, !!C8, A8 | F8, !!F6, !!G8],
            castle_rights: Rights::side(Side::W),
            clock: 2,
            active: Side::W,
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
                !!D1,
                !!E1,
            ],
            blacks: vec![
                A7 | B7 | C7 | D5 | E7 | F7 | G7 | H7,
                A6 | G8,
                C8 | F8,
                A8 | H8,
                !!D8,
                !!E8,
            ],
            castle_rights: Rights::all(),
            clock: 0,
            active: Side::W,
            enpassant: Some(D6),
            history_count: 4,
        };
        test(board, String::from(fen))
    }

    #[test]
    fn fen_to_board_case_4() -> Result<()> {
        let fen = "r6k/p5pp/p1b2qnN/8/3Q4/2P1B3/PP4PP/R5K1 b - - 2 21";
        let board = TestBoard {
            whites: vec![A2 | B2 | C3 | G2 | H2, !!H6, !!E3, !!A1, !!D4, !!G1],
            blacks: vec![A7 | A6 | G7 | H7, !!G6, !!C6, !!A8, !!F6, !!H8],
            castle_rights: Rights::empty(),
            clock: 2,
            active: Side::B,
            enpassant: None,
            history_count: 41,
        };
        test(board, String::from(fen))
    }
}

impl Reflectable for Board {
    fn reflect(&self) -> Self {
        let start_hash = Board::default().hash();
        if self.history.historical_positions().next() == Some(start_hash) {
            // If we played from the start position we can reflect properly
            let mut reflected = Board::default();
            reflected.active = Side::B;
            for m in self.history.historical_moves() {
                reflected.make(m).unwrap()
            }
            reflected
        } else {
            // Otherwise we started from some intermediate position and we cannot keep our history
            let pieces = self.pieces.reflect();
            let rights = self.rights.reflect();
            let active = self.active.reflect();
            let enpassant = self.enpassant.reflect();
            //let hash = hash(&pieces, &rights, active, enpassant);
            Board {
                history: History::default(),
                clock: self.clock,
                pieces,
                rights,
                active,
                enpassant,
                cache: RefCell::new(CalculationCache::default()),
            }
        }
    }
}

#[cfg(test)]
mod uci_conversion_test {
    use myopic_core::*;

    use super::Move;

    #[test]
    fn test_pawn_standard_conversion() {
        assert_eq!(
            "e2e4",
            Move::Standard {
                moving: Piece(Side::W, Class::P),
                from: Square::E2,
                dest: Square::E4,
                capture: None,
            }
            .uci_format()
        );
    }

    #[test]
    fn test_rook_standard_conversion() {
        assert_eq!(
            "h1h7",
            Move::Standard {
                moving: Piece(Side::B, Class::R),
                from: Square::H1,
                dest: Square::H7,
                capture: Some(Piece(Side::W, Class::Q)),
            }
            .uci_format()
        );
    }

    #[test]
    fn test_castling_conversion() {
        assert_eq!("e1g1", Move::Castle { corner: Corner(Side::W, Flank::K) }.uci_format());
        assert_eq!("e1c1", Move::Castle { corner: Corner(Side::W, Flank::Q) }.uci_format());
        assert_eq!("e8g8", Move::Castle { corner: Corner(Side::B, Flank::K) }.uci_format());
        assert_eq!("e8c8", Move::Castle { corner: Corner(Side::B, Flank::Q) }.uci_format());
    }

    #[test]
    fn test_promotion_conversion() {
        assert_eq!(
            "e7d8q",
            Move::Promotion {
                from: Square::E7,
                dest: Square::D8,
                promoted: Piece(Side::W, Class::Q),
                capture: Some(Piece(Side::B, Class::B)),
            }
            .uci_format()
        )
    }

    #[test]
    fn test_enpassant_conversion() {
        assert_eq!(
            "e5d6",
            Move::Enpassant {
                side: Side::W,
                from: Square::E5,
                dest: Square::D6,
                capture: Square::D5,
            }
            .uci_format()
        )
    }
}
