use super::*;


#[test]
fn black_move_eight() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/2pp1np1/4p3/2PPP3/1QN1BP2/PP2N1PP/R3KB1R b KQ - 3 10",
        #[rustfmt::skip]
        expected_all: vec![
            "sbpa7a6-", "sbpa7a5-",
            "sbpb7b6-", "sbpb7b5-",
            "sbnb8a6-", "sbnb8d7-",
            "sbpc6c5-",
            "sbbc8d7-", "sbbc8e6-", "sbbc8f5-", "sbbc8g4-", "sbbc8h3-",
            "sbpd6d5-",
            "sbqd8c7-", "sbqd8b6-", "sbqd8a5-", "sbqd8d7-", "sbqd8e8-", "sbqd8e7-",
            "sbpe5d4wp",
            "sbnf6d5-", "sbnf6d7-", "sbnf6e8-", "sbnf6h5-", "sbnf6g4-", "sbnf6e4wp",
            "sbrf8e8-",
            "sbkg8h8-",
            "sbbg7h6-", "sbbg7h8-",
            "sbpg6g5-",
            "sbph7h6-", "sbph7h5-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbpe5d4wp",
            "sbnf6e4wp"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbpe5d4wp",
            "sbnf6e4wp"
        ],
    })
}

#[test]
fn white_move_nine() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/2pp1np1/8/2PpP3/1QN1BP2/PP2N1PP/R3KB1R w KQ - 0 2",
        #[rustfmt::skip]
        expected_all: vec![
            "cwq",
            "swra1b1-", "swra1c1-", "swra1c1-", "swra1d1-",
            "swpa2a3-", "swpa2a4-",
            "swqb3c2-", "swqb3d1-", "swqb3a3-", "swqb3a4-", "swqb3b4-", "swqb3b5-", "swqb3b6-", "swqb3b7bp",
            "swnc3b1-", "swnc3d1-", "swnc3d5-", "swnc3d5-", "swnc3a4-", "swnc3b5-",
            "swpc4c5-",
            "swke1d1-", "swke1d2-", "swke1f2-",
            "swne2c1-", "swne2d4bp", "swne2f4-", "swne2g3-", "swne2g1-",
            "swbe3d4bp", "swbe3d2-", "swbe3c1-", "swbe3f4-", "swbe3g5-", "swbe3h6-", "swbe3f2-", "swbe3g1-",
            "swpe4e5-",
            "swpf3f4-",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-",
            "swph2h3-", "swph2h4-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swqb3b7bp",
            "swbe3d4bp",
            "swne2d4bp",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swqb3b7bp",
            "swbe3d4bp",
            "swne2d4bp",
        ],
    })
}

#[test]
fn black_move_nine() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/2pp1np1/8/2PNP3/1QN1BP2/PP4PP/R3KB1R b KQ - 0 2",
        #[rustfmt::skip]
        expected_all: vec![
            "sbpa7a6-", "sbpa7a5-",
            "sbpb7b6-", "sbpb7b5-",
            "sbnb8a6-", "sbnb8d7-",
            "sbpc6c5-",
            "sbbc8d7-", "sbbc8e6-", "sbbc8f5-", "sbbc8g4-", "sbbc8h3-",
            "sbpd6d5-",
            "sbqd8c7-", "sbqd8b6-", "sbqd8a5-", "sbqd8d7-", "sbqd8e8-", "sbqd8e7-",
            "sbnf6d5-", "sbnf6d7-", "sbnf6e8-", "sbnf6h5-", "sbnf6g4-", "sbnf6e4wp",
            "sbrf8e8-",
            "sbkg8h8-",
            "sbbg7h6-", "sbbg7h8-",
            "sbpg6g5-",
            "sbph7h6-", "sbph7h5-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbnf6e4wp"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbnf6e4wp"
        ],
    })
}

