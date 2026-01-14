use realfft::{RealFftPlanner, RealToComplex, num_complex::Complex};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender}; // For shared data if processing in another thread

pub struct Fft {
    fft: Arc<dyn RealToComplex<f32> + 'static>,
    tx: Sender<Vec<f32>>,
    rx: Receiver<Vec<f32>>,
    input: Vec<f32>,
    output: Vec<Complex<f32>>,
}

impl Fft {
    pub fn new(size: usize, tx: Sender<Vec<f32>>, rx: Receiver<Vec<f32>>) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);

        let input = fft.make_input_vec();
        let output = fft.make_output_vec();

        Self {
            fft,
            tx,
            rx,
            input,
            output,
        }
    }

    pub fn place_input(&mut self, input: &[f32]) {
        // self.input.borrow_mut().copy_from_slice(input);
        self.input.copy_from_slice(input);
    }

    pub fn transform(&mut self) -> Vec<f32> {
        self.fft
            .process(self.input.as_mut_slice(), self.output.as_mut_slice())
            .unwrap();

        self.output.iter().map(|c| c.norm()).collect()
    }

    pub fn init(self) {
        tokio::spawn(async {
            self.fft_thread().await;
        });
    }

    async fn fft_thread(mut self) {
        loop {
            match self.rx.recv().await {
                Some(v) => self.input = v,
                None => break,
            }

            let amps = self.transform();

            if self.tx.send(amps).await.is_err() {
                break;
            }
        }
    }
}
