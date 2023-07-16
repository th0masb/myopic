use std::time::Duration;

use crate::search::negascout::Context;

/// Represents some object which can determine whether a search should be
/// terminated given certain context about the current state. Implementations
/// are provided for Duration (caps the search based on time elapsed), for
/// usize which represents a maximum search depth and for a pair (Duration, usize)
/// which combines both checks.
pub trait SearchTerminator {
    fn should_terminate(&self, ctx: &Context) -> bool;
}

impl SearchTerminator for Duration {
    fn should_terminate(&self, ctx: &Context) -> bool {
        ctx.start.elapsed() > *self
    }
}

impl SearchTerminator for usize {
    fn should_terminate(&self, ctx: &Context) -> bool {
        ctx.depth as usize > *self
    }
}

impl SearchTerminator for (Duration, usize) {
    fn should_terminate(&self, ctx: &Context) -> bool {
        self.0.should_terminate(ctx) || self.1.should_terminate(ctx)
    }
}
