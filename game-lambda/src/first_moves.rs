use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

const SEP: &'static str = "|";
const FIRST_MOVES: [(&str, &str); 21] = [
    ("", "e2e4|d2d4|c2c4"),
    ("a2a3", "c7c5"),
    ("a2a4", "g8f6"),
    ("b2b3", "d7d5"),
    ("b2b4", "e7e6"),
    ("c2c3", "g8f6"),
    ("c2c4", "e7e6"),
    ("d2d3", "d7d5"),
    ("d2d4", "d7d5|e7e6"),
    ("e2e3", "g8f6"),
    ("e2e4", "c7c5|e7e5"),
    ("f2f3", "e7e5"),
    ("f2f4", "d7d5"),
    ("g2g3", "d7d5"),
    ("g2g4", "d7d5"),
    ("h2h3", "g8f6"),
    ("h2h4", "d7d5"),
    ("g1h3", "d7d5"),
    ("g1f3", "g8f6"),
    ("b1c3", "d7d5"),
    ("b1a3", "d7d5"),
];

#[derive(Debug)]
pub struct FirstMoveMap {
    internal: HashMap<String, String>,
}

impl FirstMoveMap {
    pub fn get_moves<Q: ?Sized>(&self, key: &Q) -> Vec<String>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.internal.get(key) {
            None => vec![],
            Some(encoded) => encoded.split(SEP).map(|m| m.to_owned()).collect(),
        }
    }
}

pub fn as_map() -> FirstMoveMap {
    FirstMoveMap {
        internal: FIRST_MOVES
            .iter()
            .cloned()
            .map(|(x, y)| (x.to_owned(), y.to_owned()))
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use crate::helper;
    use myopic_brain::{MoveComputeType, MutBoard};

    #[test]
    fn test_get_moves() {
        let move_map = super::as_map();
        assert_eq!(
            vec![format!("e2e4"), format!("d2d4"), format!("c2c4")],
            move_map.get_moves("")
        );
        assert_eq!(vec![format!("c7c5")], move_map.get_moves("a2a3"));
        assert_eq!(Vec::new() as Vec<String>, move_map.get_moves("a2a5"));
    }

    #[test]
    fn test_move_legality() {
        for (setup, first_moves) in super::as_map().internal {
            let (mut board, _) = helper::get_game_state(&setup).expect("Error in setup");
            let legal_moves = board.compute_moves(MoveComputeType::All);
            let expanded_first_moves = first_moves
                .trim()
                .split("|")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            for first_move in expanded_first_moves {
                let move_match = legal_moves
                    .iter()
                    .find(|m| helper::move_to_uci(m) == first_move);

                assert!(
                    move_match.is_some(),
                    "setup: {} first_move: {}",
                    setup,
                    first_move
                )
            }
        }
    }
}
