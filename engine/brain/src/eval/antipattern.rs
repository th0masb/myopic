use enum_map::{Enum, enum_map, EnumMap};
use lazy_static::lazy_static;

use crate::{ChessBoard, Move};
use crate::{BitBoard, Side, Square};
use crate::eval::EvalFacet;

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
enum Knight { B, G }

/// Give penalty for each knight whose first move is onto the board rim
pub struct KnightRimFacet {
    penalty: i32,
    first_move: EnumMap<Side, EnumMap<Knight, Option<(usize, Square)>>>,
    move_index: usize,
}

impl Default for KnightRimFacet {
    fn default() -> Self {
        KnightRimFacet {
            penalty: 80,
            first_move: Default::default(),
            move_index: 0,
        }
    }
}

impl KnightRimFacet {
    fn pattern_count(&self, side: Side) -> i32 {
        self.first_move[side].iter()
            .filter(|(_, &fm)| fm.is_some() && BitBoard::RIM.contains(fm.unwrap().1))
            .count() as i32
    }
}

lazy_static! {
    static ref START_LOCS: EnumMap<Square, Option<(Side, Knight)>> = enum_map! {
        Square::B1 => Some((Side::White, Knight::B)),
        Square::B8 => Some((Side::Black, Knight::B)),
        Square::G1 => Some((Side::White, Knight::G)),
        Square::G8 => Some((Side::Black, Knight::G)),
        _ => None,
    };
}

impl <B: ChessBoard> EvalFacet<B> for KnightRimFacet {
    fn static_eval(&self, _: &B) -> i32 {
       self.penalty * (self.pattern_count(Side::Black) - self.pattern_count(Side::White))
    }

    fn make(&mut self, mv: &Move, _: &B) {
        if let Move::Standard { from, dest, .. } = mv {
            if let Some((side, knight)) = START_LOCS[*from] {
                if self.first_move[side][knight].is_none() {
                    self.first_move[side][knight] = Some((self.move_index, *dest))
                }
            }
        }
        self.move_index += 1
    }

    fn unmake(&mut self, mv: &Move) {
        self.move_index -= 1;
        if let Move::Standard { from, .. } = mv {
            if let Some((side, knight)) = START_LOCS[*from] {
                if let Some((index, _)) = self.first_move[side][knight] {
                   if index == self.move_index {
                       self.first_move[side][knight] = None
                   }
                }
            }
        }
    }
}