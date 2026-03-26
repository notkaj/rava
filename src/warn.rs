use better_default::Default;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Warnings {
    warnings: VecDeque<&'static str>,
    #[default(Instant::now())]
    last_tick: Instant,
    #[default(Duration::from_millis(4000))]
    default_ttl: Duration,
}

impl Warnings {
    pub fn update(&mut self) {
        if self.warnings.is_empty() {
            return;
        }
        if self.last_tick.elapsed() > self.ttl() {
            self.warnings.pop_front();
            self.last_tick = Instant::now();
        }
    }

    pub fn push(&mut self, text: &'static str) {
        self.warnings.push_back(text);
        self.last_tick = Instant::now();
    }

    fn ttl(&self) -> Duration {
        if self.warnings.len() > 5 {
            Duration::from_millis(500)
        } else if self.warnings.len() > 3 {
            Duration::from_millis(1000)
        } else {
            self.default_ttl
        }
    }
}
