use crate::base::bitboard::BitBoard;
use crate::base::hash;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Positions {
    boards: [BitBoard; 12],
    hash: u64,
    whites: BitBoard,
    blacks: BitBoard,
}

impl Reflectable for Positions {
    fn reflect(&self) -> Self {
        let mut new_boards = [BitBoard::EMPTY; 12];
        for i in 0..12 {
            new_boards[i] = self.boards[(i + 6) % 12].reflect();
        }
        Positions {
            boards: new_boards,
            hash: hash_boards(&new_boards),
            whites: compute_whites(&new_boards),
            blacks: compute_blacks(&new_boards),
        }
    }
}

fn hash_boards(boards: &[BitBoard]) -> u64 {
    assert_eq!(12, boards.len());
    boards
        .iter()
        .zip(Piece::iter())
        .flat_map(|(&b, p)| b.into_iter().map(move |sq| hash::piece_feature(p, sq)))
        .fold(0u64, |a, b| a ^ b)
}

fn compute_whites(boards: &[BitBoard]) -> BitBoard {
    boards.iter().take(6).fold(BitBoard::EMPTY, |a, &b| a | b)
}

fn compute_blacks(boards: &[BitBoard]) -> BitBoard {
    boards.iter().skip(6).fold(BitBoard::EMPTY, |a, &b| a | b)
}

fn convert_rank(fen_rank: String) -> Vec<Option<Piece>> {
    let mut dest: Vec<Option<Piece>> = Vec::new();
    for character in fen_rank.chars() {
        if character.is_numeric() {
            let space = character.to_string().parse::<usize>().unwrap();
            dest.extend(itertools::repeat_n(None, space));
        } else {
            dest.extend(&[Some(match character {
                'P' => Piece::WP,
                'N' => Piece::WN,
                'B' => Piece::WB,
                'R' => Piece::WR,
                'Q' => Piece::WQ,
                'K' => Piece::WK,
                'p' => Piece::BP,
                'n' => Piece::BN,
                'b' => Piece::BB,
                'r' => Piece::BR,
                'q' => Piece::BQ,
                'k' => Piece::BK,
                _ => panic!(),
            })]);
        }
    }
    dest
}

impl Positions {
    pub fn from_fen(ranks: Vec<String>) -> Positions {
        assert_eq!(8, ranks.len());
        let mut board =
            ranks.into_iter().flat_map(|r| convert_rank(r).into_iter()).collect::<Vec<_>>();
        assert_eq!(64, board.len());
        board.reverse();
        let mut bitboards = [BitBoard::EMPTY; 12];
        for (i, x) in board.into_iter().enumerate() {
            match x {
                Some(p) => bitboards[p as usize] |= Square::from_index(i),
                _ => (),
            }
        }
        Positions {
            boards: bitboards,
            hash: hash_boards(&bitboards),
            whites: compute_whites(&bitboards),
            blacks: compute_blacks(&bitboards),
        }
    }

    pub fn new(initial_boards: &[BitBoard]) -> Positions {
        assert_eq!(12, initial_boards.len());
        let initial_hash = hash_boards(initial_boards);
        let mut dest: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        dest.copy_from_slice(initial_boards);
        Positions {
            boards: dest,
            hash: initial_hash,
            whites: compute_whites(&dest),
            blacks: compute_blacks(&dest),
        }
    }

    pub fn side_locations(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.whites(),
            Side::Black => self.blacks(),
        }
    }

    pub fn king_location(&self, side: Side) -> Square {
        self.locs_impl(Piece::king(side)).into_iter().next().unwrap()
    }

    pub fn whites(&self) -> BitBoard {
        self.whites
    }

    pub fn blacks(&self) -> BitBoard {
        self.blacks
    }

    pub fn locs_impl(&self, piece: Piece) -> BitBoard {
        self.boards[piece as usize]
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.boards.iter().zip(Piece::iter()).find(|(&b, _)| b.contains(square)).map(|(_, p)| p)
    }

    pub fn erase_square(&mut self, square: Square) -> Option<Piece> {
        let mut erased_piece = None;
        for (i, p) in Piece::iter().enumerate() {
            if self.boards[i].contains(square) {
                erased_piece = Some(p);
                self.boards[i] ^= square;
                self.hash ^= hash::piece_feature(p, square);
                match p.side() {
                    Side::White => self.whites ^= square,
                    Side::Black => self.blacks ^= square,
                }
                break;
            }
        }
        erased_piece
    }

    pub fn toggle_piece(&mut self, piece: Piece, locations: &[Square]) {
        let mut locationset = BitBoard::EMPTY;
        for &location in locations.iter() {
            locationset ^= location;
            self.hash ^= hash::piece_feature(piece, location);
        }
        self.boards[piece as usize] ^= locationset;
        match piece.side() {
            Side::White => self.whites ^= locationset,
            Side::Black => self.blacks ^= locationset,
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod test {
    use crate::base::square::Square::C3;
    use crate::base::square::Square::E4;
    use crate::base::square::Square::E5;
    use crate::pieces::Piece;

    use super::*;

    #[test]
    fn test_erase_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        let result = board.erase_square(E5);
        assert_eq!(Some(Piece::WP), result);
        assert_eq!(init_tracker(None, Some(C3)), board);
        let result2 = board.erase_square(E4);
        assert_eq!(None, result2);
        assert_eq!(init_tracker(None, Some(C3)), board);
    }

    #[test]
    fn test_toggle_square() {
        let mut board = init_tracker(Some(E5), Some(C3));
        board.toggle_piece(Piece::WP, &[E5, E4]);
        assert_eq!(init_tracker(Some(E4), Some(C3)), board);
    }

    fn init_tracker(pawn_loc: Option<Square>, knight_loc: Option<Square>) -> Positions {
        let mut boards: [BitBoard; 12] = [BitBoard::EMPTY; 12];
        boards[Piece::WP as usize] = pawn_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        boards[Piece::BN as usize] = knight_loc.map_or(BitBoard::EMPTY, |x| x.lift());
        let p_hash = pawn_loc.map_or(0u64, |x| hash::piece_feature(Piece::WP, x));
        let n_hash = knight_loc.map_or(0u64, |x| hash::piece_feature(Piece::BN, x));
        Positions {
            boards,
            hash: p_hash ^ n_hash,
            whites: compute_whites(&boards),
            blacks: compute_blacks(&boards),
        }
    }
}
