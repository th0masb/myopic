use log::info;
use tokio::time::Duration;

const DEFAULT_MOVE_LATENCY_MS: u64 = 200;
const DEFAULT_MIN_COMPUTE_TIME_MS: u64 = 200;

pub struct TimeAllocator {
    /// Given the number of moves played return the expected value of moves
    /// still to play.
    half_moves_remaining: fn(usize) -> f64,
    /// Any time added to computing a move which is not spent thinking
    latency: Duration,
    min_compute_time: Duration,
}

impl Default for TimeAllocator {
    fn default() -> Self {
        TimeAllocator {
            half_moves_remaining: expected_half_moves_remaining,
            latency: Duration::from_millis(DEFAULT_MOVE_LATENCY_MS),
            min_compute_time: Duration::from_millis(DEFAULT_MIN_COMPUTE_TIME_MS),
        }
    }
}

impl TimeAllocator {
    pub fn allocate(
        &self,
        half_moves_played: usize,
        remaining_time: Duration,
        increment: Duration,
    ) -> Duration {
        let estimated = if increment.as_secs() > 0 && remaining_time < self.inc_switch_buffer() {
            info!(
                "Remaining time {}ms is not enough to allocate anything above the increment",
                remaining_time.as_millis()
            );
            increment - self.latency
        } else {
            // Divide by two because we need to think for half of the remaining moves
            let exp_remaining = (self.half_moves_remaining)(half_moves_played) / 2f64;
            info!(
                "We have played {} half moves and expect to play {} more",
                half_moves_played / 2,
                exp_remaining
            );
            let allocated = ((remaining_time.as_millis() as f64) / exp_remaining).round() as u64;
            Duration::from_millis(allocated) + increment - self.latency
        };

        if estimated > self.min_compute_time {
            info!("Spending {}ms thinking", estimated.as_millis());
            estimated
        } else {
            info!(
                "{}ms is below min threshold, defaulting to {}ms",
                estimated.as_millis(),
                self.min_compute_time.as_millis()
            );
            self.min_compute_time
        }
    }

    fn inc_switch_buffer(&self) -> Duration {
        5 * (self.min_compute_time + self.latency)
    }
}

/// https://chess.stackexchange.com/questions/2506/what-is-the-average-length-of-a-game-of-chess
fn expected_half_moves_remaining(moves_played: usize) -> f64 {
    let k = moves_played as f64;
    59.3 + (72830f64 - 2330f64 * k) / (2644f64 + k * (10f64 + k))
}

#[cfg(test)]
mod test {
    use tokio::time::Duration;

    use crate::timing::TimeAllocator;

    fn dummy_half_moves_remaining(moves_played: usize) -> f64 {
        moves_played as f64
    }

    #[test]
    fn sub_1sec_inc_estimated_greater_than_min_returns_estimate_minus_latency() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(4799),
            timing.allocate(20, Duration::from_secs(40), Duration::from_millis(999))
        );
    }

    #[test]
    fn sub_1sec_inc_estimated_less_than_min_returns_min() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(1100),
            timing.allocate(200, Duration::from_secs(10), Duration::from_millis(999))
        );
    }

    #[test]
    fn sub_1sec_inc_time_remaining_less_than_inc_buffer_returns_estimate_minus_latency() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(809),
            timing.allocate(200, Duration::from_secs(1), Duration::from_millis(999))
        );
    }

    #[test]
    fn eq_1sec_inc_estimated_greater_than_min_returns_estimate_minus_latency() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(4800),
            timing.allocate(20, Duration::from_secs(40), Duration::from_millis(1000))
        );
    }

    #[test]
    fn eq_1sec_inc_estimated_less_than_min_returns_min() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(1100),
            timing.allocate(200, Duration::from_secs(10), Duration::from_millis(1000))
        );
    }

    #[test]
    fn eq_1sec_inc_time_remaining_less_than_inc_buffer_returns_inc_minus_latency() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(800),
            timing.allocate(200, Duration::from_secs(1), Duration::from_millis(1000))
        );
    }
}
