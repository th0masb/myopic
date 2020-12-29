pub mod opening;

use crate::eval::additional_components::opening::OpeningComponent;
use crate::eval::EvalComponent;
use crate::{Move, Reflectable};

#[derive(Clone)]
pub enum AdditionalEvalComponent {
    Opening(OpeningComponent),
}

impl Reflectable for AdditionalEvalComponent {
    fn reflect(&self) -> Self {
        match self {
            AdditionalEvalComponent::Opening(cmp) => {
                AdditionalEvalComponent::Opening(cmp.reflect())
            }
        }
    }
}

impl EvalComponent for AdditionalEvalComponent {
    fn static_eval(&mut self) -> i32 {
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
