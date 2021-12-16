use myopic_core::*;

/// TODO Could have adjacent files in a constant array
pub(super) fn squares(active: Side, enpassant_target: Square) -> BitBoard {
    ADJACENTS[enpassant_target.file_index() as usize] & active.reflect().pawn_third_rank()
}

//pub fn compute_adjacent_files() -> Vec<BitBoard> {
//    let mut dest: Vec<BitBoard> = Vec::new();
//    for i in 0..8 {
//        if i == 0 {
//            dest.push(FILES[1]);
//        } else if i == 7 {
//            dest.push(FILES[6]);
//        } else {
//            dest.push(FILES[i + 1] | FILES[i - 1]);
//        }
//    }
//    dest
//}

const ADJACENTS: [BitBoard; 8] = [
    BitBoard(144680345676153346),
    BitBoard(361700864190383365),
    BitBoard(723401728380766730),
    BitBoard(1446803456761533460),
    BitBoard(2893606913523066920),
    BitBoard(5787213827046133840),
    BitBoard(11574427654092267680),
    BitBoard(4629771061636907072),
];

#[cfg(test)]
mod test_enpassant_source_squares {
    use myopic_core::{constants::*, Side, Square};

    use super::squares;

    #[test]
    fn test() {
        assert_eq!(H4 | F4, squares(Side::Black, Square::G3));
        assert_eq!(G4, squares(Side::Black, Square::H3));
        assert_eq!(B4, squares(Side::Black, Square::A3));
        assert_eq!(H5 | F5, squares(Side::White, Square::G6));
        assert_eq!(G5, squares(Side::White, Square::H6));
        assert_eq!(B5, squares(Side::White, Square::A6));
    }
}
