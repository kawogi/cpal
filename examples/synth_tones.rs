/* This example expose parameter to pass generator of sample.
Good starting point for integration of cpal into your application.
*/

extern crate anyhow;
extern crate clap;
extern crate cpal;

use std::iter;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Transcoder, samples::{SampleBufferMut, self}, Endianness};
use cpal::{Sample, FromSample};

fn main() -> anyhow::Result<()> {
    let stream = stream_setup_for(sample_next)?;
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_millis(3000));
    Ok(())
}

fn sample_next(o: &mut SampleRequestOptions) -> f32 {
    o.tick();
    o.tone(440.) * 0.1 + o.tone(880.) * 0.1
    // combination of several tones
}

pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub sample_clock: f32,
    pub nchannels: usize,
}

impl SampleRequestOptions {
    fn tone(&self, freq: f32) -> f32 {
        (self.sample_clock * freq * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }
    fn tick(&mut self) {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
    }
}

pub fn stream_setup_for<F>(on_sample: F) -> Result<cpal::Stream, anyhow::Error>
where
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8B1 => stream_make::<samples::i8::B1NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::I16B2(Endianness::NATIVE) => stream_make::<samples::i16::B2NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::I32B4(Endianness::NATIVE) => stream_make::<samples::i32::B4NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::I64B8(Endianness::NATIVE) => stream_make::<samples::i64::B8NE, _>(&device, &config.into(), on_sample),

        cpal::SampleFormat::U8B1 => stream_make::<samples::u8::B1NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::U16B2(Endianness::NATIVE) => stream_make::<samples::u16::B2NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::U32B4(Endianness::NATIVE) => stream_make::<samples::u32::B4NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::U64B8(Endianness::NATIVE) => stream_make::<samples::u64::B8NE, _>(&device, &config.into(), on_sample),

        cpal::SampleFormat::F32B4(Endianness::NATIVE) => stream_make::<samples::f32::B4NE, _>(&device, &config.into(), on_sample),
        cpal::SampleFormat::F64B8(Endianness::NATIVE) => stream_make::<samples::f64::B8NE, _>(&device, &config.into(), on_sample),

        sample_format => Err(anyhow::Error::msg(format!("Unsupported sample format '{sample_format}'"))),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

pub fn stream_make<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    on_sample: F,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: Transcoder,
    T::Sample: FromSample<f32>,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let sample_rate = config.sample_rate.0 as f32;
    let sample_clock = 0f32;
    let nchannels = config.channels as usize;
    let mut request = SampleRequestOptions {
        sample_rate,
        sample_clock,
        nchannels,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |output: SampleBufferMut<T>, _: &cpal::OutputCallbackInfo| {
            on_window(output, &mut request, on_sample)
        },
        err_fn,
    )?;

    Ok(stream)
}

fn on_window<T, F>(output: SampleBufferMut<T>, request: &mut SampleRequestOptions, mut on_sample: F)
where
    T: Transcoder,
    T::Sample: FromSample<f32>,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static,
{

    let source = iter::from_fn(|| {
        let sample = T::Sample::from_sample(on_sample(request));
            Some(iter::repeat(sample).take(request.nchannels))
        }).flatten();

    output.into_iter().write_iter(source);
}
