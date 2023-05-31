use std::cmp::min;
use enum_map::{Enum, enum_map, EnumMap};
use lazy_static::lazy_static;

use crate::{ChessBoard, Move};
use crate::{Reflectable, Side, Square};
use crate::eval::EvalComponent;

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
enum DevelopmentPiece {
    EPawn,
    DPawn,
    BKnight,
    GKnight,
    CBishop,
    FBishop,
}

lazy_static! {
    static ref START_LOCS: EnumMap<Square, Option<(Side, DevelopmentPiece)>> = enum_map! {
        Square::E2 => Some((Side::White, DevelopmentPiece::EPawn)),
        Square::E7 => Some((Side::Black, DevelopmentPiece::EPawn)),
        Square::D2 => Some((Side::White, DevelopmentPiece::DPawn)),
        Square::D7 => Some((Side::Black, DevelopmentPiece::DPawn)),
        Square::B1 => Some((Side::White, DevelopmentPiece::BKnight)),
        Square::B8 => Some((Side::Black, DevelopmentPiece::BKnight)),
        Square::G1 => Some((Side::White, DevelopmentPiece::GKnight)),
        Square::G8 => Some((Side::Black, DevelopmentPiece::GKnight)),
        Square::C1 => Some((Side::White, DevelopmentPiece::CBishop)),
        Square::C8 => Some((Side::Black, DevelopmentPiece::CBishop)),
        Square::F1 => Some((Side::White, DevelopmentPiece::FBishop)),
        Square::F8 => Some((Side::Black, DevelopmentPiece::FBishop)),
        _ => None,
    };
}

const MAX_PENALTY: i32 = 300;

#[derive(Default, Eq, PartialEq)]
pub struct DevelopmentEvalComponent {
    move_index: usize,
    piece_moved: EnumMap<Side, EnumMap<DevelopmentPiece, Option<usize>>>,
}

impl DevelopmentEvalComponent {
    fn matching_piece(&self, move_count: usize) -> Option<(Side, DevelopmentPiece)> {
        self.piece_moved.iter().flat_map(|(side, ds)|
            ds.iter()
                .filter(|(_, &mv)| mv == Some(move_count))
                .map(move |(d, _)| (side, d))
        ).next()
    }

    fn penalty(&self, side: Side) -> i32 {
        // Don't consider a piece developed if white just moved it and now it is blacks turn so
        // we treat both sides equally
        let move_index_cutoff = (self.move_index / 2) * 2;
        let undeveloped_count = self.piece_moved[side].iter().filter(|(_, &moved_index)|
            moved_index.is_none() || moved_index.unwrap() >= move_index_cutoff
        ).count() as f64;

        let move_count_mult = (move_index_cutoff as f64 * 0.1).exp2();
        -min((move_count_mult * undeveloped_count * 10.0).round() as i32, MAX_PENALTY)
    }
}

impl <B : ChessBoard> EvalComponent<B> for DevelopmentEvalComponent {
    fn static_eval(&self, _: &B) -> i32 {
        self.penalty(Side::White) - self.penalty(Side::Black)
    }

    fn make(&mut self, mv: &Move) {
        if let &Move::Standard { from, .. } = mv {
            if let Some((side, piece)) = START_LOCS[from] {
                // Don't overwrite an existing entry as the piece was already moved
                if self.piece_moved[side][piece].is_none() {
                    self.piece_moved[side][piece] = Some(self.move_index)
                }
            }
        }
        self.move_index += 1;
    }

    fn unmake(&mut self, _: &Move) {
        self.move_index -= 1;
        if let Some((side, piece)) = self.matching_piece(self.move_index) {
            self.piece_moved[side][piece] = None
        }
    }
}