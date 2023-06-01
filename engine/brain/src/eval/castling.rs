use enum_map::EnumMap;

use crate::{CastleZone, Side};
use crate::{ChessBoard, Move};
use crate::enumset::EnumSet;
use crate::eval::EvalFacet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CastlingFacet {
    castling_status: EnumMap<Side, Option<CastleZone>>,
    penalty: i32,
}

impl Default for CastlingFacet {
    fn default() -> Self {
        CastlingFacet {
            castling_status: Default::default(),
            penalty: 100,
        }
    }
}

impl CastlingFacet {
    fn penalty(&self, side: Side, rights: &EnumSet<CastleZone>) -> i32 {
        if self.castling_status[side].is_some() {
            0
        } else {
            let rights_remaining = rights
                .iter()
                .filter(|z| z.side() == side)
                .count() as i32;
            (2i32 - rights_remaining) * self.penalty
        }
    }
}

impl<B: ChessBoard> EvalFacet<B> for CastlingFacet {
    fn static_eval(&self, board: &B) -> i32 {
        let rights = board.remaining_rights();
        self.penalty(Side::Black, &rights) - self.penalty(Side::White, &rights)
    }

    fn make(&mut self, mv: &Move, _: &B) {
        if let Move::Castle { zone, .. } = mv {
            self.castling_status[zone.side()] = Some(*zone)
        }
    }

    fn unmake(&mut self, mv: &Move) {
        if let Move::Castle { zone, .. } = mv {
            self.castling_status[zone.side()] = None
        }
    }
}

#[cfg(test)]
mod test {
    use enum_map::enum_map;
    use enumset::{EnumSet, enum_set};

    use crate::{CastleZone, Side};
    use crate::eval::castling::CastlingFacet;
    use crate::test::facets::test_facet_evolution;

    #[test]
    fn evaluation_not_castled() {
        let under_test = CastlingFacet {
            penalty: 100,
            castling_status: enum_map! {Side::White => None, Side::Black => None },
        };

        assert_eq!(200, under_test.penalty(Side::White, &enum_set!()));
        assert_eq!(200, under_test.penalty(Side::Black, &enum_set!()));

        assert_eq!(100, under_test.penalty(Side::White, &enum_set!(CastleZone::WK)));
        assert_eq!(200, under_test.penalty(Side::Black, &enum_set!(CastleZone::WK)));

        assert_eq!(200, under_test.penalty(Side::White, &enum_set!(CastleZone::BK)));
        assert_eq!(100, under_test.penalty(Side::Black, &enum_set!(CastleZone::BK)));
    }

    #[test]
    fn evolution_queenside() {
        test_facet_evolution(
            "1. d4 d5 2. Be3 Bf5 3. Nc3 Qd6 4. Qd2 Nc6 5. O-O-O O-O-O 6. g3 h6",
            vec![
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => Some(CastleZone::WQ), Side::Black => None,
                },
                enum_map! {
                    Side::White => Some(CastleZone::WQ), Side::Black => Some(CastleZone::BQ),
                },
                enum_map! {
                    Side::White => Some(CastleZone::WQ), Side::Black => Some(CastleZone::BQ),
                },
                enum_map! {
                    Side::White => Some(CastleZone::WQ), Side::Black => Some(CastleZone::BQ),
                },
            ].into_iter().map(|status| {
                let mut facet = CastlingFacet::default();
                facet.castling_status = status;
                facet
            }).collect()
        )
    }

    #[test]
    fn evolution_kingside() {
        test_facet_evolution(
            "1. e4 e5 2. Be2 Be7 3. Nf3 Nf6 4. O-O O-O 5. c4 a5",
            vec![
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => None, Side::Black => None,
                },
                enum_map! {
                    Side::White => Some(CastleZone::WK), Side::Black => None,
                },
                enum_map! {
                    Side::White => Some(CastleZone::WK), Side::Black => Some(CastleZone::BK),
                },
                enum_map! {
                    Side::White => Some(CastleZone::WK), Side::Black => Some(CastleZone::BK),
                },
                enum_map! {
                    Side::White => Some(CastleZone::WK), Side::Black => Some(CastleZone::BK),
                },
            ].into_iter().map(|status| {
                let mut facet = CastlingFacet::default();
                facet.castling_status = status;
                facet
            }).collect()
        )
    }
}