use enum_map::{enum_map, Enum, EnumMap};
use lazy_static::lazy_static;

use crate::eval::EvalFacet;
use crate::{BitBoard, Side, Square};
use crate::{ChessBoard, Move};

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
enum Knight {
    B,
    G,
}

type FirstMoveStore = EnumMap<Side, EnumMap<Knight, Option<(usize, Square)>>>;

/// Give penalty for each knight whose first move is onto the board rim
#[derive(Debug, Clone, PartialEq)]
pub struct KnightRimFacet {
    penalty: i32,
    first_move: FirstMoveStore,
    move_index: usize,
}

impl Default for KnightRimFacet {
    fn default() -> Self {
        KnightRimFacet { penalty: 80, first_move: Default::default(), move_index: 0 }
    }
}

impl KnightRimFacet {
    fn pattern_count(&self, side: Side) -> i32 {
        self.first_move[side]
            .iter()
            .filter(|(_, &fm)| fm.is_some() && BitBoard::RIM.contains(fm.unwrap().1))
            .count() as i32
    }
}

lazy_static! {
    static ref START_LOCS: EnumMap<Square, Option<(Side, Knight)>> = enum_map! {
        Square::B1 => Some((Side::W, Knight::B)),
        Square::B8 => Some((Side::B, Knight::B)),
        Square::G1 => Some((Side::W, Knight::G)),
        Square::G8 => Some((Side::B, Knight::G)),
        _ => None,
    };
}

impl<B: ChessBoard> EvalFacet<B> for KnightRimFacet {
    fn static_eval(&self, _: &B) -> i32 {
        self.penalty * (self.pattern_count(Side::B) - self.pattern_count(Side::W))
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

#[cfg(test)]
mod test {
    use super::Knight;
    use crate::eval::antipattern::{FirstMoveStore, KnightRimFacet};
    use crate::eval::EvalFacet;
    use crate::test::facets::test_facet_evolution;
    use crate::{Side, Square};
    use enum_map::enum_map;
    use myopic_board::Board;

    #[test]
    fn evaluation() {
        let facet = KnightRimFacet {
            penalty: 75,
            move_index: 9,
            first_move: enum_map! {
                Side::W => enum_map! {
                    Knight::B => Some((3, Square::C3)),
                    Knight::G => Some((3, Square::H3)),
                },
                Side::B => enum_map! {
                    Knight::B => Some((3, Square::C6)),
                    Knight::G => None,
                },
            },
        };

        assert_eq!(-75, facet.static_eval(&Board::default()));
    }

    #[test]
    fn evolution() {
        test_facet_evolution(
            "1. e4 Nc6 2. d4 d5 3. Nf3 Nh6 4. Na3 f6",
            vec![
                FirstMoveStore::default(),
                enum_map! {
                    Side::W => Default::default(),
                    Side::B => enum_map! {
                        Knight::G => None,
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => Default::default(),
                    Side::B => enum_map! {
                        Knight::G => None,
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => Default::default(),
                    Side::B => enum_map! {
                        Knight::G => None,
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        Knight::G => Some((4, Square::F3)),
                        Knight::B => None
                    },
                    Side::B => enum_map! {
                        Knight::G => None,
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        Knight::G => Some((4, Square::F3)),
                        Knight::B => None
                    },
                    Side::B => enum_map! {
                        Knight::G => Some((5, Square::H6)),
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        Knight::G => Some((4, Square::F3)),
                        Knight::B => Some((6, Square::A3))
                    },
                    Side::B => enum_map! {
                        Knight::G => Some((5, Square::H6)),
                        Knight::B => Some((1, Square::C6))
                    },
                },
                enum_map! {
                    Side::W => enum_map! {
                        Knight::G => Some((4, Square::F3)),
                        Knight::B => Some((6, Square::A3))
                    },
                    Side::B => enum_map! {
                        Knight::G => Some((5, Square::H6)),
                        Knight::B => Some((1, Square::C6))
                    },
                },
            ]
            .into_iter()
            .enumerate()
            .map(|(i, first_move)| {
                let mut facet = KnightRimFacet::default();
                facet.first_move = first_move;
                facet.move_index = i + 1;
                facet
            })
            .collect(),
        )
    }
}
