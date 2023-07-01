use std::iter::FromIterator;
use crate::Board;
use anyhow::Result;
use myopic_core::anyhow::anyhow;

use myopic_core::enum_map::EnumMap;
use myopic_core::*;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct RaySet {
    points: BitBoard,
    contents: EnumMap<Square, BitBoard>,
}

impl FromIterator<(Square, BitBoard)> for RaySet {
    fn from_iter<T: IntoIterator<Item = (Square, BitBoard)>>(iter: T) -> Self {
        let mut points = BitBoard::EMPTY;
        let mut contents = EnumMap::default();
        for (square, board) in iter {
            contents[square] = board;
            points |= square;
        }
        RaySet { points, contents }
    }
}

impl RaySet {
    pub fn points(&self) -> BitBoard {
        self.points
    }

    pub fn ray(&self, loc: Square) -> Option<BitBoard> {
        if self.points.contains(loc) {
            Some(self.contents[loc])
        } else {
            None
        }
    }
}

impl Reflectable for RaySet {
    fn reflect(&self) -> Self {
        let mut contents = EnumMap::default();
        for point in self.points {
            contents[point.reflect()] = self.contents[point].reflect();
        }
        RaySet { points: self.points.reflect(), contents }
    }
}

impl Board {
    pub(crate) fn pinned_set(&self) -> RaySet {
        let mut cache = self.cache.borrow_mut();
        match &cache.pinned_set {
            Some(x) => x.clone(),
            None => {
                let result = self.compute_pinned_on(Piece(self.active, Class::K)).unwrap();
                cache.pinned_set = Some(result.clone());
                result
            }
        }
    }

    pub(crate) fn compute_discoveries_on(&self, piece: Piece) -> Result<RaySet> {
        let Piece(discovered_side, class) = piece;
        let discoverer_side = discovered_side.reflect();
        let discovered_loc = match class {
            Class::K => self.king(discovered_side),
            _ => self.locs(&[piece]).first()
                .ok_or(anyhow!("{:?} not on the board", piece))?
        };

        let discoverer_locs = self.side(discoverer_side);
        let discovered_locs = self.side(discovered_side);

        Ok(self.compute_xrayers(discoverer_side, discovered_loc)
            .iter()
            .map(|xrayer| (xrayer, BitBoard::cord(discovered_loc, xrayer)))
            .filter(|&(_, cord)| (cord & discoverer_locs).size() == 2 && (cord & discovered_locs).size() == 1)
            .map(|(xrayer, cord)| (((cord & discoverer_locs) - xrayer).first().unwrap(), cord))
            .collect())
    }

    /// Compute all the pieces which are pinned to the given piece
    pub(crate) fn compute_pinned_on(&self, piece: Piece) -> Result<RaySet> {
        let Piece(pinned_side, class) = piece;
        let pinner_side = pinned_side.reflect();
        let pinned_loc = match class {
            Class::K => self.king(pinned_side),
            _ => self.locs(&[piece]).first()
                .ok_or(anyhow!("{:?} not on the board", piece))?
        };

        let pinner_locs = self.side(pinner_side);
        let pinned_locs = self.side(pinned_side);

        Ok(self.compute_xrayers(pinner_side, pinned_loc)
            .iter()
            .map(|sq| BitBoard::cord(pinned_loc, sq))
            .filter(|&cord| (cord & pinned_locs).size() == 2 && (cord & pinner_locs).size() == 1)
            .map(|cord| (((cord & pinned_locs) - pinned_loc).first().unwrap(), cord))
            .collect())
    }

    fn compute_xrayers(&self, side: Side, square: Square) -> BitBoard {
        [Class::B, Class::R, Class::Q]
            .iter()
            .map(|&class| Piece(side, class))
            .map(|piece| self.pieces.locs(piece) & piece.empty_control(square))
            .collect()
    }
}

#[cfg(test)]
mod pinned_test {
    use myopic_core::Square::*;

    use super::*;

    fn execute_test(fen: &'static str, expected_pinned: RaySet) {
        let board = fen.parse::<Board>().unwrap();
        let active_king = Piece(board.active, Class::K);
        assert_eq!(
            expected_pinned.reflect(),
            board.reflect().compute_pinned_on(active_king.reflect()).unwrap()
        );
        assert_eq!(
            expected_pinned,
            board.compute_pinned_on(active_king).unwrap()
        );
    }

    #[test]
    fn case_one() {
        let fen = "K2Q4/7p/1B4n1/2bq4/2rkp1R1/4p3/5br1/6B1 b KQkq - 5 10";
        let expected_pinned =
            vec![(E4, D4 | E4 | F4 | G4), (C5, B6 | C5 | D4), (D5, D4 | D5 | D6 | D7 | D8)]
                .into_iter()
                .collect();

        execute_test(fen, expected_pinned);
    }
}

#[cfg(test)]
mod discovery_test {
    use myopic_core::Square::*;

    use super::*;

    fn execute_test(fen: &'static str, expected_discoveries: RaySet) {
        let board = fen.parse::<Board>().unwrap();
        let passive_king = Piece(board.active.reflect(), Class::K);
        assert_eq!(
            expected_discoveries.reflect(),
            board.reflect().compute_discoveries_on(passive_king.reflect()).unwrap()
        );
        assert_eq!(
            expected_discoveries,
            board.compute_discoveries_on(passive_king).unwrap()
        );
    }

    #[test]
    fn case_one() {
        let fen = "6r1/5p1k/4pP2/4N3/3PN3/6P1/2B3PK/7R w - - 1 10";
        let expected_pinned =
            vec![(E4, C2 | D3 | E4 | F5 | G6 | H7), (H2, H1 | H2 | H3 | H4 | H5 | H6 | H7)]
                .into_iter()
                .collect();

        execute_test(fen, expected_pinned);
    }
}
