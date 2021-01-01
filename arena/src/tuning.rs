use myopic_brain::{EvalConfig, PieceValues, PositionTables, OpeningRewards};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tuner {
//    Tables(TablesTuner),
//    Values(ValuesTuner),
    Openings(OpeningRewardsTuner),
    TablesMultiplier(FixedFloatset),
    ValuesMultiplier(FixedFloatset),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedFloatset {
    values: Vec<f64>
}

impl Tuner {
    pub fn combinations(&self) -> Vec<EvalConfig> {
        match self {
            Tuner::TablesMultiplier(fset) => {
                let defaults = PositionTables::default();
                fset.values
                    .iter()
                    .map(|&x| multiply_tables(&defaults, x))
                    .map(|t| EvalConfig::Tables(t))
                    .collect()
            }
            Tuner::ValuesMultiplier(fset) => {
                let defaults = PieceValues::default();
                fset.values
                    .iter()
                    .map(|&x| multiply_values(&defaults, x))
                    .map(|t| EvalConfig::Values(t))
                    .collect()
            }
            Tuner::Openings(rewards_tuner) => {
                crate::cartesian::product(&vec![
                    rewards_tuner.d_pawn.values(),
                    rewards_tuner.e_pawn.values(),
                    rewards_tuner.f_bishop.values(),
                    rewards_tuner.c_bishop.values(),
                    rewards_tuner.b_knight.values(),
                    rewards_tuner.g_knight.values(),
                    rewards_tuner.q_castle.values(),
                    rewards_tuner.k_castle.values(),
                ])
                    .into_iter()
                    .map(|xs| EvalConfig::Openings(
                        OpeningRewards {
                            d_pawn: xs[0],
                            e_pawn: xs[1],
                            f_bishop: xs[2],
                            c_bishop: xs[3],
                            b_knight: xs[4],
                            g_knight: xs[5],
                            q_castle: xs[6],
                            k_castle: xs[7],
                        }
                    )).collect()
            }
        }
    }
}

fn times(n: i32, x: f64) -> i32 {
    ((n as f64) * x).round() as i32
}

fn times_pair(n: (i32, i32), x: f64) -> (i32, i32) {
    (times(n.0, x), times(n.1, x))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablesTuner {
    pub pawn_1_4: [TuningRange; 32],
    pub pawn_5_8: [TuningRange; 32],
    pub knight: [TuningRange; 32],
    pub bishop: [TuningRange; 32],
    pub rook: [TuningRange; 32],
    pub queen: [TuningRange; 32],
    pub king: [TuningRange; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesTuner {
    pub midgame: [TuningRange; 6],
    pub endgame: [TuningRange; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningRewardsTuner {
    pub e_pawn: TuningRange,
    pub d_pawn: TuningRange,
    pub b_knight: TuningRange,
    pub g_knight: TuningRange,
    pub c_bishop: TuningRange,
    pub f_bishop: TuningRange,
    pub k_castle: TuningRange,
    pub q_castle: TuningRange,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TuningRange {
    pub min: i32,
    pub max: i32,
    pub step: usize,
}

impl TuningRange {
    pub fn values(&self) -> Vec<i32> {
        (self.min..=self.max).step_by(self.step).collect()
    }
}

fn multiply_values(piece_values: &PieceValues, x: f64) -> PieceValues {
    PieceValues {
        midgame: [
            times(piece_values.midgame[0], x),
            times(piece_values.midgame[1], x),
            times(piece_values.midgame[2], x),
            times(piece_values.midgame[3], x),
            times(piece_values.midgame[4], x),
            times(piece_values.midgame[5], x),
        ],
        endgame: [
            times(piece_values.endgame[0], x),
            times(piece_values.endgame[1], x),
            times(piece_values.endgame[2], x),
            times(piece_values.endgame[3], x),
            times(piece_values.endgame[4], x),
            times(piece_values.endgame[5], x),
        ]
    }
}

// tedious long function
fn multiply_tables(tables: &PositionTables, x: f64) -> PositionTables {
    PositionTables {
        pawn_1_4: [
            times_pair(tables.pawn_1_4[0], x),
            times_pair(tables.pawn_1_4[1], x),
            times_pair(tables.pawn_1_4[2], x),
            times_pair(tables.pawn_1_4[3], x),
            times_pair(tables.pawn_1_4[4], x),
            times_pair(tables.pawn_1_4[5], x),
            times_pair(tables.pawn_1_4[6], x),
            times_pair(tables.pawn_1_4[7], x),
            times_pair(tables.pawn_1_4[8], x),
            times_pair(tables.pawn_1_4[9], x),
            times_pair(tables.pawn_1_4[10], x),
            times_pair(tables.pawn_1_4[11], x),
            times_pair(tables.pawn_1_4[12], x),
            times_pair(tables.pawn_1_4[13], x),
            times_pair(tables.pawn_1_4[14], x),
            times_pair(tables.pawn_1_4[15], x),
            times_pair(tables.pawn_1_4[16], x),
            times_pair(tables.pawn_1_4[17], x),
            times_pair(tables.pawn_1_4[18], x),
            times_pair(tables.pawn_1_4[19], x),
            times_pair(tables.pawn_1_4[20], x),
            times_pair(tables.pawn_1_4[21], x),
            times_pair(tables.pawn_1_4[22], x),
            times_pair(tables.pawn_1_4[23], x),
            times_pair(tables.pawn_1_4[24], x),
            times_pair(tables.pawn_1_4[25], x),
            times_pair(tables.pawn_1_4[26], x),
            times_pair(tables.pawn_1_4[27], x),
            times_pair(tables.pawn_1_4[28], x),
            times_pair(tables.pawn_1_4[29], x),
            times_pair(tables.pawn_1_4[30], x),
            times_pair(tables.pawn_1_4[31], x),
        ],
        pawn_5_8: [
            times_pair(tables.pawn_5_8[0], x),
            times_pair(tables.pawn_5_8[1], x),
            times_pair(tables.pawn_5_8[2], x),
            times_pair(tables.pawn_5_8[3], x),
            times_pair(tables.pawn_5_8[4], x),
            times_pair(tables.pawn_5_8[5], x),
            times_pair(tables.pawn_5_8[6], x),
            times_pair(tables.pawn_5_8[7], x),
            times_pair(tables.pawn_5_8[8], x),
            times_pair(tables.pawn_5_8[9], x),
            times_pair(tables.pawn_5_8[10], x),
            times_pair(tables.pawn_5_8[11], x),
            times_pair(tables.pawn_5_8[12], x),
            times_pair(tables.pawn_5_8[13], x),
            times_pair(tables.pawn_5_8[14], x),
            times_pair(tables.pawn_5_8[15], x),
            times_pair(tables.pawn_5_8[16], x),
            times_pair(tables.pawn_5_8[17], x),
            times_pair(tables.pawn_5_8[18], x),
            times_pair(tables.pawn_5_8[19], x),
            times_pair(tables.pawn_5_8[20], x),
            times_pair(tables.pawn_5_8[21], x),
            times_pair(tables.pawn_5_8[22], x),
            times_pair(tables.pawn_5_8[23], x),
            times_pair(tables.pawn_5_8[24], x),
            times_pair(tables.pawn_5_8[25], x),
            times_pair(tables.pawn_5_8[26], x),
            times_pair(tables.pawn_5_8[27], x),
            times_pair(tables.pawn_5_8[28], x),
            times_pair(tables.pawn_5_8[29], x),
            times_pair(tables.pawn_5_8[30], x),
            times_pair(tables.pawn_5_8[31], x),
        ],
        knight: [
            times_pair(tables.knight[0], x),
            times_pair(tables.knight[1], x),
            times_pair(tables.knight[2], x),
            times_pair(tables.knight[3], x),
            times_pair(tables.knight[4], x),
            times_pair(tables.knight[5], x),
            times_pair(tables.knight[6], x),
            times_pair(tables.knight[7], x),
            times_pair(tables.knight[8], x),
            times_pair(tables.knight[9], x),
            times_pair(tables.knight[10], x),
            times_pair(tables.knight[11], x),
            times_pair(tables.knight[12], x),
            times_pair(tables.knight[13], x),
            times_pair(tables.knight[14], x),
            times_pair(tables.knight[15], x),
            times_pair(tables.knight[16], x),
            times_pair(tables.knight[17], x),
            times_pair(tables.knight[18], x),
            times_pair(tables.knight[19], x),
            times_pair(tables.knight[20], x),
            times_pair(tables.knight[21], x),
            times_pair(tables.knight[22], x),
            times_pair(tables.knight[23], x),
            times_pair(tables.knight[24], x),
            times_pair(tables.knight[25], x),
            times_pair(tables.knight[26], x),
            times_pair(tables.knight[27], x),
            times_pair(tables.knight[28], x),
            times_pair(tables.knight[29], x),
            times_pair(tables.knight[30], x),
            times_pair(tables.knight[31], x),
        ],
        bishop: [
            times_pair(tables.bishop[0], x),
            times_pair(tables.bishop[1], x),
            times_pair(tables.bishop[2], x),
            times_pair(tables.bishop[3], x),
            times_pair(tables.bishop[4], x),
            times_pair(tables.bishop[5], x),
            times_pair(tables.bishop[6], x),
            times_pair(tables.bishop[7], x),
            times_pair(tables.bishop[8], x),
            times_pair(tables.bishop[9], x),
            times_pair(tables.bishop[10], x),
            times_pair(tables.bishop[11], x),
            times_pair(tables.bishop[12], x),
            times_pair(tables.bishop[13], x),
            times_pair(tables.bishop[14], x),
            times_pair(tables.bishop[15], x),
            times_pair(tables.bishop[16], x),
            times_pair(tables.bishop[17], x),
            times_pair(tables.bishop[18], x),
            times_pair(tables.bishop[19], x),
            times_pair(tables.bishop[20], x),
            times_pair(tables.bishop[21], x),
            times_pair(tables.bishop[22], x),
            times_pair(tables.bishop[23], x),
            times_pair(tables.bishop[24], x),
            times_pair(tables.bishop[25], x),
            times_pair(tables.bishop[26], x),
            times_pair(tables.bishop[27], x),
            times_pair(tables.bishop[28], x),
            times_pair(tables.bishop[29], x),
            times_pair(tables.bishop[30], x),
            times_pair(tables.bishop[31], x),
        ],
        rook: [
            times_pair(tables.rook[0], x),
            times_pair(tables.rook[1], x),
            times_pair(tables.rook[2], x),
            times_pair(tables.rook[3], x),
            times_pair(tables.rook[4], x),
            times_pair(tables.rook[5], x),
            times_pair(tables.rook[6], x),
            times_pair(tables.rook[7], x),
            times_pair(tables.rook[8], x),
            times_pair(tables.rook[9], x),
            times_pair(tables.rook[10], x),
            times_pair(tables.rook[11], x),
            times_pair(tables.rook[12], x),
            times_pair(tables.rook[13], x),
            times_pair(tables.rook[14], x),
            times_pair(tables.rook[15], x),
            times_pair(tables.rook[16], x),
            times_pair(tables.rook[17], x),
            times_pair(tables.rook[18], x),
            times_pair(tables.rook[19], x),
            times_pair(tables.rook[20], x),
            times_pair(tables.rook[21], x),
            times_pair(tables.rook[22], x),
            times_pair(tables.rook[23], x),
            times_pair(tables.rook[24], x),
            times_pair(tables.rook[25], x),
            times_pair(tables.rook[26], x),
            times_pair(tables.rook[27], x),
            times_pair(tables.rook[28], x),
            times_pair(tables.rook[29], x),
            times_pair(tables.rook[30], x),
            times_pair(tables.rook[31], x),
        ],
        queen: [
            times_pair(tables.queen[0], x),
            times_pair(tables.queen[1], x),
            times_pair(tables.queen[2], x),
            times_pair(tables.queen[3], x),
            times_pair(tables.queen[4], x),
            times_pair(tables.queen[5], x),
            times_pair(tables.queen[6], x),
            times_pair(tables.queen[7], x),
            times_pair(tables.queen[8], x),
            times_pair(tables.queen[9], x),
            times_pair(tables.queen[10], x),
            times_pair(tables.queen[11], x),
            times_pair(tables.queen[12], x),
            times_pair(tables.queen[13], x),
            times_pair(tables.queen[14], x),
            times_pair(tables.queen[15], x),
            times_pair(tables.queen[16], x),
            times_pair(tables.queen[17], x),
            times_pair(tables.queen[18], x),
            times_pair(tables.queen[19], x),
            times_pair(tables.queen[20], x),
            times_pair(tables.queen[21], x),
            times_pair(tables.queen[22], x),
            times_pair(tables.queen[23], x),
            times_pair(tables.queen[24], x),
            times_pair(tables.queen[25], x),
            times_pair(tables.queen[26], x),
            times_pair(tables.queen[27], x),
            times_pair(tables.queen[28], x),
            times_pair(tables.queen[29], x),
            times_pair(tables.queen[30], x),
            times_pair(tables.queen[31], x),
        ],
        king: [
            times_pair(tables.king[0], x),
            times_pair(tables.king[1], x),
            times_pair(tables.king[2], x),
            times_pair(tables.king[3], x),
            times_pair(tables.king[4], x),
            times_pair(tables.king[5], x),
            times_pair(tables.king[6], x),
            times_pair(tables.king[7], x),
            times_pair(tables.king[8], x),
            times_pair(tables.king[9], x),
            times_pair(tables.king[10], x),
            times_pair(tables.king[11], x),
            times_pair(tables.king[12], x),
            times_pair(tables.king[13], x),
            times_pair(tables.king[14], x),
            times_pair(tables.king[15], x),
            times_pair(tables.king[16], x),
            times_pair(tables.king[17], x),
            times_pair(tables.king[18], x),
            times_pair(tables.king[19], x),
            times_pair(tables.king[20], x),
            times_pair(tables.king[21], x),
            times_pair(tables.king[22], x),
            times_pair(tables.king[23], x),
            times_pair(tables.king[24], x),
            times_pair(tables.king[25], x),
            times_pair(tables.king[26], x),
            times_pair(tables.king[27], x),
            times_pair(tables.king[28], x),
            times_pair(tables.king[29], x),
            times_pair(tables.king[30], x),
            times_pair(tables.king[31], x),
        ],
    }
}
