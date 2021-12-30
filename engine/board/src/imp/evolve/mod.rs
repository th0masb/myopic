use anyhow::{anyhow, Result};

use myopic_core::*;

use crate::{ChessBoard, Move};
use crate::imp::Board;
use crate::imp::history::Discards;
use crate::mv::Move::*;

#[cfg(test)]
mod test;

/// Implementation of board evolution/devolution via some given Move
/// instance which is assumed to be legal for this board.
impl Board {
    /// Public API for evolving a board. All that is required is a reference to
    /// a move which is assumed to be legal. The information required to reverse
    /// this same move is returned and the board is mutated to the next state.
    pub(crate) fn make(&mut self, mv: Move) -> Result<()> {
        // Check this move came from this position
        if self.hash() != mv.source() {
            return Err(anyhow!("Mismatched source hash for {}", mv));
        }

        // Preserve the current state
        self.history.push(
            mv.clone(),
            Discards {
                rights: self.rights,
                enpassant: self.enpassant(),
                half_move_clock: self.half_move_clock(),
            },
        );

        // Moves specific actions
        match mv {
            Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => self.evolve_s(moving, from, dest, capture),
            Castle { zone, .. } => self.evolve_c(zone),
            Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => self.evolve_e(side, from, dest, capture),
            Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => self.evolve_p(from, dest, promoted, capture),
        };

        // General actions
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(())
    }

    fn evolve_s(&mut self, moving: Piece, source: Square, target: Square, captured: Option<Piece>) {
        self.pieces.toggle_piece(moving, &[source, target]);
        match captured {
            None => {}
            Some(p) => self.pieces.toggle_piece(p, &[target]),
        };
        self.rights = self.rights.remove_rights(source | target);
        self.enpassant = Board::compute_enpassant(source, target, moving);
        self.clock = if captured.is_some() || moving.is_pawn() {
            0
        } else {
            self.clock + 1
        };
    }

    fn evolve_c(&mut self, zone: CastleZone) {
        self.rights = self.rights.apply_castling(zone.side());
        self.toggle_castle_pieces(zone);
        self.enpassant = None;
        self.clock += 1;
    }

    fn evolve_e(&mut self, side: Side, from: Square, dest: Square, capture: Square) {
        self.toggle_enpassant_pieces(side, from, dest, capture);
        self.enpassant = None;
        self.clock = 0;
    }

    fn evolve_p(&mut self, from: Square, dest: Square, promoted: Piece, captured: Option<Piece>) {
        let moved = Piece::pawn(promoted.side());
        self.pieces.toggle_piece(moved, &[from]);
        self.pieces.toggle_piece(promoted, &[dest]);
        match captured {
            None => {}
            Some(p) => self.pieces.toggle_piece(p, &[dest]),
        };
        self.enpassant = None;
        self.clock = 0;
    }

    /// Public API for devolving a move, the information lost at evolve time is
    /// required as an input here to recover the lost state exactly.
    pub(crate) fn unmake(&mut self) -> Result<Move> {
        let (mv, state) = self.history.attempt_pop()?;

        match &mv {
            &Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => self.devolve_s(moving, from, dest, capture),

            &Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => self.devolve_p(from, dest, promoted, capture),
            &Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => self.devolve_e(side, from, dest, capture),
            &Castle { zone, .. } => self.devolve_c(zone),
        };

        self.rights = state.rights;
        self.clock = state.half_move_clock;
        self.enpassant = state.enpassant;
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(mv)
    }

    fn devolve_s(&mut self, piece: Piece, source: Square, target: Square, captured: Option<Piece>) {
        self.pieces.toggle_piece(piece, &[target, source]);
        match captured {
            None => {}
            Some(p) => self.pieces.toggle_piece(p, &[target]),
        };
    }

    fn devolve_c(&mut self, zone: CastleZone) {
        self.toggle_castle_pieces(zone);
    }

    fn devolve_e(&mut self, side: Side, from: Square, dest: Square, capture: Square) {
        self.toggle_enpassant_pieces(side, from, dest, capture);
    }

    fn devolve_p(&mut self, from: Square, dest: Square, promoted: Piece, captured: Option<Piece>) {
        let moved_pawn = Piece::pawn(promoted.side());
        self.pieces.toggle_piece(moved_pawn, &[from]);
        self.pieces.toggle_piece(promoted, &[dest]);
        match captured {
            None => {}
            Some(p) => self.pieces.toggle_piece(p, &[dest]),
        };
    }

    fn toggle_castle_pieces(&mut self, zone: CastleZone) {
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
    }

    fn toggle_enpassant_pieces(
        &mut self,
        side: Side,
        source: Square,
        dest: Square,
        capture: Square,
    ) {
        self.pieces.toggle_piece(Piece::pawn(side), &[source, dest]);
        self.pieces
            .toggle_piece(Piece::pawn(side.reflect()), &[capture]);
    }

    /// Determines the enpassant square for the next board state given a
    /// piece which has just moved from the source to the target.
    fn compute_enpassant(source: Square, target: Square, piece: Piece) -> Option<Square> {
        if piece.is_pawn() {
            let side = piece.side();
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
