use crate::moves::Move;
use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrincipleVariation {
    path: Vec<Move>,
}

impl PrincipleVariation {
    pub fn set(&mut self, path: &[Move]) {
        self.path = path.to_vec();
    }

    pub fn in_pv(&self, path: &[Move]) -> bool {
        self.path.starts_with(&path[0..min(self.path.len(), path.len())])
    }

    pub fn get_next_move(&self, path: &[Move]) -> Option<Move> {
        self.path.strip_prefix(path).and_then(|rest| rest.first()).cloned()
    }
}

#[cfg(test)]
mod test {
    use crate::constants::piece;
    use crate::constants::square::*;
    use crate::moves::Move;
    use crate::search::pv::PrincipleVariation;
    use Move::Normal;

    #[test]
    fn in_pv_test() {
        let path = vec![
            Normal { moving: piece::WP, from: E2, dest: E4, capture: None },
            Normal { moving: piece::BP, from: E5, dest: E7, capture: None },
            Normal { moving: piece::WN, from: F1, dest: G3, capture: None },
        ];

        let pv = PrincipleVariation { path: path[..2].to_vec() };

        assert!(pv.in_pv(&[]));
        assert!(pv.in_pv(&path[..1]));
        assert!(pv.in_pv(&path[..2]));
        assert!(pv.in_pv(path.as_slice()));
    }
}
