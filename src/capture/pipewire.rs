use crate::capture::capturer::Capturer;
use crate::capture::capturer::Error;
use bytemuck::cast_slice;
use pipewire as pw;
use pw::spa::param::format_utils;
use pw::{properties::properties, spa};
use spa::param::format::{MediaSubtype, MediaType};
use std::mem;
use std::sync::{Arc, RwLock};
// use spa::param::format::{MediaSubtype, MediaType};
// use spa::param::format_utils;

#[derive(Default)]
pub struct Pipewire {
    buffer: Arc<RwLock<Vec<f32>>>,
    user_data: Arc<RwLock<UserData>>,
}

#[derive(Default)]
struct UserData {
    format: spa::param::audio::AudioInfoRaw,
}

impl Capturer for Pipewire {
    fn capture() -> Result<Vec<f32>, Error> {
        Ok(Vec::new())
    }

    fn init(&mut self) -> Result<(), Error> {
        // init pipewire
        pipewire::init();

        let mainloop = pw::main_loop::MainLoopRc::new(None)?;
        let context = pw::context::ContextRc::new(&mainloop, None)?;
        let core = context.connect_rc(None)?;

        let data = UserData {
            format: Default::default(),
        };

        let props = properties! {
            *pw::keys::MEDIA_TYPE => "Audio",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Music",
            *pw::keys::STREAM_CAPTURE_SINK => "true",

        };

        let stream = pw::stream::StreamBox::new(&core, "audio-capture", props)?;
        let buffer_clone = Arc::clone(&self.buffer);

        let _listener = stream
            .add_local_listener_with_user_data(data)
            .param_changed(|_, user_data, id, param| {
                let Some(param) = param else {
                    return;
                };

                if id != pw::spa::param::ParamType::Format.as_raw() {
                    return;
                }

                let (media_type, media_subtype) = match format_utils::parse_format(param) {
                    Ok(v) => v,
                    Err(_) => return,
                };

                if media_type != MediaType::Audio || media_subtype != MediaSubtype::Raw {
                    return;
                }

                user_data
                    .format
                    .parse(param)
                    .expect("Failed to parse param on the param_changed event");
            })
            .process(move |stream, user_data| {
                let mut buffer = stream.dequeue_buffer().unwrap();
                let datas = buffer.datas_mut();
                if datas.is_empty() {
                    return;
                }

                let data = &mut datas[0];
                let channels = user_data.format.channels() as usize;
                let size = data.chunk().size() as usize;

                let type_size = mem::size_of::<f32>();
                let step = type_size * channels;

                buffer_clone.write().unwrap().resize(size / step, 0.0);

                if let Some(samples) = data.data() {
                    // let mut start = 0;
                    // let mut end = start + type_size;
                    // while end < size {
                    //     let index = start / step;
                    //     let mut sum = 0.0;
                    //     for _ in 0..channels {
                    //         let chan = &samples[start..end];
                    //         let sample = f32::from_le_bytes(chan.try_into().unwrap());
                    //         sum += sample;
                    //         start += type_size;
                    //         end += type_size;
                    //     }
                    //     let avg = sum / channels as f32;
                    //     buffer_clone.write().unwrap().insert(index, avg);
                    // }
                    for start in (0..size).step_by(step) {
                        let end = start + step;
                        let sample = &samples[start..end];
                        let chans = cast_slice(sample);
                        let avg = chans.iter().sum::<f32>() / channels as f32;
                        buffer_clone.write().unwrap().insert(start / step, avg);
                    }
                }
            })
            .register()?;

        Ok(())
    }

    fn channels(&self) -> usize {
        self.user_data.read().unwrap().format.channels() as usize
    }
}

impl From<pipewire::Error> for Error {
    fn from(value: pipewire::Error) -> Self {
        match value {
            pipewire::Error::CreationFailed => Error::CreationFailed,
            pipewire::Error::NoMemory => Error::InternalError,
            pipewire::Error::WrongProxyType => Error::InvalidArgument,
            pipewire::Error::SpaError(_) => Error::InternalError,
        }
    }
}

// impl Into<Error> for pipewire::Error {
//     fn into(self) -> Error {
//         Error::CreationFailed
//     }
// }
