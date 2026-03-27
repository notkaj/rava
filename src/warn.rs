use better_default::Default;
use std::{
    collections::VecDeque,
    sync::RwLock,
    time::{Duration, Instant},
};

static WARNINGS: RwLock<Warnings> = RwLock::new(Warnings::new());

#[derive(Default)]
struct Warnings {
    warnings: VecDeque<&'static str>,
    #[default(Some(Instant::now()))]
    last_tick: Option<Instant>,
    #[default(Duration::from_millis(4000))]
    default_ttl: Duration,
}

impl Warnings {
    const fn new() -> Self {
        let warnings = VecDeque::new();
        let last_tick = None;
        let default_ttl = Duration::from_millis(4000);
        Self {
            warnings,
            last_tick,
            default_ttl,
        }
    }

    fn update(&mut self) {
        if self.is_empty() {
            return;
        }
        if let Some(tick) = self.last_tick
            && tick.elapsed() > self.ttl()
        {
            self.pop();
            self.last_tick = Some(Instant::now());
        }
    }

    fn push(&mut self, text: &'static str) {
        if self.len() > 9 {
            self.warnings.pop_front();
        }
        self.warnings.push_back(text);
        self.last_tick = Some(Instant::now());
    }

    fn ttl(&self) -> Duration {
        if self.len() > 5 {
            Duration::from_millis(500)
        } else if self.len() > 3 {
            Duration::from_millis(1000)
        } else {
            self.default_ttl
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn pop(&mut self) -> Option<&'static str> {
        self.warnings.pop_front()
    }

    fn len(&self) -> usize {
        self.warnings.len()
    }
}

pub fn warn(text: &'static str) {
    WARNINGS.write().unwrap().push(text);
}

pub fn warnings() -> Vec<&'static str> {
    WARNINGS.read().unwrap().warnings.clone().into()
}

pub fn has_warnings() -> bool {
    !WARNINGS.read().unwrap().is_empty()
}

pub fn update() {
    WARNINGS.write().unwrap().update()
}
