use myopic_core::*;

use crate::{Board, Move};
use crate::cache::MoveConstraints;
use crate::Square::*;

impl Reflectable for Move {
    fn reflect(&self) -> Self {
        match self {
            &Move::Standard { moving, dest, from, capture } => Move::Standard {
                moving: moving.reflect(),
                dest: dest.reflect(),
                from: from.reflect(),
                capture: capture.reflect(),
            },
            &Move::Promotion { from, dest, promoted, capture } => Move::Promotion {
                from: from.reflect(),
                dest: dest.reflect(),
                promoted: promoted.reflect(),
                capture: capture.reflect(),
            },
            &Move::Enpassant { side, from, dest, capture } => Move::Enpassant {
                side: side.reflect(),
                from: from.reflect(),
                dest: dest.reflect(),
                capture: capture.reflect(),
            },
            &Move::Castle { corner } => Move::Castle { corner: corner.reflect() },
        }
    }
}

impl Move {
    pub fn moving_side(&self) -> Side {
        match self {
            &Move::Standard { moving: Piece(side, _), .. } => side,
            &Move::Enpassant { side, .. } => side,
            &Move::Promotion { promoted: Piece(side, _), .. } => side,
            &Move::Castle { corner: Corner(side, _), .. } => side,
        }
    }

    /// Convert this move into a human readable uci long format string.
    pub fn uci_format(&self) -> String {
        match self {
            Move::Standard { from, dest, .. } => format!("{}{}", from, dest),
            Move::Enpassant { from, dest, .. } => format!("{}{}", from, dest),
            Move::Castle { corner, .. } => {
                let Line(src, dest) = Line::king_castling(*corner);
                format!("{}{}", src, dest)
            }
            Move::Promotion { from, dest, promoted: Piece(_, class), .. } => {
                format!("{}{}{}", from, dest, class)
            }
        }
    }
}

impl Board {
    pub(crate) fn all_moves_impl(&self) -> Vec<Move> {
        let moves = { self.cache.borrow().moves.clone() };
        match moves {
            Some(mvs) => mvs,
            None => {
                let result = self.compute_all_moves_impl();
                self.cache.borrow_mut().moves = Some(result.clone());
                result
            }
        }
    }

    fn compute_all_moves_impl(&self) -> Vec<Move> {
        let king_loc = self.king(self.active);
        let passive_control = self.passive_control();
        let pins = self.compute_pinned();
        let constraints = if self.in_check() {
            self.compute_check_constraints(passive_control, &pins)
        } else {
            let mut result = MoveConstraints::all(BitBoard::ALL);
            result.intersect(king_loc, !passive_control);
            result.intersect_pins(&pins);
            result
        };
        self.compute_pawn_moves(&constraints)
            .into_iter()
            .chain(self.compute_nbrqk_moves(&constraints).into_iter())
            .chain(self.compute_castle_moves(&constraints).into_iter())
            .collect()
    }

    fn compute_pawn_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let (active_pawn, (whites, blacks)) = (Piece(self.active, Class::P), self.sides());
        let compute_moves = |loc: Square| active_pawn.moves(loc, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(location) & constraints.get(location);
            moves.extend(self.standards(active_pawn, location, targets));
        }
        let active = self.active;
        for from in enpassant {
            let dest = self.enpassant.unwrap();
            let capture = dest.next(active.reflect().pawn_dir()).unwrap();
            if constraints.get(from).contains(capture)
                && self.enpassant_doesnt_discover_attack(from)
            {
                moves.push(Move::Enpassant { side: active, from, dest, capture });
            }
        }
        for location in promotion {
            let targets = compute_moves(location) & constraints.get(location);
            moves.extend(self.promotions(self.active, location, targets));
        }

