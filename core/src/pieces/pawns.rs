use crate::bitboard::BitBoard;
use crate::Side;
use crate::Square;

pub fn white_control(loc: Square, _whites: BitBoard, _blacks: BitBoard) -> BitBoard {
    WHITE_CONTROL[loc as usize]
}

pub fn white_moves(loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
    let all = white | black;
    let mut result = ((loc - BitBoard::RANKS[7]) << 8) - all;
    if Side::White.pawn_first_rank().contains(loc) && !result.is_empty() {
        result = result | ((loc.lift() << 16) - all)
    }
    result | (white_control(loc, white, black) & black)
}

pub fn black_control(loc: Square, _whites: BitBoard, _blacks: BitBoard) -> BitBoard {
    BLACK_CONTROL[loc as usize]
}

pub fn black_moves(loc: Square, white: BitBoard, black: BitBoard) -> BitBoard {
    let all = white | black;
    let mut result = ((loc - BitBoard::RANKS[0]) >> 8) - all;
    if Side::Black.pawn_first_rank().contains(loc) && !result.is_empty() {
        result = result | ((loc.lift() >> 16) - all)
    }
    result | (black_control(loc, white, black) & white)
}

///  Control sets for all squares for the white pawn.
const WHITE_CONTROL: [BitBoard; 64] = [
    BitBoard(512),
    BitBoard(1280),
    BitBoard(2560),
    BitBoard(5120),
    BitBoard(10240),
    BitBoard(20480),
    BitBoard(40960),
    BitBoard(16384),
    BitBoard(131072),
    BitBoard(327680),
    BitBoard(655360),
    BitBoard(1310720),
    BitBoard(2621440),
    BitBoard(5242880),
    BitBoard(10485760),
    BitBoard(4194304),
    BitBoard(33554432),
    BitBoard(83886080),
    BitBoard(167772160),
    BitBoard(335544320),
    BitBoard(671088640),
    BitBoard(1342177280),
    BitBoard(2684354560),
    BitBoard(1073741824),
    BitBoard(8589934592),
    BitBoard(21474836480),
    BitBoard(42949672960),
    BitBoard(85899345920),
    BitBoard(171798691840),
    BitBoard(343597383680),
    BitBoard(687194767360),
    BitBoard(274877906944),
    BitBoard(2199023255552),
    BitBoard(5497558138880),
    BitBoard(10995116277760),
    BitBoard(21990232555520),
    BitBoard(43980465111040),
    BitBoard(87960930222080),
    BitBoard(175921860444160),
    BitBoard(70368744177664),
    BitBoard(562949953421312),
    BitBoard(1407374883553280),
    BitBoard(2814749767106560),
    BitBoard(5629499534213120),
    BitBoard(11258999068426240),
    BitBoard(22517998136852480),
    BitBoard(45035996273704960),
    BitBoard(18014398509481984),
    BitBoard(144115188075855872),
    BitBoard(360287970189639680),
    BitBoard(720575940379279360),
    BitBoard(1441151880758558720),
    BitBoard(2882303761517117440),
    BitBoard(5764607523034234880),
    BitBoard(11529215046068469760),
    BitBoard(4611686018427387904),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
];

