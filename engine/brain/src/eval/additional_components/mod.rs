pub mod opening;

use crate::eval::additional_components::opening::OpeningComponent;
use crate::eval::EvalComponent;
use crate::Move;

#[derive(Clone)]
pub enum AdditionalEvalComponent {
    /// This component must only be used from a standard
    /// starting position.
    Opening(OpeningComponent),
}

impl EvalComponent for AdditionalEvalComponent {
    fn static_eval(&self) -> i32 {
        match self {
            AdditionalEvalComponent::Opening(cmp) => cmp.static_eval(),
        }
    }

    fn make(&mut self, mv: &Move) {
        match self {
            AdditionalEvalComponent::Opening(cmp) => cmp.make(mv),
        }
    }

    fn unmake(&mut self, mv: &Move) {
        match self {
            AdditionalEvalComponent::Opening(cmp) => cmp.unmake(mv),
        }
    }
}
