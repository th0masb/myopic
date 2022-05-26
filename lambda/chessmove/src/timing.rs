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
        let remaining_including_latency = if remaining_time < self.latency {
            Duration::from_millis(0)
        } else {
            remaining_time - self.latency
        };
        // Divide by two because we need to think for half of the remaining moves
        let exp_remaining = (self.half_moves_remaining)(half_moves_played) / 2f64;
        info!(
            "Played {} half moves and expect {} more",
            half_moves_played / 2,
            exp_remaining
        );
        let estimated_no_inc =
            ((remaining_including_latency.as_millis() as f64) / exp_remaining).round() as u64;
        let estimated = Duration::from_millis(estimated_no_inc) + increment;
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
    fn remaining_less_than_latency() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };
        assert_eq!(
            Duration::from_millis(1100),
            timing.allocate(20, Duration::from_millis(100), Duration::from_millis(0))
        )
    }

    #[test]
    fn estimated_greater_than_min() {
        let timing = TimeAllocator {
            half_moves_remaining: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            latency: Duration::from_millis(200),
        };

        assert_eq!(
            Duration::from_millis(4979),
            timing.allocate(20, Duration::from_millis(40000), Duration::from_millis(999))
        );
    }

    #[test]
    fn estimated_less_than_min() {
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
}
