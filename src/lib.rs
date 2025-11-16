use std::time::{Duration, Instant};

pub trait Puzzle {
    fn solve(&self) -> String;

    fn measure(&self) -> (String, Duration) {
        let start = Instant::now();
        let result = self.solve();
        let elapsed = start.elapsed();
        (result, elapsed)
    }
}
