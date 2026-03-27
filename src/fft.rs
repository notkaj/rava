use realfft::{RealFftPlanner, RealToComplex, num_complex::Complex};
use std::sync::Arc;
use tokio::sync::mpsc::{
    Receiver, Sender,
    error::{TryRecvError, TrySendError},
};

use crate::warn;

pub struct Fft {
    fft: Arc<dyn RealToComplex<f32> + 'static>,
    tx: Sender<Vec<f32>>,
    rx: Receiver<Vec<f32>>,
    output: Vec<Complex<f32>>,
}

impl Fft {
    pub fn new(size: usize, tx: Sender<Vec<f32>>, rx: Receiver<Vec<f32>>) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);

        let output = fft.make_output_vec();

        Self {
            fft,
            tx,
            rx,
            // input,
            output,
        }
    }

    pub fn transform(&mut self, input: &mut Vec<f32>) -> Vec<f32> {
        self.fft
            .process(input.as_mut_slice(), self.output.as_mut_slice())
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
            let mut input = match self.rx.recv().await {
                Some(v) => v,
                None => break,
            };

            let amps = self.transform(&mut input);

            if self.tx.send(amps).await.is_err() {
                break;
            }
        }
    }
}

pub fn exchange(
    sample: Vec<f32>,
    tx_to_fft: &Sender<Vec<f32>>,
    rx_from_fft: &mut Receiver<Vec<f32>>,
) -> Vec<f32> {
    let sample_len = sample.len();

    let transform = match rx_from_fft.try_recv() {
        Ok(t) => t,
        Err(e) => match e {
            TryRecvError::Empty => vec![0.0; sample_len],
            TryRecvError::Disconnected => {
                panic!("transform could not be received: channel disconnected")
            }
        },
    };

    if let Err(e) = tx_to_fft.try_send(sample) {
        match e {
            TrySendError::Full(_) => {
                warn::warn("fft buffer full")
                // should throw a warning, but the channels should eventually sync back up
                // panic!("sample could not be transferred: fft buffer is full")
            }
            TrySendError::Closed(_) => {
                panic!("sample could not be transferred: fft rx has been closed")
            }
        }
    }

    transform
}
