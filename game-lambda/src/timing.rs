use log::info;
use tokio::time::Duration;

pub struct Timing {
    /// The function estimating how many
    /// half moves remain in the game given
    /// how many moves have been played
    exp_moves_fn: fn(usize) -> f64,
    /// Increment per move now
    inc: Duration,
    /// Any time added to computing a move
    /// which is not spent thinking
    move_latency: Duration,
    /// Our minimum time to pass as a terminator to
    /// the search function
    min_compute_time: Duration,
}
impl Timing {
    pub fn new(inc: Duration, move_latency: Duration, min_compute_time: Duration) -> Timing {
        Timing {
            exp_moves_fn: expected_half_moves_remaining,
            inc,
            move_latency,
            min_compute_time,
        }
    }

    pub fn compute_thinking_time(
        &self,
        half_moves_played: usize,
        remaining_time: Duration,
    ) -> Duration {
        let estimated = if self.inc.as_secs() > 0 && remaining_time < self.inc_switch_buffer() {
            info!(
                "Remaining time {}ms is not enough to allocate anything above the increment",
                remaining_time.as_millis()
            );
            self.inc - self.move_latency
        } else {
            // Divide by two because we need to think for half of the remaining moves
            let exp_remaining = (self.exp_moves_fn)(half_moves_played) / 2f64;
            info!(
                "We have played {} half moves and expect to play {} more",
                half_moves_played / 2,
                exp_remaining
            );
            let allocated = ((remaining_time.as_millis() as f64) / exp_remaining).round() as u64;
            Duration::from_millis(allocated) + self.inc - self.move_latency
        };

        if estimated > self.min_compute_time {
            info!(
                "Estimated we should spend {}ms thinking",
                estimated.as_millis()
            );
            estimated
        } else {
            info!(
                "Our estimate of {}ms was too low, defaulting to min of {}ms",
                estimated.as_millis(),
                self.min_compute_time.as_millis()
            );
            self.min_compute_time
        }
    }

    fn inc_switch_buffer(&self) -> Duration {
        5 * (self.min_compute_time + self.move_latency)
    }
}

/// https://chess.stackexchange.com/questions/2506/what-is-the-average-length-of-a-game-of-chess
fn expected_half_moves_remaining(moves_played: usize) -> f64 {
    let k = moves_played as f64;
    59.3 + (72830f64 - 2330f64 * k) / (2644f64 + k * (10f64 + k))
}

#[cfg(test)]
mod test {
    use crate::timing::Timing;
    use tokio::time::Duration;

    fn dummy_half_moves_remaining(moves_played: usize) -> f64 {
        moves_played as f64
    }

    #[test]
    fn sub_1sec_inc_estimated_greater_than_min_returns_estimate_minus_latency() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(999),
        };

        assert_eq!(
            Duration::from_millis(4799),
            timing.compute_thinking_time(20, Duration::from_secs(40))
        );
    }

    #[test]
    fn sub_1sec_inc_estimated_less_than_min_returns_min() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(999),
        };

        assert_eq!(
            Duration::from_millis(1100),
            timing.compute_thinking_time(200, Duration::from_secs(10))
        );
    }

    #[test]
    fn sub_1sec_inc_time_remaining_less_than_inc_buffer_returns_estimate_minus_latency() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(999),
        };

        assert_eq!(
            Duration::from_millis(809),
            timing.compute_thinking_time(200, Duration::from_secs(1))
        );
    }

    #[test]
    fn eq_1sec_inc_estimated_greater_than_min_returns_estimate_minus_latency() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(1000),
        };

        assert_eq!(
            Duration::from_millis(4800),
            timing.compute_thinking_time(20, Duration::from_secs(40))
        );
    }

    #[test]
    fn eq_1sec_inc_estimated_less_than_min_returns_min() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(1100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(1000),
        };

        assert_eq!(
            Duration::from_millis(1100),
            timing.compute_thinking_time(200, Duration::from_secs(10))
        );
    }

    #[test]
    fn eq_1sec_inc_time_remaining_less_than_inc_buffer_returns_inc_minus_latency() {
        let timing = Timing {
            exp_moves_fn: dummy_half_moves_remaining,
            min_compute_time: Duration::from_millis(100),
            move_latency: Duration::from_millis(200),
            inc: Duration::from_millis(1000),
        };

        assert_eq!(
            Duration::from_millis(800),
            timing.compute_thinking_time(200, Duration::from_secs(1))
        );
    }
}