        moves
    }

    fn enpassant_doesnt_discover_attack(&self, enpassant_source: Square) -> bool {
        let (active, passive) = (self.active, self.active.reflect());
        let active_king = self.king(active);
        let third_rank = passive.pawn_third_rank();
        if !third_rank.contains(active_king) {
            return true;
        }
        let (r, q) = (Piece(passive, Class::R), Piece(passive, Class::Q));
        let potential_attackers = self.locs(&[r, q]) & third_rank;
        let all_pieces = self.all_pieces();
        for loc in potential_attackers {
            let cord = BitBoard::cord(loc, active_king) & all_pieces;
            if cord.size() == 4
                && cord.contains(enpassant_source)
                && cord.intersects(self.locs(&[Piece(passive, Class::P)]))
            {
                return false;
            }
        }
        return true;
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source =
            self.enpassant.map_or(BitBoard::EMPTY, |sq| enpassant_attack_squares(self.active, sq));
        let promotion_rank = self.active.pawn_promoting_from_rank();
        let pawn_locs = self.locs(&[Piece(self.active, Class::P)]);
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_nbrqk_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        let (whites, blacks) = self.sides();
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // Add standard moves for pieces which aren't pawns
        for class in &[Class::N, Class::B, Class::R, Class::Q, Class::K] {
            let piece = Piece(self.active, *class);
            for location in self.pieces.locs(piece) {
                let moves = unchecked_moves(piece, location) & constraints.get(location);
                dest.extend(self.standards(piece, location, moves));
            }
        }
        dest
    }

    fn standards(&self, moving: Piece, from: Square, dests: BitBoard) -> Vec<Move> {
        dests
            .iter()
            .map(|dest| Move::Standard { moving, from, dest, capture: self.piece(dest) })
            .collect()
    }

    fn promotions(&self, side: Side, from: Square, dests: BitBoard) -> Vec<Move> {
        dests
            .iter()
            .flat_map(|dest| {
                [Class::Q, Class::R, Class::B, Class::N].iter().map(move |&promoted| {
                    Move::Promotion {
                        from,
                        dest,
                        promoted: Piece(side, promoted),
                        capture: self.piece(dest),
                    }
                })
            })
            .collect()
    }

    fn compute_castle_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let king_constraint = constraints.get(self.king(self.active));
        let (whites, blacks) = self.sides();
        let p1 = |c: Corner| king_constraint.subsumes(uncontrolled_req(c));
        let p2 = |c: Corner| !(whites | blacks).intersects(unoccupied_req(c));
        self.rights
            .corners()
            .filter(|Corner(side, _)| *side == self.active)
            .filter(|&c| p1(c) && p2(c))
            .filter(|&c| {
                let Line(k_source, _) = Line::king_castling(c);
                let Line(r_source, _) = Line::rook_castling(c);
                self.piece(k_source).map(|p| p.1 == Class::K).unwrap_or(false)
                    && self.piece(r_source).map(|p| p.1 == Class::R).unwrap_or(false)
            })
            .map(|corner| Move::Castle { corner })
            .collect()
    }
}

fn uncontrolled_req(Corner(side, flank): Corner) -> BitBoard {
    match (side, flank) {
        (Side::W, Flank::K) => E1 | F1 | G1,
        (Side::W, Flank::Q) => E1 | D1 | C1,
        (Side::B, Flank::K) => E8 | F8 | G8,
        (Side::B, Flank::Q) => E8 | D8 | C8,
    }
}

fn unoccupied_req(Corner(side, flank): Corner) -> BitBoard {
    match (side, flank) {
        (Side::W, Flank::K) => F1 | G1,
        (Side::W, Flank::Q) => D1 | C1 | B1,
        (Side::B, Flank::K) => F8 | G8,
        (Side::B, Flank::Q) => D8 | C8 | B8,
    }
}

fn enpassant_attack_squares(active: Side, enpassant_square: Square) -> BitBoard {
    ADJACENTS[enpassant_square.file_index() as usize] & active.reflect().pawn_third_rank()
}

const ADJACENTS: [BitBoard; 8] = [
    BitBoard(144680345676153346),
    BitBoard(361700864190383365),
    BitBoard(723401728380766730),
    BitBoard(1446803456761533460),
    BitBoard(2893606913523066920),
    BitBoard(5787213827046133840),
    BitBoard(11574427654092267680),
    BitBoard(4629771061636907072),
];

#[cfg(test)]
mod test_enpassant_source_squares {
    use myopic_core::{Side, Square::*};

    use super::enpassant_attack_squares;

    #[test]
    fn test() {
        assert_eq!(H4 | F4, enpassant_attack_squares(Side::B, G3));
        assert_eq!(!!G4, enpassant_attack_squares(Side::B, H3));
        assert_eq!(!!B4, enpassant_attack_squares(Side::B, A3));
        assert_eq!(H5 | F5, enpassant_attack_squares(Side::W, G6));
        assert_eq!(!!G5, enpassant_attack_squares(Side::W, H6));
        assert_eq!(!!B5, enpassant_attack_squares(Side::W, A6));
    }
}
