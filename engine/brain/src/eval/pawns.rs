use crate::eval::EvalComponent;
use crate::Move;
use myopic_board::enum_map::{enum_map, Enum, EnumMap};

#[derive(Debug, Copy, PartialEq, Enum)]
enum ScoreParam {
    IsolatedPawn,
    DoubledPawn,
    PassedPawn,
    PassedPawnProximityBonus,
    PassedPawnConnectedBonus,
}

struct ScoreParams {
    content: EnumMap<ScoreParam, i32>,
}

pub struct PawnStructureEvalComponent;

impl EvalComponent for PawnStructureEvalComponent {
    fn static_eval(&self) -> i32 {
        // - Passed pawns (weighted dependent on proximity to opposite side)
        //     + Probably want to consider bonus for connected passed pawns
        // - Isolated pawns
        // - Doubled pawns
        // - Size of pawn chains
        //
        todo!()
    }

    fn make(&mut self, _mv: &Move) {}

    fn unmake(&mut self, _mv: &Move) {}
}
