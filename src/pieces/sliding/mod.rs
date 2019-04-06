use std::num::Wrapping;

use crate::base::bitboard::BitBoard;
use crate::base::dir::Dir;
use crate::base::dir::{E, N, S, W};
use crate::base::dir::{NE, NW, SE, SW};
use crate::base::square::constants::SQUARES;
use crate::base::square::Square;

pub mod bishops;
pub mod queens;
pub mod rooks;

/// API for computing the magic index for a bishop positioned at a given
/// location with the given piece arrangement on the board.
fn compute_bishop_index(location: Square, pieces: BitBoard) -> usize {
    let i = location.i as usize;
    compute_magic_index(
        pieces.0 & BISHOP_MASKS[i],
        BISHOP_MAGICS[i],
        BISHOP_SHIFTS[i],
    )
}

/// API for computing the magic index for a rook positioned at a given
/// location with the given piece arrangement on the board.
fn compute_rook_index(location: Square, pieces: BitBoard) -> usize {
    let i = location.i as usize;
    compute_magic_index(pieces.0 & ROOK_MASKS[i], ROOK_MAGICS[i], ROOK_SHIFTS[i])
}

/// Applies the magic index mapping operation by multiplying the occupancy
/// and magic number together (allowing overflow) and then performing a right
/// shift on the result.
fn compute_magic_index(occupancy: u64, magic: u64, shift: usize) -> usize {
    let (o, m) = (Wrapping(occupancy), Wrapping(magic));
    ((o * m).0 >> shift) as usize
}

// Implementation details and related tests.
/// Computes a vector containing all the directions a bishop can move in.
fn bishop_dirs() -> Vec<Dir> {
    vec![NE, SE, SW, NW]
}

/// Computes a vector containing all the directions a rook can move in.
fn rook_dirs() -> Vec<Dir> {
    vec![N, E, S, W]
}

/// Computes a vector containing the occupancy masks for each base.square. The
/// occupancy mask at a base.square for some direction set is defined to be the
/// locations a piece could move to on an empty board excluding the last
/// base.square in each of the direction 'rays'.
fn compute_masks(dirs: &Vec<Dir>) -> Vec<u64> {
    SQUARES
        .iter()
        .map(|&sq| {
            dirs.iter()
                .map(|&dir| search_remove_last(sq, dir))
                .collect()
        })
        .map(|bb: BitBoard| bb.0)
        .collect()
}

