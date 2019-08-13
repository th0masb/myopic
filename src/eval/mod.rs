use crate::board::Board;

mod see;
mod tables;
mod values;
mod evalboardimpl;

/// Extension of the Board trait which adds a static evaluation function.
///
pub trait EvalBoard: Board {
    /// The static evaluation function assigns a score to this exact
    /// position at the point of time it is called. It does not take
    /// into account potential captures/recaptures etc. It must follow
    /// the rule that 'a higher score is best for the active side'. That
    /// is if it is white to move next then a high positive score indicates
    /// a favorable position for white and if it is black to move a high
    /// positive score indicates a favorable position for black.
    fn static_eval(&self) -> i32;
}
