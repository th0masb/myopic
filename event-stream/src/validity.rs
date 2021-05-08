use crate::events::ClockTimeControl;

#[derive(Copy, Clone)]
pub struct TimeValidity {
    pub initial_bounds_secs: (u32, u32),
    pub increment_bounds_secs: (u32, u32),
}

impl TimeValidity {
    pub fn is_valid(&self, under_test: &ClockTimeControl) -> bool {
        within(self.initial_bounds_secs, under_test.limit)
            && within(self.increment_bounds_secs, under_test.increment)
    }
}

fn within(bounds: (u32, u32), test: u32) -> bool {
    bounds.0 <= test && test <= bounds.1
}
