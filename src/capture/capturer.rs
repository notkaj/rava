use std::sync::RwLock;

use thiserror::Error;

use crate::capture::pipewire::Pipewire;

pub(super) static BUFFER: RwLock<Vec<f32>> = RwLock::new(Vec::new());

pub trait Capturer {
    fn init(&self) -> Result<(), Error>;
    fn channels(&self) -> usize;
    fn rate(&self) -> usize;
}

pub fn default_capturer() -> Box<dyn Capturer> {
    Box::new(Pipewire::default())
}

pub fn capture() -> Result<Vec<f32>, Error> {
    Ok(BUFFER.read().unwrap().clone())
}

// pub fn default_capturer() -> impl Capturer {
//     Pipewire::default()
// }

#[derive(Error, Debug)]
pub enum Error {
    #[error("Creation failed")]
    CreationFailed,
    #[error("Internal Error")]
    InternalError,
    #[error("Invalid Arguement")]
    InvalidArgument,
}
