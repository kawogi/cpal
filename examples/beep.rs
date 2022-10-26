extern crate anyhow;
extern crate clap;
extern crate cpal;

use std::{iter, marker::PhantomData};

use clap::arg;
use cpal::{
    buffers::{AudioSource, SampleBufferMut},
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, SampleRate, SizedSample, StreamConfig, StreamError, I24, U24,
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

fn main() -> anyhow::Result<()> {
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

    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate();
    let config = StreamConfig::from(config);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    match sample_format {
        cpal::RawSampleFormat::I8(_) => run::<i8, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::I16(_) => run::<i16, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::I24(_) => run::<I24, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::I32(_) => run::<i32, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::I64(_) => run::<i64, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::U8(_) => run::<u8, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::U16(_) => run::<u16, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::U24(_) => run::<U24, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::U32(_) => run::<u32, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::U64(_) => run::<u64, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::F32(_) => run::<f32, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        cpal::RawSampleFormat::F64(_) => run::<f64, _>(
            &device,
            &config,
            sample_format,
            Sinus::new(sample_rate),
            err_fn,
        ),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
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
        println!("fill_buffer");
        let sample = T::from_sample(self.next());
        let channel_count = buffer.channel_count();
        let frame = || iter::repeat(sample).take(usize::from(channel_count));
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
    T: SizedSample + FromSample<f32>, // + SizedSample,
    E: FnMut(StreamError) + Send + 'static,
{
    let stream =
        device.build_output_stream_new(config, sample_format, audio_source, err_fn, None)?;
    stream.play()?;

    Ok(())
}

// pub fn run<B>(
//     device: &cpal::Device,
//     config: &cpal::StreamConfig,
//     sample_source: Sinus,
//     channels: usize,
// ) -> Result<(), anyhow::Error>
// where
//     for<'buffer> B: SampleBufferMut<'buffer>,
//     //for<'buffer> <B as cpal::buffers::SampleBufferMut<'buffer>>::Item: FromSample<f32>,
// {
//     //let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

//     // let stream = device.build_output_stream(
//     //     config,
//     //     move |data: B, _: &cpal::OutputCallbackInfo| write_data(data, channels, &mut next_value),
//     //     err_fn,
//     //     None,
//     // )?;
//     // stream.play()?;

//     std::thread::sleep(std::time::Duration::from_millis(1000));

//     Ok(())
// }

// fn write_data<T, B>(mut output: B, channels: usize, sample_source: &mut Sinus)
// where
//     B: for<'buffer> SampleBufferMut<'buffer, Item = T>, // SizedSample + FromSample<f32>,
//     T: Sample + FromSample<f32>,
//     //for<'buffer> <B as cpal::buffers::SampleBufferMut<'buffer>>::Item: FromSample<f32>,
// {
//     let sample: B::Item = B::Item::from_sample(sample_source.next());
//     let frame = || iter::repeat(sample).take(channels);
//     let frames = iter::repeat_with(frame);
//     output.write_frames(frames);
// }
