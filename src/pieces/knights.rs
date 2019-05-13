use crate::base::bitboard::BitBoard;
use crate::base::dir::*;
use crate::base::square::{constants::SQUARES, Square};

use crate::base::Side;

pub fn control(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    CONTROL[loc.i as usize]
}

pub fn white_moves(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    CONTROL[loc.i as usize] - whites
}

pub fn black_moves(loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
    CONTROL[loc.i as usize] - blacks
}

const CONTROL: [BitBoard; 64] = [
    BitBoard(132096),
    BitBoard(329728),
    BitBoard(659712),
    BitBoard(1319424),
    BitBoard(2638848),
    BitBoard(5277696),
    BitBoard(10489856),
    BitBoard(4202496),
    BitBoard(33816580),
    BitBoard(84410376),
    BitBoard(168886289),
    BitBoard(337772578),
    BitBoard(675545156),
    BitBoard(1351090312),
    BitBoard(2685403152),
    BitBoard(1075839008),
    BitBoard(8657044482),
    BitBoard(21609056261),
    BitBoard(43234889994),
    BitBoard(86469779988),
    BitBoard(172939559976),
    BitBoard(345879119952),
    BitBoard(687463207072),
    BitBoard(275414786112),
    BitBoard(2216203387392),
    BitBoard(5531918402816),
    BitBoard(11068131838464),
    BitBoard(22136263676928),
    BitBoard(44272527353856),
    BitBoard(88545054707712),
    BitBoard(175990581010432),
    BitBoard(70506185244672),
    BitBoard(567348067172352),
    BitBoard(1416171111120896),
    BitBoard(2833441750646784),
    BitBoard(5666883501293568),
    BitBoard(11333767002587136),
    BitBoard(22667534005174272),
    BitBoard(45053588738670592),
    BitBoard(18049583422636032),
    BitBoard(145241105196122112),
    BitBoard(362539804446949376),
    BitBoard(725361088165576704),
    BitBoard(1450722176331153408),
    BitBoard(2901444352662306816),
    BitBoard(5802888705324613632),
    BitBoard(11533718717099671552),
    BitBoard(4620693356194824192),
    BitBoard(288234782788157440),
    BitBoard(576469569871282176),
    BitBoard(1224997833292120064),
    BitBoard(2449995666584240128),
    BitBoard(4899991333168480256),
    BitBoard(9799982666336960512),
    BitBoard(1152939783987658752),
    BitBoard(2305878468463689728),
    BitBoard(1128098930098176),
    BitBoard(2257297371824128),
    BitBoard(4796069720358912),
    BitBoard(9592139440717824),
    BitBoard(19184278881435648),
    BitBoard(38368557762871296),
    BitBoard(4679521487814656),
    BitBoard(9077567998918656),
];

//fn compute_empty_board_control() -> Vec<BitBoard> {
//    let search_dirs = vec![NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW];
//    SQUARES
//        .iter()
//        .map(|&sq| sq.search_one(&search_dirs))
//        .collect()
//}

#[cfg(test)]
mod white_test {
    use crate::base::square::constants::*;
    use crate::pieces::WN;

    use super::*;

    #[test]
    fn test_control() {
        let zero = BitBoard::EMPTY;
        assert_eq!(
            D1 | C2 | C4 | D5 | F5 | G4 | G2 | F1,
            WN.control(E3, zero, zero)
        );
        assert_eq!(A4 | C4 | D3 | D1, WN.control(B2, zero, zero));
    }

    #[test]
    fn test_moves() {
        assert_eq!(A4 | C4 | D3, WN.moves(B2, D1 | B1, A4 | D7));
    }

    //    #[test]
    //    fn test_attacks() {
    //        let wn = WhiteKnight;
    //        assert_eq!(A4.lift(), wn.attacks(B2, D1 | B1, A4 | D7));
    //    }
}

#[cfg(test)]
mod black_test {
    use crate::base::square::constants::*;
    use crate::pieces::BN;

    use super::*;

    #[test]
    fn test_control() {
        let zero = BitBoard::EMPTY;
        assert_eq!(
            D1 | C2 | C4 | D5 | F5 | G4 | G2 | F1,
            BN.control(E3, zero, zero)
        );
        assert_eq!(A4 | C4 | D3 | D1, BN.control(B2, zero, zero));
    }

    #[test]
    fn test_moves() {
        assert_eq!(C4 | D3 | D1, BN.moves(B2, D1 | B1, A4 | D7));
    }

    //    #[test]
    //    fn test_attacks() {
    //        let bn = BlackKnight;
    //        assert_eq!(D1.lift(), bn.attacks(B2, D1 | B1, A4 | D7));
    //    }
}
