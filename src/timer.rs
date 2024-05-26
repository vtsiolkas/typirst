use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start_time: Option<Instant>,
    last_action: Option<Instant>,
    total_duration: Duration,
    last_action_duration: Duration,
    pub running: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            last_action: None,
            total_duration: Duration::new(0, 0),
            last_action_duration: Duration::new(0, 0),
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.last_action = Some(Instant::now());
        self.running = true;
    }

    pub fn pause(&mut self) {
        if let Some(start_time) = self.start_time.take() {
            self.total_duration += start_time.elapsed();
        }

        if let Some(last_action) = self.last_action.take() {
            self.last_action_duration += last_action.elapsed();
        }
        self.running = false;
    }

    pub fn reset_last_action(&mut self) {
        self.last_action = Some(Instant::now());
        self.last_action_duration = Duration::new(0, 0);
    }

    pub fn elapsed(&self) -> Duration {
        let extra_duration = self
            .start_time
            .map_or_else(|| Duration::new(0, 0), |t| t.elapsed());

        self.total_duration + extra_duration
    }

    // pub fn elapsed_last_action(&self) -> Duration {
    //     let extra_duration = self
    //         .last_action
    //         .map_or_else(|| Duration::new(0, 0), |t| t.elapsed());
    //
    //     self.last_action_duration + extra_duration
    // }
}
