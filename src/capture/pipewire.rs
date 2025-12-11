use crate::capture::capturer::BUFFER;
use crate::capture::capturer::Capturer;
use crate::capture::capturer::Error;
use bytemuck::cast_slice;
use pipewire as pw;
use pw::spa::param::format_utils;
use pw::{properties::properties, spa};
use spa::param::format::{MediaSubtype, MediaType};
use spa::pod::Pod;
use std::mem;
use std::sync::{Arc, RwLock};
use std::thread;
// use spa::param::format::{MediaSubtype, MediaType};
// use spa::param::format_utils;

#[derive(Default)]
pub struct Pipewire {
    user_data: Arc<RwLock<UserData>>,
}

#[derive(Default)]
struct UserData {
    format: spa::param::audio::AudioInfoRaw,
}

impl Capturer for Pipewire {
    // fn capture(&self) -> Result<Vec<f32>, Error> {
    //     match self.buffer.read() {
    //         Ok(r) => Ok(r.to_owned()),
    //         Err(_) => Err(Error::InternalError),
    //     }
    // }

    // fn capture(&self) -> Result<Vec<f32>, Error> {
    //     Ok(self.buffer.read().unwrap().clone())
    // }

    fn init(&self) -> Result<(), Error> {
        let data = Arc::clone(&self.user_data);
        thread::spawn(move || pw_thread(data));
        Ok(())
    }

    fn channels(&self) -> usize {
        self.user_data.read().unwrap().format.channels() as usize
    }

    fn rate(&self) -> usize {
        self.user_data.read().unwrap().format.rate() as usize
    }
}

fn pw_thread(data: Arc<RwLock<UserData>>) -> Result<(), Error> {
    // init pipewire
    pipewire::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_rc(None)?;

    // let data = Arc::clone(&cap_data);

    let props = properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Capture",
        *pw::keys::MEDIA_ROLE => "Music",
        *pw::keys::STREAM_CAPTURE_SINK => "true",

    };

    let stream = pw::stream::StreamBox::new(&core, "audio-capture", props)?;
    // let buffer_clone = Arc::clone(&cap_buffer);

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
                .write()
                .unwrap()
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
            let channels = user_data.read().unwrap().format.channels() as usize;
            let size = data.chunk().size() as usize;

            let type_size = mem::size_of::<f32>();
            let step = type_size * channels;
            let buffer_size = size / step;

            // println!("Size: {} bytes", size);
            // println!("Buffer size: {}", buffer_size);
            if BUFFER.read().unwrap().len() != buffer_size {
                BUFFER.write().unwrap().resize(buffer_size, 0.0);
            }

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
                let mut guard = BUFFER.write().unwrap();
                for start in (0..size).step_by(step) {
                    let end = start + step;
                    let sample = &samples[start..end];
                    let chans = cast_slice(sample);
                    let avg = chans.iter().sum::<f32>() / channels as f32;

                    // let s = guard.len();
                    // eprintln!("buffer starting size: {}", s);
                    // eprintln!("start address: {}", start);
                    // eprintln!("end address: {}", end);
                    // eprintln!("channel average: {}", avg);
                    // eprintln!("buffer index to write: {}", start / step);
                    // eprintln!();

                    guard[start / step] = avg;
                }
                drop(guard);

                // println!(
                //     "Buffer cap after write: {}",
                //     BUFFER.read().unwrap().capacity()
                // );
            }
        })
        .register()?;

    let mut audio_info = spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(spa::param::audio::AudioFormat::F32LE);
    let obj = pw::spa::pod::Object {
        type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
        id: pw::spa::param::ParamType::EnumFormat.as_raw(),
        properties: audio_info.into(),
    };
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [Pod::from_bytes(&values).unwrap()];

    stream.connect(
        spa::utils::Direction::Input,
        None,
        pw::stream::StreamFlags::AUTOCONNECT
            | pw::stream::StreamFlags::MAP_BUFFERS
            | pw::stream::StreamFlags::RT_PROCESS,
        &mut params,
    )?;

    mainloop.run();

    Ok(())
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
