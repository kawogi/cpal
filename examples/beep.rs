extern crate anyhow;
extern crate clap;
extern crate cpal;

use std::{iter, marker::PhantomData};

use clap::arg;
use cpal::{
    buffers::{AudioSource, SampleBufferMut},
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, Sample, SampleRate, SizedSample, StreamConfig, StreamError, I24, U24,
};
use cpal::{FromSample, RawSampleFormat};

#[derive(Debug)]
struct Opt {
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    jack: bool,

    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let app = clap::Command::new("beep").arg(arg!([DEVICE] "The audio device to use"));
        #[cfg(all(
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            ),
            feature = "jack"
        ))]
        let app = app.arg(arg!(-j --jack "Use the JACK host"));
        let matches = app.get_matches();
        let device = matches.value_of("DEVICE").unwrap_or("default").to_string();

        #[cfg(all(
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            ),
            feature = "jack"
        ))]
        return Opt {
            jack: matches.is_present("jack"),
            device,
        };

        #[cfg(any(
            not(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            )),
            not(feature = "jack")
        ))]
        Opt { device }
    }
}

fn main_new() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find output device");
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let format = config.sample_format();
    println!("Format: {format}");

    let err = |err| eprintln!("an error occurred on stream: {}", err);

    let rate = config.sample_rate();
    let config = StreamConfig::from(config);

    // difference to old implementation

    match format {
        RawSampleFormat::I8(_) => run::<i8, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::I16(_) => run::<i16, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::I24(_) => run::<I24, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::I32(_) => run::<i32, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::I64(_) => run::<i64, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::U8(_) => run::<u8, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::U16(_) => run::<u16, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::U24(_) => run::<U24, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::U32(_) => run::<u32, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::U64(_) => run::<u64, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::F32(_) => run::<f32, _>(&device, &config, format, Sinus::new(rate), err),
        RawSampleFormat::F64(_) => run::<f64, _>(&device, &config, format, Sinus::new(rate), err),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find output device");
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let format = config.sample_format();
    println!("Format: {format}");

    let err = |err| eprintln!("an error occurred on stream: {}", err);

    let rate = config.sample_rate();
    let config = StreamConfig::from(config);

    match format {
        RawSampleFormat::I8(_) => run_old::<i8, _>(&device, &config.into(), err),
        RawSampleFormat::I16(_) => run_old::<i16, _>(&device, &config.into(), err),
        // RawSampleFormat::I24(_) => run_old::<I24, _>(&device, &config.into(), err),
        RawSampleFormat::I32(_) => run_old::<i32, _>(&device, &config.into(), err),
        // RawSampleFormat::I48(_) => run_old::<I48, _>(&device, &config.into(), err),
        RawSampleFormat::I64(_) => run_old::<i64, _>(&device, &config.into(), err),
        RawSampleFormat::U8(_) => run_old::<u8, _>(&device, &config.into(), err),
        RawSampleFormat::U16(_) => run_old::<u16, _>(&device, &config.into(), err),
        // RawSampleFormat::U24(_) => run_old::<U24, _>(&device, &config.into(), err),
        RawSampleFormat::U32(_) => run_old::<u32, _>(&device, &config.into(), err),
        // RawSampleFormat::U48(_) => run_old::<U48, _>(&device, &config.into(), err),
        RawSampleFormat::U64(_) => run_old::<u64, _>(&device, &config.into(), err),
        RawSampleFormat::F32(_) => run_old::<f32, _>(&device, &config.into(), err),
        RawSampleFormat::F64(_) => run_old::<f64, _>(&device, &config.into(), err),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

struct Sinus<T> {
    sample_clock: f32,
    sample_rate: f32,
    phantom_data: PhantomData<T>,
}

impl<T> Sinus<T> {
    fn new(sample_rate: SampleRate) -> Self {
        Self {
            sample_clock: 0.0,
            sample_rate: sample_rate.0 as f32,
            phantom_data: PhantomData::default(),
        }
    }

    // Produce a sinusoid of maximum amplitude.
    fn next(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }
}

impl<T: SizedSample + FromSample<f32>> AudioSource for Sinus<T> {
    type Item = T;

    fn fill_buffer<'buffer, B: SampleBufferMut<Item = T>>(
        &mut self,
        mut buffer: B,
        _info: &InputCallbackInfo,
    ) {
        println!(
            "fill_buffer: frames {}, channels {}",
            buffer.frame_count(),
            buffer.channel_count()
        );
        let channel_count = buffer.channel_count();
        let frame = || {
            let sample = T::from_sample(self.next());
            iter::repeat(sample).take(usize::from(channel_count))
        };
        let frames = iter::repeat_with(frame);
        buffer.write_frames(frames);
    }
}

fn run<T, E>(
    device: &cpal::Device,
    config: &StreamConfig,
    sample_format: RawSampleFormat,
    audio_source: Sinus<T>,
    err_fn: E,
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
    E: FnMut(StreamError) + Send + 'static,
{
    let stream =
        device.build_output_stream_new(config, sample_format, audio_source, err_fn, None)?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

pub fn run_old<T, E>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    err_fn: E,
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
    E: FnMut(StreamError) + Send + 'static,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data_old(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data_old<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
