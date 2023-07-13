
use crate::{Board, piece_class, Corner, CornerMap, square_file, lift, Piece, piece_side, PieceMap, square_rank, Side, SideMap, Square, SquareMap, Symmetric};
use crate::moves::{Move, Moves, Move::*};

use anyhow::{anyhow, Result};
use crate::board::iter;
use crate::constants::{class, side, piece};



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

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub piece_boards: PieceMap<Board>,
    pub piece_locs: SquareMap<Option<Piece>>,
    pub side_boards: SideMap<Board>,
    pub castling_rights: CornerMap<bool>,
    pub active: Side,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
    pub history: Vec<(Discards, Move)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Discards {
    pub castling_rights: CornerMap<bool>,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
}

impl Default for Position {
    fn default() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse().unwrap()
    }
}

// Unwind the position to earliest point in history, reflect the board, then play back
// through the reflected moves
impl Symmetric for Position {
    fn reflect(&self) -> Self {
        todo!()
    }
}

// Implementation block for making/unmaking moves
impl Position {
    pub fn make(&mut self, m: Move) -> Result<()> {
        self.history.push((self.create_discards(), m.clone()));
        self.enpassant.map(|sq| self.key ^= crate::hash::enpassant(sq));
        match m {
            Null => self.enpassant = None,
            Standard { moving, from, dest, capture } => {
                capture.map(|p| self.unset_piece(p, dest));
                self.unset_piece(moving, from);
                self.set_piece(moving, dest);
                self.remove_rights(rights_removed(from));
                self.remove_rights(rights_removed(dest));
                let is_pawn = piece_class(moving) == class::P;
                self.clock = if capture.is_some() || is_pawn { 0 } else { self.clock + 1};
                self.enpassant = if is_pawn {
                    let is_white = piece_side(moving) == side::W;
                    let start_r = if is_white { 1 } else { 6 };
                    let third_r = if is_white { 3 } else { 4 };
                    let shifter = if is_white { from } else  { dest };
                    if square_rank(from) == start_r && square_rank(dest) == third_r {
                        let next_ep = 8 * (square_rank(shifter) + 1) + square_file(shifter);
                        self.key ^= crate::hash::enpassant(next_ep);
                        Some(next_ep)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Promotion { from, dest, promoted, capture } => {
                capture.map(|p| self.unset_piece(p, dest));
                let moved = if piece_side(promoted) == side::W { piece::WP } else { piece::BP };
                self.remove_rights(rights_removed(dest));
                self.unset_piece(moved, from);
                self.set_piece(promoted, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Enpassant { side, from, dest, capture } => {
                let moving = if side == side::W { piece::WP } else { piece::BP };
                let taken = if side == side::W { piece::BP } else { piece::WP };
                self.unset_piece(taken, capture);
                self.unset_piece(moving, from);
                self.set_piece(moving, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Castle { corner } => {
                let (r_source, r_target) = rook_line(corner);
                let (k_source, k_target) = king_line(corner);
                self.remove_rights(rights_removed(k_source));
                let side = corner / 2;
                let rook = if side == side::W { piece::WR } else { piece::BR };
                let king = if side == side::W { piece::WK } else { piece::BK };
                self.unset_piece(rook, r_source);
                self.unset_piece(king, k_source);
                self.set_piece(rook, r_target);
                self.set_piece(king, k_target);
                self.enpassant = None;
                self.clock += 1;
            }
        };
        self.key ^= crate::hash::black_move();
        self.active = if self.active == side::W { side::B } else { side::W };
        Ok(())
    }

    pub fn unmake(&mut self) -> Result<Move> {
        if self.history.len() == 0 {
            return Err(anyhow!("No moves left to unmake!"));
        }
        let (state, mv) = self.history.remove(self.history.len() - 1);
        match &mv {
            Null => {}
            &Standard { moving, from, dest, capture } => {
                self.unset_piece(moving, dest);
                self.set_piece(moving, from);
                capture.map(|p| self.set_piece(p, dest));
            }
            &Promotion { from, dest, promoted, capture } => {
                let moved = if piece_side(promoted) == side::W { piece::WP } else { piece::BP };
                self.unset_piece(promoted, dest);
                self.set_piece(moved, from);
                capture.map(|p| self.set_piece(p, dest));
            }
            &Enpassant { side, from, dest, capture } => {
                let moving = if side == side::W { piece::WP } else { piece::BP };
                let taken = if side == side::W { piece::BP } else { piece::WP };
                self.unset_piece(moving, dest);
                self.set_piece(taken, capture);
                self.set_piece(moving, from);
            }
            &Castle { corner } => {
                let (r_source, r_target) = rook_line(corner);
                let (k_source, k_target) = king_line(corner);
                let side = corner / 2;
                let rook = if side == side::W { piece::WR } else { piece::BR };
                let king = if side == side::W { piece::WK } else { piece::BK };
                self.set_piece(rook, r_source);
                self.set_piece(king, k_source);
                self.unset_piece(rook, r_target);
                self.unset_piece(king, k_target);
            }
        };
        self.castling_rights = state.castling_rights;
        self.clock = state.clock;
        self.enpassant = state.enpassant;
        self.key = state.key;
        self.active = if self.active == side::W { side::B } else { side::W };
        Ok(mv)
    }

    fn set_piece(&mut self, piece: Piece, square: Square) {
        self.key ^= crate::hash::piece(piece, square);
        let lifted = lift(square);
        let side = piece_side(piece);
        self.piece_boards[piece] |= lifted;
        self.side_boards[side] |= lifted;
        self.piece_locs[square] = Some(piece);
    }

    fn unset_piece(&mut self, piece: Piece, square: Square) {
        self.key ^= crate::hash::piece(piece, square);
        let lifted = !lift(square);
        self.piece_boards[piece] &= lifted;
        self.side_boards[piece_side(piece)] &= lifted;
        self.piece_locs[square] = None;
    }

    fn remove_rights(&mut self, corners: &[Corner]) {
        corners.iter().for_each(|&c| {
            if self.castling_rights[c] {
                self.castling_rights[c] = false;
                self.key ^= crate::hash::corner(c);
            }
        })
    }

    pub fn create_discards(&self) -> Discards {
        Discards {
            castling_rights: self.castling_rights.clone(),
            enpassant: self.enpassant,
            clock: self.clock,
            key: self.key,
        }
    }
}

// Implementation block for misc property generation
impl Position {
    pub fn compute_control(&self, side: Side) -> Board {
        use crate::constants::{piece::*, side::*};
        use crate::board::control;
        let invisible_king = self.piece_boards[if side == W { BK } else { WK }];
        let occupied = (self.side_boards[W] | self.side_boards[B]) & !invisible_king;
        [class::N, class::B, class::R, class::Q, class::K]
            .into_iter()
            .map(|class| side * 6 + class)
            .flat_map(|p| iter(self.piece_boards[p]).map(move |sq| control(p, sq, occupied)))
            .fold(0u64, |a, n| a | n)
            | pawn_control(side, self.piece_boards[if side == W { WP } else { BP }])
    }
}

fn pawn_control(side: Side, pawns: Board) -> Board {
    use crate::constants::boards::FILES;
    let (not_a_file, not_h_file) = (!FILES[7], !FILES[0]);
    if side == side::W {
        ((pawns & not_a_file) << 9) | ((pawns & not_h_file) << 7)
    } else {
        ((pawns & not_h_file) >> 9) | ((pawns & not_a_file) >> 7)
    }
}

// Implementation block for move generation
impl Position {
    pub fn moves(&self, _moves: Moves) -> Vec<Move> {
        todo!()
    }
}

fn rights_removed<'a>(square: Square) -> &'a [Corner] {
    use crate::constants::{square::*, corner::*};
    match square {
        H1 => &[WK],
        E1 => &[WK, WQ],
        A1 => &[WQ],
        H8 => &[BK],
        E8 => &[BK, BQ],
        A8 => &[BQ],
        _ => &[],
    }
}

fn king_line(corner: Corner) -> (Square, Square) {
    use crate::constants::{square, corner};
    match corner {
        corner::WK => (square::E1, square::G1),
        corner::WQ => (square::E1, square::C1),
        corner::BK => (square::E8, square::G8),
        corner::BQ => (square::E8, square::C8),
        _ => panic!("{} is not a valid corner", corner)
    }
}

fn rook_line(corner: Corner) -> (Square, Square) {
    use crate::constants::{square, corner};
    match corner {
        corner::WK => (square::H1, square::F1),
        corner::WQ => (square::A1, square::D1),
        corner::BK => (square::H8, square::F8),
        corner::BQ => (square::A8, square::D8),
        _ => panic!("{} is not a valid corner", corner)
    }
}

