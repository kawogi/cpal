extern crate anyhow;
extern crate clap;
extern crate cpal;

use std::{iter, marker::PhantomData};

use clap::arg;
use cpal::{
    buffers::{AudioSource, SampleBufferMut},
    traits::{DeviceTrait, HostTrait, StreamTrait},
    ChannelCount, Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig, StreamError,
    SupportedStreamConfig, I24, U24,
};
use cpal::{FromSample, SampleFormat};

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
            get_default_output_device,
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

fn main() -> anyhow::Result<()> {
    main_new()
    //main_old()
}

fn main_new() -> anyhow::Result<()> {
    let device = get_default_output_device()?;
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let sample_format = config.sample_format();
    println!("Sample format: {sample_format}");

    match sample_format {
        SampleFormat::I8(_) => run::<i8>(&device, config),
        SampleFormat::I16(_) => run::<i16>(&device, config),
        SampleFormat::I24(_) => run::<I24>(&device, config),
        SampleFormat::I32(_) => run::<i32>(&device, config),
        SampleFormat::I64(_) => run::<i64>(&device, config),
        SampleFormat::U8(_) => run::<u8>(&device, config),
        SampleFormat::U16(_) => run::<u16>(&device, config),
        SampleFormat::U24(_) => run::<U24>(&device, config),
        SampleFormat::U32(_) => run::<u32>(&device, config),
        SampleFormat::U64(_) => run::<u64>(&device, config),
        SampleFormat::F32(_) => run::<f32>(&device, config),
        SampleFormat::F64(_) => run::<f64>(&device, config),
        sample_format => panic!("Unsupported sample format {sample_format}'"),
    }
}

// Produce a sinusoid of maximum amplitude.
struct Sinus<T> {
    sample_clock: f32,
    sample_rate: f32,
    phantom_data: PhantomData<T>,
}

impl<T: Sample + FromSample<f32>> Sinus<T> {
    fn new(sample_rate: SampleRate) -> Self {
        Self {
            sample_clock: 0.0,
            sample_rate: sample_rate.0 as f32,
            phantom_data: PhantomData::default(),
        }
    }

    // Produce a sinusoid of maximum amplitude.
    fn next(&mut self) -> T {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        T::from_sample(
            (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / self.sample_rate).sin(),
        )
    }

    fn next_frame(&mut self, channel_count: ChannelCount) -> impl Iterator<Item = T> {
        iter::repeat(self.next()).take(usize::from(channel_count))
    }
}

impl<T: Sample + FromSample<f32>> AudioSource for Sinus<T> {
    type Item = T;

    fn fill_buffer<'buffer, B: SampleBufferMut<Item = T>>(
        &mut self,
        mut buffer: B,
        _info: &OutputCallbackInfo,
    ) {
        let channel_count = buffer.channel_count();
        buffer.write_frames(iter::repeat_with(|| self.next_frame(channel_count)));
    }
}

fn run<T>(device: &cpal::Device, config: SupportedStreamConfig) -> Result<(), anyhow::Error>
where
    T: Sample + FromSample<f32>,
{
    let config = StreamConfig::from(config);
    let audio_source = Sinus::<T>::new(config.sample_rate);
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream_new(&config, audio_source, err_fn, None)?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn get_default_output_device() -> Result<Device, anyhow::Error> {
    let opt = Opt::from_args();
    let host = get_default_host();

    Ok(if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find output device"))
}

fn get_default_host() -> Host {
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
    cpal::default_host()
}

fn main_old() -> anyhow::Result<()> {
    let device = get_default_output_device()?;
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let format = config.sample_format();
    println!("Format: {format}");

    let err = |err| eprintln!("an error occurred on stream: {}", err);

    let rate = config.sample_rate();
    let config = StreamConfig::from(config);

    match format {
        SampleFormat::I8(_) => run0::<i8, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::I16(_) => run0::<i16, _>(&device, &config, Sinus::new(rate), err),
        // RawSampleFormat::I24(_) => run_old::<I24, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::I32(_) => run0::<i32, _>(&device, &config, Sinus::new(rate), err),
        // RawSampleFormat::I48(_) => run_old::<I48, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::I64(_) => run0::<i64, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::U8(_) => run0::<u8, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::U16(_) => run0::<u16, _>(&device, &config, Sinus::new(rate), err),
        // RawSampleFormat::U24(_) => run_old::<U24, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::U32(_) => run0::<u32, _>(&device, &config, Sinus::new(rate), err),
        // RawSampleFormat::U48(_) => run_old::<U48, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::U64(_) => run0::<u64, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::F32(_) => run0::<f32, _>(&device, &config, Sinus::new(rate), err),
        SampleFormat::F64(_) => run0::<f64, _>(&device, &config, Sinus::new(rate), err),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

fn run0<T, E>(
    device: &cpal::Device,
    config: &StreamConfig,
    mut audio_source: Sinus<T>,
    err_fn: E,
) -> Result<(), anyhow::Error>
where
    T: Sample + FromSample<f32>,
    E: FnMut(StreamError) + Send + 'static,
{
    let channels = config.channels as usize;
    let callback = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        println!(
            "fill_buffer: frames {}, channels {channels}",
            data.len() / channels
        );

        for frame in data.chunks_mut(channels) {
            let value: T = audio_source.next();
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    };

    let stream = device.build_output_stream(config, callback, err_fn, None)?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