#[test]
fn white_move_ten() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/2p2np1/3p4/2PNP3/1QN1BP2/PP4PP/R3KB1R w KQ - 0 3",
        #[rustfmt::skip]
        expected_all: vec![
            "cwq",
            "swra1b1-", "swra1c1-", "swra1c1-", "swra1d1-",
            "swpa2a3-", "swpa2a4-",
            "swqb3c2-", "swqb3d1-", "swqb3a3-", "swqb3a4-", "swqb3b4-", "swqb3b5-", "swqb3b6-", "swqb3b7bp",
            "swnc3b1-", "swnc3d1-", "swnc3e2-", "swnc3d5bp", "swnc3a4-", "swnc3b5-",
            "swpc4c5-", "swpc4d5bp",
            "swke1d1-", "swke1d2-", "swke1f2-", "swke1e2-",
            "swnd4c2-", "swnd4e2-", "swnd4f5-", "swnd4e6-", "swnd4c6bp", "swnd4b5-",
            "swbf1e2-", "swbf1d3-",
            "swbe3d2-", "swbe3c1-", "swbe3f4-", "swbe3g5-", "swbe3h6-", "swbe3f2-", "swbe3g1-",
            "swpe4e5-", "swpe4d5bp",
            "swpf3f4-",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-",
            "swph2h3-", "swph2h4-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swqb3b7bp",
            "swnc3d5bp",
            "swpc4d5bp",
            "swnd4c6bp",
            "swpe4d5bp"
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swqb3b7bp",
            "swnc3d5bp",
            "swpc4d5bp",
            "swnd4c6bp",
            "swpe4d5bp"
        ],
    })
}

#[test]
fn black_move_ten() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/2p2np1/3P4/3NP3/1QN1BP2/PP4PP/R3KB1R b KQ - 0 3",
        #[rustfmt::skip]
        expected_all: vec![
            "sbpa7a6-", "sbpa7a5-",
            "sbpb7b6-", "sbpb7b5-",
            "sbnb8a6-", "sbnb8d7-",
            "sbpc6c5-", "sbpc6d5wp",
            "sbbc8d7-", "sbbc8e6-", "sbbc8f5-", "sbbc8g4-", "sbbc8h3-",
            "sbqd8c7-", "sbqd8b6-", "sbqd8a5-", "sbqd8d7-", "sbqd8e8-", "sbqd8e7-", "sbqd8d6-", "sbqd8d5wp",
            "sbnf6d5wp", "sbnf6d7-", "sbnf6e8-", "sbnf6h5-", "sbnf6g4-", "sbnf6e4wp",
            "sbrf8e8-",
            "sbkg8h8-",
            "sbbg7h6-", "sbbg7h8-",
            "sbpg6g5-",
            "sbph7h6-", "sbph7h5-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbpc6d5wp",
            "sbqd8d5wp",
            "sbnf6d5wp",
            "sbnf6e4wp",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbpc6d5wp",
            "sbqd8d5wp",
            "sbnf6d5wp",
            "sbnf6e4wp",
        ],
    })
}

#[test]
fn white_move_eleven() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/5np1/3p4/3NP3/1QN1BP2/PP4PP/R3KB1R w KQ - 0 4",
        #[rustfmt::skip]
        expected_all: vec![
            "cwq",
            "swra1b1-", "swra1c1-", "swra1c1-", "swra1d1-",
            "swpa2a3-", "swpa2a4-",
            "swqb3c2-", "swqb3d1-", "swqb3a3-", "swqb3a4-", "swqb3b4-", "swqb3b5-", "swqb3b6-", "swqb3b7bp", "swqb3c4-", "swqb3d5bp",
            "swnc3b1-", "swnc3d1-", "swnc3e2-", "swnc3d5bp", "swnc3a4-", "swnc3b5-",
            "swke1d1-", "swke1d2-", "swke1f2-", "swke1e2-",
            "swnd4c2-", "swnd4e2-", "swnd4f5-", "swnd4e6-", "swnd4c6-", "swnd4b5-",
            "swbf1e2-", "swbf1d3-", "swbf1c4-", "swbf1b5-", "swbf1a6-",
            "swbe3d2-", "swbe3c1-", "swbe3f4-", "swbe3g5-", "swbe3h6-", "swbe3f2-", "swbe3g1-",
            "swpe4e5-", "swpe4d5bp",
            "swpf3f4-",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-",
            "swph2h3-", "swph2h4-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swqb3b7bp",
            "swqb3d5bp",
            "swnc3d5bp",
            "swpe4d5bp",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swqb3b7bp",
            "swqb3d5bp",
            "swnc3d5bp",
            "swpe4d5bp",
        ],
    })
}

