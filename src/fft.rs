use realfft::{RealFftPlanner, RealToComplex, num_complex::Complex};
use std::sync::Arc; // For shared data if processing in another thread

pub struct Fft {
    fft: Arc<dyn RealToComplex<f32> + 'static>,
    input: Vec<f32>,
    output: Vec<Complex<f32>>,
}

impl Default for Fft {
    fn default() -> Self {
        Self::new(2048)
    }
}

impl Fft {
    pub fn new(size: usize) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);

        let input = fft.make_input_vec();
        let output = fft.make_output_vec();
        Self { fft, input, output }
    }

    pub fn place_input(&mut self, input: &[f32]) {
        // self.input.borrow_mut().copy_from_slice(input);
        self.input.copy_from_slice(input);
    }

    pub fn transform(&mut self) -> Vec<f32> {
        self.fft
            .process(
                // self.input.borrow_mut().as_mut_slice(),
                // self.output.borrow_mut().as_mut_slice(),
                self.input.as_mut_slice(),
                self.output.as_mut_slice(),
            )
            .unwrap();

        self.output.iter().map(|c| c.norm()).collect()
    }
}
