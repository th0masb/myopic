use enum_map::{Enum, enum_map, EnumMap};
use lazy_static::lazy_static;

use crate::{ChessBoard, Move};
use crate::{BitBoard, Side, Square};
use crate::eval::EvalFacet;

//pub trait AntiPattern<B: ChessBoard> {
//    fn push(&mut self, mv: &Move, board: &B) -> bool;
//
//    fn pop(&mut self);
//
//    fn penalty(&self) -> i32;
//}
//
//pub trait AntiPatternFacet2<B: ChessBoard> {
//    fn make(&mut self, mv: &Move, board: &B);
//
//    fn unmake(&mut self, mv: &Move);
//
//    fn penalty(&self) -> i32;
//
//}
//
//pub struct AntiPatternFacet<B, P>
//    where
//        B: ChessBoard,
//        P: AntiPattern<B>,
//{
//    move_index: usize,
//    pattern_stack: EnumMap<Side, Vec<usize>>,
//    pattern: P,
//    board_type: PhantomData<B>,
//}
//
//impl <B, P> AntiPatternFacet<B, P>
//    where
//        B: ChessBoard,
//        P: AntiPattern<B>,
//{
//    pub fn new(pattern: P) -> AntiPatternFacet<B, P> {
//        AntiPatternFacet {
//            move_index: 0,
//            pattern_stack: Default::default(),
//            pattern,
//            board_type: PhantomData
//        }
//    }
//}
//
//impl<B, P> EvalFacet<B> for AntiPatternFacet<B, P>
//    where
//        B: ChessBoard,
//        P: AntiPattern<B>,
//{
//    fn static_eval(&self, _: &B) -> i32 {
//        (self.pattern_stack[Side::White].len() as i32 -
//            self.pattern_stack[Side::Black].len() as i32) * self.pattern.penalty()
//    }
//
//    fn make(&mut self, mv: &Move, board: &B) {
//        if self.pattern.push(mv, board) {
//            self.pattern_stack[board.active()].push(self.move_index);
//        }
//        self.move_index += 1
//    }
//
//    fn unmake(&mut self, mv: &Move) {
//        self.move_index -= 1;
//        if self.pattern_stack[mv.moving_side()].last().cloned() == Some(self.move_index) {
//            self.pattern_stack[mv.moving_side()].pop();
//            self.pattern.pop();
//        }
//    }
//}


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
        if let Move::Standard { from, dest, .. } = mv {
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