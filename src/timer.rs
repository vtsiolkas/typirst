use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start_time: Option<Instant>,
    total_duration: Duration,
    pub running: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            total_duration: Duration::new(0, 0),
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.running = true;
    }

    pub fn pause(&mut self) {
        if !self.running {
            return;
        }
        if let Some(start_time) = self.start_time.take() {
            self.total_duration += start_time.elapsed();
        }
        self.running = false;
    }

    pub fn elapsed(&self) -> Duration {
        let extra_duration = self
            .start_time
            .map_or_else(|| Duration::new(0, 0), |t| t.elapsed());

        self.total_duration + extra_duration
    }
}
