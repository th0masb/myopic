use myopic_board::Move;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrincipleVariation {
    path: Vec<Move>,
}

impl PrincipleVariation {
    pub fn set(&mut self, path: &[Move]) {
        self.path = path.to_vec();
    }

    pub fn get_next_move(&self, path: &[Move]) -> Option<Move> {
        self.path.strip_prefix(path).and_then(|rest| rest.first()).cloned()
    }
}