///  Control sets for all squares for the black pawn.
const BLACK_CONTROL: [BitBoard; 64] = [
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(2),
    BitBoard(5),
    BitBoard(10),
    BitBoard(20),
    BitBoard(40),
    BitBoard(80),
    BitBoard(160),
    BitBoard(64),
    BitBoard(512),
    BitBoard(1280),
    BitBoard(2560),
    BitBoard(5120),
    BitBoard(10240),
    BitBoard(20480),
    BitBoard(40960),
    BitBoard(16384),
    BitBoard(131072),
    BitBoard(327680),
    BitBoard(655360),
    BitBoard(1310720),
    BitBoard(2621440),
    BitBoard(5242880),
    BitBoard(10485760),
    BitBoard(4194304),
    BitBoard(33554432),
    BitBoard(83886080),
    BitBoard(167772160),
    BitBoard(335544320),
    BitBoard(671088640),
    BitBoard(1342177280),
    BitBoard(2684354560),
    BitBoard(1073741824),
    BitBoard(8589934592),
    BitBoard(21474836480),
    BitBoard(42949672960),
    BitBoard(85899345920),
    BitBoard(171798691840),
    BitBoard(343597383680),
    BitBoard(687194767360),
    BitBoard(274877906944),
    BitBoard(2199023255552),
    BitBoard(5497558138880),
    BitBoard(10995116277760),
    BitBoard(21990232555520),
    BitBoard(43980465111040),
    BitBoard(87960930222080),
    BitBoard(175921860444160),
    BitBoard(70368744177664),
    BitBoard(562949953421312),
    BitBoard(1407374883553280),
    BitBoard(2814749767106560),
    BitBoard(5629499534213120),
    BitBoard(11258999068426240),
    BitBoard(22517998136852480),
    BitBoard(45035996273704960),
    BitBoard(18014398509481984),
];

//fn compute_all_empty_board_control(side: Side) -> Vec<BitBoard> {
//    SQUARES
//        .iter()
//        .map(|&sq| compute_empty_board_control(side, sq))
//        .collect()
//}
//
//fn compute_empty_board_control(side: Side, loc: Square) -> BitBoard {
//    let (x, left, right) = (loc.lift(), FILES[7], FILES[0]);
//    match side {
//        White => ((x - left) << 9u8) | ((x - right) << 7u8),
//        Black => ((x - left) >> 7u8) | ((x - right) >> 9u8),
//    }
//}
#[cfg(test)]
mod black_test {
    use crate::pieces::Piece;
    use crate::Square::*;

    use super::*;

    #[test]
    fn test_control() {
        assert_eq!(D2 | F2, Piece::BP.control(E3, A1 | B6, D8 | D4));
        assert_eq!(F7 | D7, Piece::BP.control(E8, A1 | B6, D8 | D4));
        assert_eq!(B2.lift(), Piece::BP.control(A3, A4 | C5, F4 | H8));
        assert_eq!(G2.lift(), Piece::BP.control(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moves() {
        assert_eq!(D1.lift(), Piece::BP.moves(D2, E2 | D5, G1 | F7));
        assert_eq!(G6 | G5, Piece::BP.moves(G7, A1 | F5, H6 | B3));
        assert_eq!(BitBoard::EMPTY, Piece::BP.moves(G7, A1 | G6, H6 | B3));
        assert_eq!(G6.lift(), Piece::BP.moves(G7, A1 | G8, G5 | B3));
    }
}

#[cfg(test)]
mod white_test {
    use crate::pieces::Piece;
    use crate::Square::*;

    use super::*;

    #[test]
    fn test_control() {
        assert_eq!(D4 | F4, Piece::WP.control(E3, A1 | B6, D8 | D4));
        assert_eq!(BitBoard::EMPTY, Piece::WP.control(E8, A1 | B6, D8 | D4));
        assert_eq!(B4.lift(), Piece::WP.control(A3, A4 | C5, F4 | H8));
        assert_eq!(G4.lift(), Piece::WP.control(H3, A4 | C5, F4 | H8));
    }

    #[test]
    fn test_moves() {
        assert_eq!(D3 | D4 | E3, Piece::WP.moves(D2, E2 | D5, G1 | F7 | E3));
        assert_eq!(D3.lift(), Piece::WP.moves(D2, D4 | G6, A2 | D7));
        assert_eq!(BitBoard::EMPTY, Piece::WP.moves(D2, D3 | A1, B5 | D5));
        assert_eq!(G7.lift(), Piece::WP.moves(G6, A1 | F5, H6 | B3));
        assert_eq!(BitBoard::EMPTY, Piece::WP.moves(G6, A1 | G7, H6 | B3));
        assert_eq!(BitBoard::EMPTY, Piece::WP.moves(G6, A1 | A5, H6 | G7));
        assert_eq!(BitBoard::EMPTY, Piece::WP.moves(A8, D3 | H7, F4 | C3));
    }
}
