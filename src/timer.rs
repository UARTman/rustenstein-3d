use std::time::{Instant, Duration};

pub struct Timer {
    now: Instant,
    pub dt: Duration
}

impl Timer {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            dt: Default::default()
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.dt = now.duration_since(self.now);
        self.now = now;
    }
}