/// Computes the set of squares in a given direction from some source base.square
/// with the furthest away excluded.
fn search_remove_last(loc: Square, dir: Dir) -> BitBoard {
    let mut res = loc.search_vec(dir);
    if res.len() > 0 {
        res.remove(res.len() - 1);
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod mask_tests {
    use crate::base::square::constants::*;

    use super::*;

    #[test]
    fn test_bishop_masks() {
        let bmasks = compute_masks(&bishop_dirs());
        assert_eq!(C7 | C5 | D4 | E3 | F2, BitBoard(bmasks[B6.i as usize]));
        let rmasks = compute_masks(&rook_dirs());
        assert_eq!(
            A2 | A3 | A5 | A6 | A7 | B4 | C4 | D4 | E4 | F4 | G4,
            BitBoard(rmasks[A4.i as usize])
        );
    }
}

/// Computes the magic bitshift values for all squares which is defined to
/// be the 1 count of the corresponding occupancy mask subtracted from 64.
fn compute_shifts(dirs: &Vec<Dir>) -> Vec<usize> {
    let f = |x: u64| 64 - BitBoard(x).size();
    compute_masks(dirs).into_iter().map(f).collect()
}

/// Computes the powerset of some set of squares with the resulting elements
/// of the powerset represented as bitboards.
fn compute_powerset(squares: &Vec<Square>) -> Vec<BitBoard> {
    if squares.is_empty() {
        vec![BitBoard::EMPTY]
    } else {
        let (head, rest) = (squares[0], &squares[1..].to_vec());
        let recursive = compute_powerset(rest);
        let mut res = vec![];
        for set in recursive {
            res.push(set);
            res.push(set | head);
        }
        res
    }
}

#[cfg(test)]
mod powerset_test {
    use std::collections::HashSet;

    use crate::base::square::constants::*;

    use super::*;

    #[test]
    fn test() {
        let empty = vec![BitBoard::EMPTY];
        assert_eq!(empty, compute_powerset(&vec![]));
        let non_empty = vec![A1, F3, H5];
        let mut expected = HashSet::new();
        expected.insert(BitBoard::EMPTY);
        expected.insert(A1.lift());
        expected.insert(F3.lift());
        expected.insert(H5.lift());
        expected.insert(A1 | F3);
        expected.insert(A1 | H5);
        expected.insert(F3 | H5);
        expected.insert(A1 | F3 | H5);
        let actual: HashSet<_> = compute_powerset(&non_empty).into_iter().collect();
        assert_eq!(expected, actual);
    }
}

/// Computes the control set for a piece assumed to be located at a given
/// source base.square and which is permitted to move in a specified set of
/// directions.
fn compute_control(loc: Square, occ: BitBoard, dirs: &Vec<Dir>) -> BitBoard {
    let mut res = 0u64;
    for &dir in dirs {
        for sq in loc.search_vec(dir) {
            res |= 1u64 << sq.i;
            if !(occ & sq).is_empty() {
                break;
            }
        }
    }
    BitBoard(res)
}

#[cfg(test)]
mod control_tests {
    use crate::base::square::constants::*;

    use super::*;

    #[test]
    fn test_sliding_control() {
        // Could split this up to test rook and bishop separately.
        let loc = D4;
        let whites = D1 | F4 | D6 | G7 | H8;
        let blacks = B2 | B4 | E3 | A7;
        let dirs = vec![N, NE, E, SE, S, SW, W, NW];
        let expected_control =
            D5 | D6 | E5 | F6 | G7 | E4 | F4 | E3 | D3 | D2 | D1 | C3 | B2 | C4 | B4 | C5 | B6 | A7;
        assert_eq!(
            expected_control,
            compute_control(loc, whites | blacks, &dirs)
        );
    }
}

/// Commented out the magic number generation code to get rid of unused
/// code compiler warnings.

///// Use brute force trial end error to compute a valid set of magic numbers.
///// A magic number for a base.square is considered to be valid if it causes no
///// conflicting collisions among the occupancy variations, that is no two
///// variations which map to the same index but have different control sets.
//fn compute_magic_numbers(dirs: &Vec<Dir>) -> Vec<u64> {
//    let (masks, shifts) = (compute_masks(&dirs), compute_shifts(&dirs));
//    let mut magics: Vec<u64> = Vec::with_capacity(64);
//    for (&sq, &mask, &shift) in izip!(SQUARES.iter(), &masks, &shifts) {
//        let occ_vars = compute_powerset(&BitBoard(mask).into_iter().collect());
//        let control: Vec<_> = occ_vars
//            .iter()
//            .map(|&ov| compute_control(sq, ov, &dirs).0)
//            .collect();
//        let mut indices: Vec<_> = repeat(064).take(occ_vars.len()).collect();
//        let mut moves: Vec<_> = indices.clone();
//        let upper = 100000000;
//        'outer: for i in 1..=upper {
//            let magic = gen_magic_candidate();
//            for (&occ_var, &control) in occ_vars.iter().zip(control.iter()) {
//                let index = compute_magic_index(occ_var.0, magic, shift);
//                if indices[index] == i {
//                    if moves[index] != control {
//                        continue 'outer; // The magic candidate has failed
//                    }
//                } else {
//                    indices[index] = i;
//                    moves[index] = control;
//                }
//            }
//            if i == upper {
//                panic!("Failed to generate number!")
//            } else {
//                magics.push(magic);
//                break;
//            }
//        }
//    }
//    magics
//}

///// Generates a random unsigned long with a sparse set of 1 bits.
//fn gen_magic_candidate() -> u64 {
//    rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
//}

/// Constants forming the constituent parts of the 'magic base.bitboard' mapping
/// technique.
///
/// The shifts are used to reduce the result of the magic multiplication
/// to an index.
///
/// The masks are combined with the locations of pieces on the board via
/// bitwise 'and' to create an 'occupancy variation'.
///
/// The magic numbers are combined with the masked occupancy variation via
/// overflowing multiplication.

const BISHOP_SHIFTS: [usize; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 58,
];

const ROOK_SHIFTS: [usize; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
];

const BISHOP_MASKS: [u64; 64] = [
    18049651735527936,
    70506452091904,
    275415828992,
    1075975168,
    38021120,
    8657588224,
    2216338399232,
    567382630219776,
    9024825867763712,
    18049651735527424,
    70506452221952,
    275449643008,
    9733406720,
    2216342585344,
    567382630203392,
    1134765260406784,
    4512412933816832,
    9024825867633664,
    18049651768822272,
    70515108615168,
    2491752130560,
    567383701868544,
    1134765256220672,
    2269530512441344,
    2256206450263040,
    4512412900526080,
    9024834391117824,
    18051867805491712,
    637888545440768,
    1135039602493440,
    2269529440784384,
    4539058881568768,
    1128098963916800,
    2256197927833600,
    4514594912477184,
    9592139778506752,
    19184279556981248,
    2339762086609920,
    4538784537380864,
    9077569074761728,
    562958610993152,
    1125917221986304,
    2814792987328512,
    5629586008178688,
    11259172008099840,
    22518341868716544,
    9007336962655232,
    18014673925310464,
    2216338399232,
    4432676798464,
    11064376819712,
    22137335185408,
    44272556441600,
    87995357200384,
    35253226045952,
    70506452091904,
    567382630219776,
    1134765260406784,
    2832480465846272,
    5667157807464448,
    11333774449049600,
    22526811443298304,
    9024825867763712,
    18049651735527936,
];

const ROOK_MASKS: [u64; 64] = [
    282578800148862,
    565157600297596,
    1130315200595066,
    2260630401190006,
    4521260802379886,
    9042521604759646,
    18085043209519166,
    36170086419038334,
    282578800180736,
    565157600328704,
    1130315200625152,
    2260630401218048,
    4521260802403840,
    9042521604775424,
    18085043209518592,
    36170086419037696,
    282578808340736,
    565157608292864,
    1130315208328192,
    2260630408398848,
    4521260808540160,
    9042521608822784,
    18085043209388032,
    36170086418907136,
    282580897300736,
    565159647117824,
    1130317180306432,
    2260632246683648,
    4521262379438080,
    9042522644946944,
    18085043175964672,
    36170086385483776,
    283115671060736,
    565681586307584,
    1130822006735872,
    2261102847592448,
    4521664529305600,
    9042787892731904,
    18085034619584512,
    36170077829103616,
    420017753620736,
    699298018886144,
    1260057572672512,
    2381576680245248,
    4624614895390720,
    9110691325681664,
    18082844186263552,
    36167887395782656,
    35466950888980736,
    34905104758997504,
    34344362452452352,
    33222877839362048,
    30979908613181440,
    26493970160820224,
    17522093256097792,
    35607136465616896,
    9079539427579068672,
    8935706818303361536,
    8792156787827803136,
    8505056726876686336,
    7930856604974452736,
    6782456361169985536,
    4485655873561051136,
    9115426935197958144,
];

const BISHOP_MAGICS: [u64; 64] = [
    7728262862096860416,
    220677507089776865,
    3386521924206592,
    4613942907922024450,
    1270580313697035264,
    5075362929222160,
    2311477087219744816,
    6896689110597697,
    2450292586280190512,
    36172034782470656,
    576619120637050894,
    4508071795621952,
    9817849395557959680,
    9229008176677126208,
    11547229745261322272,
    1152923171205222400,
    13988323674017630208,
    1234285382376461056,
    6210080462669056,
    5629672407703560,
    145241133417506307,
    6918091990483665994,
    1970325929074704,
    6922173374756194304,
    14125545729882657284,
    1174350986501685512,
    9081967124218370,
    2306407196252012896,
    9552139207841947908,
    9241670113680183318,
    4756279494363385860,
    4684871853554139648,
    9587760252870688,
    1304226949500930,
    1152958892603873280,
    577058920992931920,
    563516893429792,
    1157426204819718340,
    2308519364401562626,
    1128649366044928,
    1162073857658462208,
    2324421470224146592,
    4684311099581958144,
    720611408294987776,
    36072850565627968,
    2326109791685970064,
    4612251176027554816,
    92332601609683008,
    2577289649332228,
    4612390358990487808,
    72066666150756352,
    3498065461877670915,
    2252074828046336,
    4434830885632,
    577626269026615688,
    4693885679539683457,
    1153485623899718656,
    3692970803156095250,
    81351784116257792,
    35328866124808,
    72057594323272192,
    306262504353374726,
    9140495749376,
    9185389143196176,
];

const ROOK_MAGICS: [u64; 64] = [
    2341874280142213664,
    594475425959256064,
    108095255871488641,
    144119629074153488,
    144120144474409072,
    4755959553358037504,
    432346148410294784,
    36029896534859904,
    144678277649285376,
    594616025744410508,
    581105226859151496,
    1315332604825702400,
    1153484540627394704,
    146930521992073260,
    615304578832794632,
    1154891833739067522,
    9225800308394049536,
    9226829176558854144,
    585469326216153089,
    10698248273543712,
    2252350844600392,
    5846244066938863668,
    2450099484550496768,
    288232575192825924,
    666533073417961472,
    11556271830356460160,
    175924008976512,
    1443685572161831944,
    18296204201838592,
    18577666299921416,
    18159122252034,
    1128725995388996,
    9242094795738841400,
    576531123237036037,
    11294189891358721,
    108103985465919488,
    5188709791718966432,
    1153484471748534912,
    230931868684289,
    6971572790272983105,
    431696826957828,
    1157495473515298944,
    2451119294454497344,
    4688820058456129568,
    38843564507660305,
    81346285449838598,
    162982876361785347,
    72063111003242508,
    144255930163282176,
    211108415671424,
    581567161361797632,
    35789965623552,
    1224996708144972032,
    50102722221052416,
    1441228851271173120,
    9386068972664819200,
    72571307467906,
    288218347995265,
    3458870070174812241,
    2306973444874447105,
    145804072854750210,
    93731193081563201,
    8800401621508,
    857903159050370,
];
