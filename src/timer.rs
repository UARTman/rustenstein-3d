use std::time::{Duration, Instant};

pub struct Timer {
    now: Instant,
    pub dt: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            dt: Default::default(),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.dt = now.duration_since(self.now);
        self.now = now;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
