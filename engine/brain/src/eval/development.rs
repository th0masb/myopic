use std::cmp::min;
use enum_map::{Enum, enum_map, EnumMap};
use lazy_static::lazy_static;

use crate::{ChessBoard, Move};
use crate::{Reflectable, Side, Square};
use crate::eval::EvalFacet;

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
type PiecesMoved = EnumMap<Side, EnumMap<DevelopmentPiece, Option<usize>>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DevelopmentFacet {
    move_index: usize,
    pieces_moved: PiecesMoved,
    undeveloped_cost: usize,
    move_index_divisor: usize,
    max_penalty: i32,
}

impl Default for DevelopmentFacet {
    fn default() -> Self {
        DevelopmentFacet {
            move_index: 0,
            pieces_moved: Default::default(),
            undeveloped_cost: 10,
            move_index_divisor: 10,
            max_penalty: MAX_PENALTY,
        }
    }
}

impl DevelopmentFacet {
    fn matching_piece(&self, move_count: usize) -> Option<(Side, DevelopmentPiece)> {
        self.pieces_moved.iter().flat_map(|(side, ds)|
            ds.iter()
                .filter(|(_, &mv)| mv == Some(move_count))
                .map(move |(d, _)| (side, d))
        ).next()
    }

    fn penalty(&self, side: Side) -> i32 {
        // TODO Don't consider a piece developed if white just moved it and now it is blacks turn so
        //  we treat both sides equally. Is this a good idea?
        let undeveloped_count = self.pieces_moved[side].iter()
            .filter(|(_, &moved_index)| moved_index.is_none()).count() as f64;

        let move_index_mult = (self.move_index as f64 / self.move_index_divisor as f64).exp2();
        min((move_index_mult * undeveloped_count * (self.undeveloped_cost as f64)).round() as i32, self.max_penalty)
    }
}

impl <B : ChessBoard> EvalFacet<B> for DevelopmentFacet {
    fn static_eval(&self, _: &B) -> i32 {
        self.penalty(Side::Black) - self.penalty(Side::White)
    }

    fn make(&mut self, mv: &Move, _: &B) {
        if let &Move::Standard { from, .. } = mv {
            if let Some((side, piece)) = START_LOCS[from] {
                // Don't overwrite an existing entry as the piece was already moved
                if self.pieces_moved[side][piece].is_none() {
                    self.pieces_moved[side][piece] = Some(self.move_index)
                }
            }
        }
        self.move_index += 1;
    }

    fn unmake(&mut self, _: &Move) {
        self.move_index -= 1;
        if let Some((side, piece)) = self.matching_piece(self.move_index) {
            self.pieces_moved[side][piece] = None
        }
    }
}

#[cfg(test)]
mod test {
    use enum_map::enum_map;
    use myopic_board::{Board, ChessBoard};
    use crate::eval::development::{DevelopmentFacet, MAX_PENALTY, PiecesMoved};
    use crate::eval::EvalFacet;
    use crate::Side;
    use super::DevelopmentPiece;

    #[test]
    fn penalty_test() {
        let under_test = DevelopmentFacet {
            move_index: 10,
            undeveloped_cost: 3,
            move_index_divisor: 2,
            max_penalty: 10000,
            pieces_moved: enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    _ => None
                },
            },
        };

        assert_eq!(3 * 3 * 32, under_test.penalty(Side::White));
        assert_eq!(4 * 3 * 32, under_test.penalty(Side::Black));
    }

    #[test]
    fn evolution_test() {
        let mut board = Board::default();
        let moves = board.play_pgn("1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. Bxc6 bxc6 \
            5. d4 exd4 6. Nxd4 Bc5 7. Be3 Bb7 8. Nc3 d6").unwrap();

        let expected_states: Vec<DevelopmentFacet> = vec![
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    _ => None
                },
                Side::Black => Default::default(),
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    DevelopmentPiece::FBishop => Some(11),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    DevelopmentPiece::CBishop => Some(12),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    DevelopmentPiece::FBishop => Some(11),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    DevelopmentPiece::CBishop => Some(12),
                    _ => None
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    DevelopmentPiece::FBishop => Some(11),
                    DevelopmentPiece::CBishop => Some(13),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    DevelopmentPiece::CBishop => Some(12),
                    DevelopmentPiece::BKnight => Some(14),
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    DevelopmentPiece::FBishop => Some(11),
                    DevelopmentPiece::CBishop => Some(13),
                    _ => None
                },
            },
            enum_map! {
                Side::White => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    DevelopmentPiece::DPawn => Some(8),
                    DevelopmentPiece::CBishop => Some(12),
                    DevelopmentPiece::BKnight => Some(14),
                },
                Side::Black => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    DevelopmentPiece::GKnight => Some(5),
                    DevelopmentPiece::FBishop => Some(11),
                    DevelopmentPiece::CBishop => Some(13),
                    DevelopmentPiece::DPawn => Some(15),
                },
            },
        ].into_iter().enumerate().map(|(i, pieces)| DevelopmentFacet {
            move_index: i + 1,
            pieces_moved: pieces,
            move_index_divisor: 10,
            undeveloped_cost: 10,
            max_penalty: MAX_PENALTY,
        }).collect();

        let mut state = DevelopmentFacet {
            move_index: 0,
            pieces_moved: Default::default(),
            move_index_divisor: 10,
            undeveloped_cost: 10,
            max_penalty: MAX_PENALTY,
        };
        let mut board = Board::default();
        for (expected, mv) in expected_states.into_iter().zip(moves.iter()) {
            let state_start = state.clone();
            let position = board.clone();
            <DevelopmentFacet as EvalFacet<Board>>::make(&mut state, mv, &position);
            assert_eq!(expected, state);
            <DevelopmentFacet as EvalFacet<Board>>::unmake(&mut state, mv);
            assert_eq!(state_start, state);
            <DevelopmentFacet as EvalFacet<Board>>::make(&mut state, mv, &position);
            board.make(mv.clone()).unwrap();
        }
    }
}