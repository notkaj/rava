use crate::capture::capturer::BUFFER;
use crate::capture::capturer::Capturer;
use crate::capture::capturer::Error;
use crate::capture::capturer::Interp;
use bytemuck::cast_slice;
use pipewire as pw;
use pw::spa::param::format_utils;
use pw::stream::Stream;
use pw::{properties::properties, spa};
use spa::param::format::{MediaSubtype, MediaType};
use spa::pod::Pod;
use std::mem;
use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Default)]
pub struct Pipewire {
    user_data: Arc<RwLock<UserData>>,
    interp: Interp,
}

#[derive(Default)]
struct UserData {
    format: spa::param::audio::AudioInfoRaw,
}

impl Pipewire {
    pub fn new(interp: Interp) -> Self {
        let user_data = Default::default();
        Pipewire { user_data, interp }
    }
}

impl Capturer for Pipewire {
    fn init(&self) -> Result<(), Error> {
        let data = Arc::clone(&self.user_data);
        let interp = self.interp;
        thread::spawn(move || pw_thread(data, interp));
        Ok(())
    }

    fn channels(&self) -> usize {
        self.user_data.read().unwrap().format.channels() as usize
    }

    fn rate(&self) -> usize {
        self.user_data.read().unwrap().format.rate() as usize
    }
}

fn pw_thread(data: Arc<RwLock<UserData>>, interp: Interp) -> Result<(), Error> {
    // init pipewire
    pipewire::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_rc(None)?;

    let props = properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Capture",
        *pw::keys::MEDIA_ROLE => "Music",
        *pw::keys::STREAM_CAPTURE_SINK => "true",

    };

    let stream = pw::stream::StreamBox::new(&core, "rava-audio-capture", props)?;

    let process_fn = match interp {
        Interp::Averaged => averaged,
        Interp::Interpolated => interpolated,
    };

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
        .process(process_fn)
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

fn averaged(stream: &Stream, user_data: &mut Arc<RwLock<UserData>>) {
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

    // TODO:probably don't need this conditional
    if BUFFER.read().unwrap().len() != buffer_size {
        BUFFER.write().unwrap().resize(buffer_size, 0.0);
    }

    if let Some(samples) = data.data() {
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
        // println!(
        //     "Buffer cap after write: {}",
        //     BUFFER.read().unwrap().capacity()
        // );
    }
}

#[allow(dead_code)]
fn interpolated(stream: &Stream, _: &mut Arc<RwLock<UserData>>) {
    let mut buffer = stream.dequeue_buffer().unwrap();
    let datas = buffer.datas_mut();
    if datas.is_empty() {
        return;
    }

    let data = &mut datas[0];
    let size = data.chunk().size() as usize;

    let type_size = mem::size_of::<f32>();
    let buffer_size = size / type_size;

    if BUFFER.read().unwrap().len() != buffer_size {
        BUFFER.write().unwrap().resize(buffer_size, 0.0);
    }

    if let Some(samples) = data.data() {
        let mut guard = BUFFER.write().unwrap();
        let interpol = cast_slice(samples);
        guard.copy_from_slice(interpol);
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
