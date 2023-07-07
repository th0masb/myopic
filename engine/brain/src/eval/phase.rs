use enum_map::{enum_map, EnumMap};

use myopic_board::{Board, Move};

use crate::eval::Evaluation;
use crate::{Class, Piece, Square};

const MAX_PHASE: i32 = 256;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Phase {
    phase_values: EnumMap<Class, i32>,
    phase_counter: i32,
    total_phase: i32,
    phase: i32,
}

impl Default for Phase {
    fn default() -> Self {
        let phase_values = enum_map! {
            Class::P => 0i32, Class::N => 1i32, Class::B => 1i32,
            Class::R => 2i32, Class::Q => 6i32, Class::K => 0i32,
        };
        Phase {
            phase_counter: 0,
            phase: 0,
            total_phase: 16 * phase_values[Class::P]
                + 4 * (phase_values[Class::N] + phase_values[Class::B] + phase_values[Class::R])
                + 2 * phase_values[Class::Q],
            phase_values,
        }
    }
}

impl<'a> From<&'a Board> for Phase {
    fn from(value: &Board) -> Self {
        let mut phase = Phase::default();
        phase.phase_counter = phase.total_phase
            - Square::iter()
                .flat_map(|sq| value.piece(sq))
                .map(|Piece(_, class)| phase.phase_values[class])
                .sum::<i32>();
        phase.update_phase();
        phase
    }
}

impl Phase {
    pub fn unwrap(&self, eval: Evaluation) -> i32 {
        match eval {
            Evaluation::Single(eval) => eval,
            Evaluation::Phased { mid, end } => self.interpolate(mid, end),
        }
    }

    pub fn phase_progression(&self) -> f32 {
        (self.phase as f32) / (MAX_PHASE as f32)
    }

    pub fn interpolate(&self, mid: i32, end: i32) -> i32 {
        ((mid * (MAX_PHASE - self.phase)) + end * self.phase) / MAX_PHASE
    }

    fn update_phase(&mut self) {
        self.phase = (self.phase_counter * MAX_PHASE + self.total_phase / 2i32) / self.total_phase;
    }

    pub fn make(&mut self, mv: &Move) {
        let counter_start = self.phase_counter;
        match mv {
            Move::Null | Move::Castle { .. } => {}
            Move::Enpassant { .. } => self.phase_counter += self.phase_values[Class::P],
            Move::Standard { capture, .. } => {
                if let Some(Piece(_, class)) = capture {
                    self.phase_counter += self.phase_values[*class]
                }
            }
            Move::Promotion { promoted, .. } => {
                self.phase_counter += self.phase_values[Class::P];
                self.phase_counter -= self.phase_values[promoted.1];
            }
        }
        if self.phase_counter != counter_start {
            self.update_phase()
        }
    }

    pub fn unmake(&mut self, mv: &Move) {
        let counter_start = self.phase_counter;
        match mv {
            Move::Null | Move::Castle { .. } => {}
            Move::Enpassant { .. } => self.phase_counter -= self.phase_values[Class::P],
            Move::Standard { capture, .. } => {
                if let Some(Piece(_, class)) = capture {
                    self.phase_counter -= self.phase_values[*class]
                }
            }
            Move::Promotion { promoted, .. } => {
                self.phase_counter -= self.phase_values[Class::P];
                self.phase_counter += self.phase_values[promoted.1];
            }
        }
        if self.phase_counter != counter_start {
            self.update_phase()
        }
    }
}
