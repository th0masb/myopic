use anyhow::Result;

use myopic_core::*;

use crate::moves::Move::*;
use crate::private::history::Discards;
use crate::private::Board;
use crate::Move;

#[cfg(test)]
mod test;

/// Implementation of board evolution/devolution via some given Move
/// instance which is assumed to be legal for this board.
impl Board {
    /// Public API for evolving a board. All that is required is a reference to
    /// a move which is assumed to be legal. The information required to reverse
    /// this same move is returned and the board is mutated to the next state.
    pub(crate) fn make_impl(&mut self, mv: Move) -> Result<()> {
        // Preserve the current state
        self.history.push(
            mv.clone(),
            Discards {
                rights: self.rights.clone(),
                enpassant: self.enpassant(),
                half_move_clock: self.half_move_clock(),
                hash: self.hash(),
            },
        );

        // Moves specific actions
        match mv {
            Standard { moving, from, dest, capture } => {
                if let Some(p) = capture {
                    self.pieces.unset_piece(p, dest);
                }
                self.pieces.unset_piece(moving, from);
                self.pieces.set_piece(moving, dest);
                self.rights.remove_rights(from | dest);
                self.enpassant = Board::compute_enpassant(from, dest, moving);
                self.clock =
                    if capture.is_some() || moving.1 == Class::P { 0 } else { self.clock + 1 };
            }
            Promotion { from, dest, promoted, capture } => {
                if let Some(p) = capture {
                    self.pieces.unset_piece(p, dest);
                }
                let moved = Piece(promoted.0, Class::P);
                self.pieces.unset_piece(moved, from);
                self.pieces.set_piece(promoted, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Enpassant { side, from, dest, capture } => {
                let moving_pawn = Piece(side, Class::P);
                self.pieces.unset_piece(moving_pawn.reflect(), capture);
                self.pieces.unset_piece(moving_pawn, from);
                self.pieces.set_piece(moving_pawn, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Castle { corner } => {
                self.rights.apply_castling(corner.0);
                let Line(r_source, r_target) = Line::rook_castling(corner);
                let Line(k_source, k_target) = Line::king_castling(corner);
                let rook = Piece(corner.0, Class::R);
                let king = Piece(corner.0, Class::K);
                self.pieces.unset_piece(rook, r_source);
                self.pieces.unset_piece(king, k_source);
                self.pieces.set_piece(rook, r_target);
                self.pieces.set_piece(king, k_target);
                self.enpassant = None;
                self.clock += 1;
            }
        };

        // General actions
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(())
    }

    /// Public API for devolving a move, the information lost at evolve time is
    /// required as an input here to recover the lost state exactly.
    pub(crate) fn unmake_impl(&mut self) -> Result<Move> {
        let (mv, state) = self.history.attempt_pop()?;

        match &mv {
            &Standard { moving, from, dest, capture } => {
                self.pieces.unset_piece(moving, dest);
                self.pieces.set_piece(moving, from);
                if let Some(p) = capture {
                    self.pieces.set_piece(p, dest);
                }
            }
            &Promotion { from, dest, promoted, capture } => {
                let moved = Piece(promoted.0, Class::P);
                self.pieces.unset_piece(promoted, dest);
                self.pieces.set_piece(moved, from);
                if let Some(p) = capture {
                    self.pieces.set_piece(p, dest);
                }
            }
            &Enpassant { side, from, dest, capture } => {
                let moving_pawn = Piece(side, Class::P);
                self.pieces.unset_piece(moving_pawn, dest);
                self.pieces.set_piece(moving_pawn.reflect(), capture);
                self.pieces.set_piece(moving_pawn, from);
            }
            &Castle { corner } => {
                let Line(r_source, r_target) = Line::rook_castling(corner);
                let Line(k_source, k_target) = Line::king_castling(corner);
                let rook = Piece(corner.0, Class::R);
                let king = Piece(corner.0, Class::K);
                self.pieces.set_piece(rook, r_source);
                self.pieces.set_piece(king, k_source);
                self.pieces.unset_piece(rook, r_target);
                self.pieces.unset_piece(king, k_target);
            }
        };

        self.rights = state.rights;
        self.clock = state.half_move_clock;
        self.enpassant = state.enpassant;
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(mv)
    }

    // Determines the enpassant square for the next board state given a
    // piece which has just moved from the source to the target.
    fn compute_enpassant(
        source: Square,
        target: Square,
        Piece(side, class): Piece,
    ) -> Option<Square> {
        if class == Class::P {
            if side.pawn_first_rank().contains(source) && side.pawn_third_rank().contains(target) {
                source.next(side.pawn_dir())
            } else {
                None
            }
        } else {
            None
        }
    }
}
