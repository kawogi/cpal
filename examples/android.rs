#![allow(dead_code)]

extern crate anyhow;
extern crate cpal;

use std::iter;

use cpal::{
    buffers::SampleBufferMut,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, I24, U24,
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");

    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I8(_) => run::<i8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16(_) => run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I24(_) => run::<I24>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I32(_) => run::<i32>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::I48(_) => run::<I48>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I64(_) => run::<i64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U8(_) => run::<u8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16(_) => run::<u16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U24(_) => run::<U24>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U32(_) => run::<u32>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::U48(_) => run::<U48>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U64(_) => run::<u64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F32(_) => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F64(_) => run::<f64>(&device, &config.into()).unwrap(),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: Sample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream::<T, _, _>(
        config,
        move |data: T::BufferMut<'_>, _: &cpal::OutputCallbackInfo| {
            write_data::<T>(data, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(mut output: T::BufferMut<'_>, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    let next_frame = || iter::repeat(T::from_sample(next_sample()));
    output.write_frames(iter::repeat_with(next_frame));
}
