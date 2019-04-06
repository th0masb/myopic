use super::{BlackKing, Piece, WhiteKing};
use crate::bitboard::BitBoard;
use crate::dir::*;
use crate::square::{constants::SQUARES, Square};

/// Piece trait implementation for the white knight struct. It simply queries
/// a static vector of moves for each square.
impl Piece for WhiteKing {
    fn controlset(self, location: Square, _white: BitBoard, _black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize]
    }

    fn moveset(self, location: Square, white: BitBoard, _black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] - white
    }

    fn attackset(self, location: Square, _white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] & black
    }
}

/// Piece trait implementation for the black knight struct. It simply queries
/// a static vector of moves for each square.
impl Piece for BlackKing {
    fn controlset(self, location: Square, _white: BitBoard, _black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize]
    }

    fn moveset(self, location: Square, _white: BitBoard, black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] - black
    }

    fn attackset(self, location: Square, white: BitBoard, _black: BitBoard) -> BitBoard {
        CONTROL[location.i as usize] & white
    }
}

const CONTROL: [BitBoard; 64] = [
    BitBoard(770),
    BitBoard(1797),
    BitBoard(3594),
    BitBoard(7188),
    BitBoard(14376),
    BitBoard(28752),
    BitBoard(57504),
    BitBoard(49216),
    BitBoard(197123),
    BitBoard(460039),
    BitBoard(920078),
    BitBoard(1840156),
    BitBoard(3680312),
    BitBoard(7360624),
    BitBoard(14721248),
    BitBoard(12599488),
    BitBoard(50463488),
    BitBoard(117769984),
    BitBoard(235539968),
    BitBoard(471079936),
    BitBoard(942159872),
    BitBoard(1884319744),
    BitBoard(3768639488),
    BitBoard(3225468928),
    BitBoard(12918652928),
    BitBoard(30149115904),
    BitBoard(60298231808),
    BitBoard(120596463616),
    BitBoard(241192927232),
    BitBoard(482385854464),
    BitBoard(964771708928),
    BitBoard(825720045568),
    BitBoard(3307175149568),
    BitBoard(7718173671424),
    BitBoard(15436347342848),
    BitBoard(30872694685696),
    BitBoard(61745389371392),
    BitBoard(123490778742784),
    BitBoard(246981557485568),
    BitBoard(211384331665408),
    BitBoard(846636838289408),
    BitBoard(1975852459884544),
    BitBoard(3951704919769088),
    BitBoard(7903409839538176),
    BitBoard(15806819679076352),
    BitBoard(31613639358152704),
    BitBoard(63227278716305408),
    BitBoard(54114388906344448),
    BitBoard(216739030602088448),
    BitBoard(505818229730443264),
    BitBoard(1011636459460886528),
    BitBoard(2023272918921773056),
    BitBoard(4046545837843546112),
    BitBoard(8093091675687092224),
    BitBoard(16186183351374184448),
    BitBoard(13853283560024178688),
    BitBoard(144959613005987840),
    BitBoard(362258295026614272),
    BitBoard(724516590053228544),
    BitBoard(1449033180106457088),
    BitBoard(2898066360212914176),
    BitBoard(5796132720425828352),
    BitBoard(11592265440851656704),
    BitBoard(4665729213955833856),
];

// Implementation and tests.
//fn compute_empty_board_control() -> Vec<BitBoard> {
//    let search_dirs = vec![N, E, S, W, NE, SE, SW, NW];
//    SQUARES
//        .iter()
//        .map(|&sq| sq.search_one(&search_dirs))
//        .collect()
//}

#[cfg(test)]
mod white_test {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_control() {
        let (wk, zero) = (WhiteKing, BitBoard::EMPTY);
        assert_eq!(
            D2 | E2 | F2 | F3 | F4 | E4 | D4 | D3,
            wk.controlset(E3, zero, zero)
        );
        assert_eq!(B1 | B2 | C2 | D2 | D1, wk.controlset(C1, zero, zero));
    }

    #[test]
    fn test_moveset() {
        let wk = WhiteKing;
        assert_eq!(B2 | C2 | D2 | D1, wk.moveset(C1, B1.lift(), C2.lift()));
    }

    #[test]
    fn test_attackset() {
        let wk = WhiteKing;
        assert_eq!(C2.lift(), wk.attackset(C1, B1.lift(), C2.lift()));
    }
}

#[cfg(test)]
mod black_test {
    use super::*;
    use crate::square::constants::*;

    #[test]
    fn test_control() {
        let (bk, zero) = (BlackKing, BitBoard::EMPTY);
        assert_eq!(
            D2 | E2 | F2 | F3 | F4 | E4 | D4 | D3,
            bk.controlset(E3, zero, zero)
        );
        assert_eq!(B1 | B2 | C2 | D2 | D1, bk.controlset(C1, zero, zero));
    }

    #[test]
    fn test_moveset() {
        let bk = BlackKing;
        assert_eq!(B2 | B1 | D2 | D1, bk.moveset(C1, B1.lift(), C2.lift()));
    }

    #[test]
    fn test_attackset() {
        let bk = BlackKing;
        assert_eq!(B1.lift(), bk.attackset(C1, B1.lift(), C2.lift()));
    }
}
