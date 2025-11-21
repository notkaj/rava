use crate::capture::pipewire::Pipewire;
use thiserror::Error;

pub trait Capturer {
    fn capture(&self) -> Result<Vec<f32>, Error>;
    fn init(&self) -> Result<(), Error>;
    fn channels(&self) -> usize;
    fn rate(&self) -> usize;
}

pub fn default_capturer() -> impl Capturer {
    Pipewire::default()
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Creation failed")]
    CreationFailed,
    #[error("Internal Error")]
    InternalError,
    #[error("Invalid Arguement")]
    InvalidArgument,
}
