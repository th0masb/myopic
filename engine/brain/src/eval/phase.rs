use enum_map::{enum_map, EnumMap};

use myopic_board::{Board, Move};

use crate::{Class, Piece, Square};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Phase {
    phase: i32,
    phase_values: EnumMap<Class, i32>,
    total_phase: i32,
}

impl Default for Phase {
    fn default() -> Self {
        let phase_values = enum_map! {
            Class::P => 0i32, Class::N => 1i32, Class::B => 1i32,
            Class::R => 2i32, Class::Q => 4i32, Class::K => 0i32,
        };
        Phase {
            phase: 0,
            total_phase: 16 * phase_values[Class::P] +
                4 * (phase_values[Class::N] + phase_values[Class::B] + phase_values[Class::R]) +
                2 * phase_values[Class::Q],
            phase_values
        }
    }
}

impl <'a> From<&'a Board> for Phase {
    fn from(value: &Board) -> Self {
        let mut phase = Phase::default();
        phase.phase = phase.total_phase - Square::iter()
            .flat_map(|sq| value.piece(sq))
            .map(|Piece(_, class)| phase.phase_values[class])
            .sum::<i32>();
        phase
    }
}

impl Phase {
    pub fn interpolate(&self, mid: i32, end: i32) -> i32 {
        let phase = (self.phase * 256i32 + self.total_phase / 2i32) / self.total_phase;
        ((mid * (256 - phase)) + end * phase) / 256
    }

    pub fn make(&mut self, mv: &Move) {
        match mv {
            Move::Castle { .. } => {}
            Move::Enpassant { .. } => {
                self.phase += self.phase_values[Class::P]
            }
            Move::Standard { capture, .. } => {
                if let Some(Piece(_, class)) = capture {
                    self.phase += self.phase_values[*class]
                }
            }
            Move::Promotion { promoted, .. } => {
                self.phase += self.phase_values[Class::P];
                self.phase -= self.phase_values[promoted.1];
            }
        }
    }

    pub fn unmake(&mut self, mv: &Move) {
        match mv {
            Move::Castle { .. } => {}
            Move::Enpassant { .. } => {
                self.phase -= self.phase_values[Class::P]
            }
            Move::Standard { capture, .. } => {
                if let Some(Piece(_, class)) = capture {
                    self.phase -= self.phase_values[*class]
                }
            }
            Move::Promotion { promoted, .. } => {
                self.phase -= self.phase_values[Class::P];
                self.phase += self.phase_values[promoted.1];
            }
        }
    }
}