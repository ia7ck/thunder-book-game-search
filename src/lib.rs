use ::std::time::{Duration, Instant};

pub mod game;
pub mod search;

struct TimeKeeper {
    instant: Instant,
    threshold: Duration,
}

impl TimeKeeper {
    fn new(threshold: Duration) -> Self {
        Self {
            instant: Instant::now(),
            threshold,
        }
    }

    fn time_over(&self) -> bool {
        self.instant.elapsed() >= self.threshold
    }
}
