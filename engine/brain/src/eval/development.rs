use std::cmp::min;

use enum_map::{enum_map, Enum, EnumMap};
use lazy_static::lazy_static;
use myopic_board::Board;

use crate::eval::{EvalFacet, Evaluation};
use crate::Move;
use crate::{Side, Square};

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
        Square::E2 => Some((Side::W, DevelopmentPiece::EPawn)),
        Square::E7 => Some((Side::B, DevelopmentPiece::EPawn)),
        Square::D2 => Some((Side::W, DevelopmentPiece::DPawn)),
        Square::D7 => Some((Side::B, DevelopmentPiece::DPawn)),
        Square::B1 => Some((Side::W, DevelopmentPiece::BKnight)),
        Square::B8 => Some((Side::B, DevelopmentPiece::BKnight)),
        Square::G1 => Some((Side::W, DevelopmentPiece::GKnight)),
        Square::G8 => Some((Side::B, DevelopmentPiece::GKnight)),
        Square::C1 => Some((Side::W, DevelopmentPiece::CBishop)),
        Square::C8 => Some((Side::B, DevelopmentPiece::CBishop)),
        Square::F1 => Some((Side::W, DevelopmentPiece::FBishop)),
        Square::F8 => Some((Side::B, DevelopmentPiece::FBishop)),
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
        self.pieces_moved
            .iter()
            .flat_map(|(side, ds)| {
                ds.iter().filter(|(_, &mv)| mv == Some(move_count)).map(move |(d, _)| (side, d))
            })
            .next()
    }

    fn penalty(&self, side: Side) -> i32 {
        // TODO Don't consider a piece developed if white just moved it and now it is blacks turn so
        //  we treat both sides equally. Is this a good idea?
        let undeveloped_count = self.pieces_moved[side]
            .iter()
            .filter(|(_, &moved_index)| moved_index.is_none())
            .count() as f64;

        let move_index_mult = (self.move_index as f64 / self.move_index_divisor as f64).exp2();
        min(
            (move_index_mult * undeveloped_count * (self.undeveloped_cost as f64)).round() as i32,
            self.max_penalty,
        )
    }
}

impl EvalFacet for DevelopmentFacet {
    fn static_eval(&self, _: &Board) -> Evaluation {
        Evaluation::Single(self.penalty(Side::B) - self.penalty(Side::W))
    }

    fn make(&mut self, mv: &Move, _: &Board) {
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

    use crate::eval::development::DevelopmentFacet;
    use crate::eval::{EvalFacet, Evaluation};
    use crate::test::facets::test_facet_evolution;
    use crate::Board;
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
                Side::W => enum_map! {
                    DevelopmentPiece::EPawn => Some(0),
                    DevelopmentPiece::GKnight => Some(2),
                    DevelopmentPiece::FBishop => Some(4),
                    _ => None
                },
                Side::B => enum_map! {
                    DevelopmentPiece::EPawn => Some(1),
                    DevelopmentPiece::BKnight => Some(3),
                    _ => None
                },
            },
        };

        assert_eq!(3 * 3 * 32, under_test.penalty(Side::W));
        assert_eq!(4 * 3 * 32, under_test.penalty(Side::B));
        assert_eq!(Evaluation::Single(1 * 3 * 32), under_test.static_eval(&Board::default()));
    }

    #[test]
    fn evolution() {
        test_facet_evolution(
            "1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. Bxc6 bxc6 5. d4 exd4 6. Nxd4 Bc5 7. Be3 Bb7 8. Nc3 d6",
            vec![
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        _ => None
                    },
                    Side::B => Default::default(),
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        DevelopmentPiece::FBishop => Some(11),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        DevelopmentPiece::CBishop => Some(12),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        DevelopmentPiece::FBishop => Some(11),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        DevelopmentPiece::CBishop => Some(12),
                        _ => None
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        DevelopmentPiece::FBishop => Some(11),
                        DevelopmentPiece::CBishop => Some(13),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        DevelopmentPiece::CBishop => Some(12),
                        DevelopmentPiece::BKnight => Some(14),
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        DevelopmentPiece::FBishop => Some(11),
                        DevelopmentPiece::CBishop => Some(13),
                        _ => None
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        DevelopmentPiece::EPawn => Some(0),
                        DevelopmentPiece::GKnight => Some(2),
                        DevelopmentPiece::FBishop => Some(4),
                        DevelopmentPiece::DPawn => Some(8),
                        DevelopmentPiece::CBishop => Some(12),
                        DevelopmentPiece::BKnight => Some(14),
                    },
                    Side::B => enum_map! {
                        DevelopmentPiece::EPawn => Some(1),
                        DevelopmentPiece::BKnight => Some(3),
                        DevelopmentPiece::GKnight => Some(5),
                        DevelopmentPiece::FBishop => Some(11),
                        DevelopmentPiece::CBishop => Some(13),
                        DevelopmentPiece::DPawn => Some(15),
                    },
                },
            ].into_iter().enumerate().map(|(i, pieces)| {
                let mut facet = DevelopmentFacet::default();
                facet.move_index = i + 1;
                facet.pieces_moved = pieces;
                facet
            }).collect(),
        )
    }
}