#[test]
fn black_move_eleven() -> Result<()> {
    execute_test(TestCase {
        board: "rnbq1rk1/pp3pbp/5np1/3P4/3N4/1QN1BP2/PP4PP/R3KB1R b KQ - 0 4",
        #[rustfmt::skip]
        expected_all: vec![
            "sbpa7a6-", "sbpa7a5-",
            "sbpb7b6-", "sbpb7b5-",
            "sbnb8a6-", "sbnb8d7-", "sbnb8c6-",
            "sbbc8d7-", "sbbc8e6-", "sbbc8f5-", "sbbc8g4-", "sbbc8h3-",
            "sbqd8c7-", "sbqd8b6-", "sbqd8a5-", "sbqd8d7-", "sbqd8e8-", "sbqd8e7-", "sbqd8d6-", "sbqd8d5wp",
            "sbnf6d5wp", "sbnf6d7-", "sbnf6e8-", "sbnf6h5-", "sbnf6g4-", "sbnf6e4-",
            "sbrf8e8-",
            "sbkg8h8-",
            "sbbg7h6-", "sbbg7h8-",
            "sbpg6g5-",
            "sbph7h6-", "sbph7h5-"
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "sbqd8d5wp",
            "sbnf6d5wp",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "sbqd8d5wp",
            "sbnf6d5wp",
        ],
    })
}

#[test]
fn white_move_twelve() -> Result<()> {
    execute_test(TestCase {
        board: "r1bq1rk1/pp3pbp/2n2np1/3P4/3N4/1QN1BP2/PP4PP/R3KB1R w KQ - 1 5",
        #[rustfmt::skip]
        expected_all: vec![
            "cwq",
            "swra1b1-", "swra1c1-", "swra1c1-", "swra1d1-",
            "swpa2a3-", "swpa2a4-",
            "swqb3c2-", "swqb3d1-", "swqb3a3-", "swqb3a4-", "swqb3b4-", "swqb3b5-", "swqb3b6-", "swqb3b7bp", "swqb3c4-",
            "swnc3b1-", "swnc3d1-", "swnc3e2-", "swnc3e4-", "swnc3a4-", "swnc3b5-",
            "swke1d1-", "swke1d2-", "swke1f2-", "swke1e2-",
            "swnd4c2-", "swnd4e2-", "swnd4f5-", "swnd4e6-", "swnd4c6bn", "swnd4b5-",
            "swbf1e2-", "swbf1d3-", "swbf1c4-", "swbf1b5-", "swbf1a6-",
            "swbe3d2-", "swbe3c1-", "swbe3f4-", "swbe3g5-", "swbe3h6-", "swbe3f2-", "swbe3g1-",
            "swpd5d6-", "swpd5c6bn",
            "swpf3f4-",
            "swpg2g3-", "swpg2g4-",
            "swrh1g1-",
            "swph2h3-", "swph2h4-",
        ],
        #[rustfmt::skip]
        expected_attacks: vec![
            "swqb3b7bp",
            "swnd4c6bn",
            "swpd5c6bn",
        ],
        #[rustfmt::skip]
        expected_attacks_checks: vec![
            "swqb3b7bp",
            "swnd4c6bn",
            "swpd5c6bn",
        ],
    })
}
