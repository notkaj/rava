use super::Filter;

const DEFAULT_RATE_OF_DECAY: f32 = 0.05;
const DEFAULT_PEAK_DUR_TICKS: u8 = 5;

pub struct ExperimentalFilter {
    rate_of_decay: f32,
    peak_dur_ticks: u8,
    ticks: Vec<u8>,
}

#[allow(dead_code)]
impl ExperimentalFilter {
    pub fn new(len: usize, rate_of_decay: f32, peak_dur_ticks: u8) -> Self {
        let ticks = vec![0; len];
        Self {
            rate_of_decay,
            peak_dur_ticks,
            ticks,
        }
    }

    pub fn new_default(len: usize) -> Self {
        ExperimentalFilter::new(len, DEFAULT_RATE_OF_DECAY, DEFAULT_PEAK_DUR_TICKS)
    }
}

impl Filter for ExperimentalFilter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]) {
        let len = input.len();
        if self.ticks.len() != len {
            self.ticks.resize(len, 0);
        }
        for (i, e) in input.iter().enumerate() {
            let entry = *e as u32;
            let tick = self.ticks[i];
            if tick == 0 && entry > out[i] {
                // let diff = entry - out[i];
                // out[i] += (diff as f32 * self.rate_of_increase) as u32;
                out[i] = entry;
                self.ticks[i] = self.peak_dur_ticks;
            } else {
                // i don't think this needs to be saturated
                self.ticks[i] = tick.saturating_sub(1);
                let curr = out[i];
                let decay = (curr as f32 * self.rate_of_decay).ceil() as u32;
                out[i] = curr - decay;
            }
        }
    }
}
