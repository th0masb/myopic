use crate::board;
use crate::position::Position;
use crate::constants::side;
use crate::constants::square::*;

#[test]
fn control_case_1() {
    let fen = "4k3/8/8/3r4/8/8/3K4/3Q4 w - - 4 15";
    assert_eq!(
        fen.parse::<Position>().unwrap().compute_control(side::B),
        board!(
            ~E8 => F8, F7, E7, D7, D8;
            ~D5 => D1, A5, H5, D8
        )
    );
}

#[test]
fn control_case_10() {
    let fen = "r1b1kb1R/p6p/2np4/1N1q4/1n5P/P1N1B1P1/2PK1P2/R2QnB1R w q - 4 15";
    assert_eq!(
        fen.parse::<Position>().unwrap().compute_control(side::B),
        board!(
            ~H7 => G6;
            ~F8 => D6, H6;
            ~E8 => F8, F7, E7, D7, D8;
            ~E1 => G2, F3, D3, C2;
            ~D6 => E5, C5;
            ~D5 => D1, H1, H5, G8, D6, C6, B5, A2;
            ~C8 => A6, H3;
            ~C6 => D8, B8, A7, A5, B4, D4, E5, E7;
            ~B4 => A6, A2, C2, D3, D5, C6;
            ~A8 => C8, A7;
            ~A7 => B6
        )
    );
}