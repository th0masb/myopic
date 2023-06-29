use crate::search::InputTable;
use crate::{Evaluator, SearchOutcome, SearchParameters};
use InputTable::Blank;

#[test]
fn early_draw_bug_1() {
    search(
        "1.e4 e5 2.Nc3 Nf6 3.Nf3 Nc6 4.d4 exd4 5.Nxd4 Bc5 6.Nxc6 bxc6 7.Be2 d5 8.exd5 cxd5 \
         9.Bb5 Bd7 10.Bxd7 Qxd7 11.O-O O-O 12.Bg5 d4 13.Bxf6 gxf6 14.Ne2 Rab8 15.b3 Rbe8 16.c4 Qg4 \
         17.Ng3 Qxd1 18.Rfxd1 Re5 19.a3 a5 20.Kf1 Rb8 21.Rd3 Rbe8 22.f4 Re3 23.Rxe3 Rxe3 24.b4 axb4 \
         25.axb4 Bxb4 26.Ra8 Bf8 27.Rd8 c5 28.Kf2 Rc3 29.Nf5 Rc2 30.Kf3 Rc3 31.Ke4 Rxc4 \
         32.g4 Rc1 33.h4 Re1 34.Kd3 Rd1 35.Ke2 Rc1 36.Nh6 Kg7 37.Nf5 Kg8 38.h5 d3 39.Rxd3 Rc2 \
         40.Kd1 Rf2 41.h6 Rf1 42.Ke2 Rxf4 43.Rd8 Re4 44.Kf3 Re6 45.Kf4 Re1 46.Ra8 Rf1 47.Ke3 Re1 \
         48.Kd2 Re4 49.Rd8 Re6 50.Ra8 Re4 51.Kd1 Re5 52.Rb8 Re4 53.Ra8 Re5 54.Kc1 Re2 55.Rd8 Re5 \
         56.Kd2 Re6 57.Rc8 Re5 58.Kd3 Re1 59.Kc3 Re6 60.Kd2 Re4 61.Kc2 Re2 62.Kb3 Re6 63.Kc3 Re4 \
         64.Rb8 Re1 65.Kd2 Re5 66.Kc3 c4 67.Kc2 Re2 68.Kd1 Re4 69.Rc8 Re5 70.Kd2 Re4 71.Rd8 Re5 \
         72.Kc3 Re4 73.Ra8 Re2 74.Kxc4 Re1 75.Kd3 Re5 76.Ne3 Rb5 77.Ke2 Rb2 78.Kd3 Rb5 79.Ke2 Rb2 \
         80.Kf3 Rd2 81.Nf1 Rd1 82.Ne3 Rd7 83.Rc8 Rd6 84.Nf5 Re6 85.Nd4 Rd6 86.Nc6 Rd3 87.Ke4 Rd7 \
         88.Nb8 Re7 89.Kd3 Re1 90.Nc6 Rd1 91.Ke4 Re1 92.Kd3 Re6 93.Nb4 Rd6 94.Ke4 Re6 95.Kd3 Re5 \
         96.Kd4 Re1 97.Nd5 Re6 98.Nf4 Re1 99.Nd5",
        3
    );
}

#[test]
fn early_draw_bug_2() {
    search(
        "1. e4 e5 2. Nf3 Nf6 3. Nxe5 d6 4. Nf3 Nxe4 5. Be2 Be7 6. d3 Nf6 7. O-O O-O \
        8. d4 Nc6 9. d5 Nb4 10. Nc3 Bf5 11. a3 Nxc2 12. Nh4 Nxa1 13. Nxf5 Qd7 14. Bd3 Nc2 \
        15. Nxe7+ Qxe7 16. Qxc2 Qe5 17. Bb5 Nxd5 18. Bc4 Nxc3 19. bxc3 Qc5 20. Be2 g6 \
        21. Bb2 Rae8 22. Qd2 Re7 23. c4 Rfe8 24. Qh6 f6 25. Bd3 Re6 26. h3 R8e7 27. Qc1 Kg7 \
        28. Kh2 d5 29. cxd5 Qxd5 30. Bc4 Qd6+ 31. Kg1 Re4 32. Bb3 Re2 33. Bc3 Qc6 34. Rd1 Qb6 \
        35. Qf4 R2e4 36. Qf3 Re2 37. a4 Qc6 38. Rd5 h5 39. Bc4 Rc2 40. Qd3 Rc1+ 41. Kh2 h4 \
        42. f3 Qxa4 43. Bxf6+ Kf7 44. Rc5+ Rxc4 45. Rxc4 Rd7 46. Qf1 Qd1 47. Qxd1 Rxd1 48. Bg5 Rd7 \
        49. Ra4 Rd5 50. Rf4+ Rf5 51. Rxf5+ gxf5 52. Bxh4 Ke6 53. g4 fxg4 54. Bd8 gxf3 55. Bxc7 Kf5 \
        56. Kg1 Ke4 57. Kf2 b6 58. Bd8 b5 59. Ba5 Kf4 60. Bd2+ Ke4 61. h4 a6 62. Kg3 a5 \
        63. Bxa5 Ke3 64. h5 Ke2 65. Bb6 b4 66. h6 b3 67. h7 b2 68. h8=Q b1=Q 69. Qh2+ f2 \
        70. Qxf2+ Kd1 71. Qf1+ Kc2 72. Qc4+ Kd1 73. Qd4+ Kc1 74. Qf4+ Kd1 75. Qf1+ Kc2 \
        76. Qf5+ Kc1 77. Be3+ Kb2 78. Bd4+ Kc1 79. Be3+ Kb2 80. Bd4+ Kc1 81. Qf1+ Kc2 82. Qc4+ Kd1 \
        83. Qf1+ Kc2 84. Qc4+ Kd1 85. Be3 Qb2 86. Qd3+ Ke1 87. Qc4 Qa3 88. Kf3 Qb2 89. Qa6 Kd1 \
        90. Qd3+ Ke1 91. Qa6 Kd1 92. Qa4+ Ke1 93. Qh4+ Kd1 94. Qa4+ Ke1 95. Qh4+ Kd1 96. Qc4 Qf6+ \
        97. Kg2 Qg6+ 98. Kh3 Qb1 99. Qf1+ Kc2 100. Qc4+ Kd1 101. Qf1+ Kc2 102. Qf5+ Kb2 \
        103. Bd4+ Kc1 104. Qf4+ Kd1 105. Qf3+ Kc1 106. Qf1+ Kc2 107. Qf5+ Kc1 108. Qf1+ Kc2 \
        109. Qg2+ Kd1 110. Qf3+ Kc1 111. Qf4+ Kd1 112. Qg4+ Kc1 113. Qg1+ Kc2 114. Qg2+ Kd1 \
        115. Qh1+ Kc2 116. Qe4+ Kc1 117. Qe1+ Kc2 118. Qe4+ Kc1 119. Qe1+",
        3,
    );
}

fn search(pgn: &str, depth: usize) -> SearchOutcome {
    let mut board = Evaluator::default();
    board.play_pgn(pgn).expect(format!("Invalid {}", pgn).as_str());
    crate::search(board, SearchParameters { terminator: depth, table: Blank(10_000) })
        .map_err(|e| panic!("{}", e))
        .unwrap()
}
