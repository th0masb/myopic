use crate::eval::additional_components::opening::OpeningComponent;
use crate::eval::EvalComponent;
use crate::Move;

pub mod opening;

#[derive(Clone)]
pub enum EvalComponents {
    /// This component must only be used from a standard
    /// starting position.
    Opening(OpeningComponent),
}

impl EvalComponent for EvalComponents {
    fn static_eval(&self) -> i32 {
        match self {
            EvalComponents::Opening(cmp) => cmp.static_eval(),
        }
    }

    fn make(&mut self, mv: &Move) {
        match self {
            EvalComponents::Opening(cmp) => cmp.make(mv),
        }
    }

    fn unmake(&mut self, mv: &Move) {
        match self {
            EvalComponents::Opening(cmp) => cmp.unmake(mv),
        }
    }
}
