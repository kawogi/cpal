use std::{mem::size_of, ops::Index, slice};

use crate::{types::RawSample, ChannelCount, FrameCount, InputCallbackInfo, SizedSample};

pub mod interleaved;
pub mod separated;

pub type ChannelIndex = ChannelCount;
pub type FrameIndex = FrameCount;
pub type SampleIndex = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SampleAddress {
    pub channel: ChannelIndex,
    pub frame: FrameIndex,
}

pub trait AudioSource: 'static + Send {
    type Item: SizedSample;

    fn fill_buffer<B>(&mut self, buffer: B, info: &InputCallbackInfo)
    where
        B: SampleBufferMut<Item = Self::Item>;
}

pub trait SampleBuffer {
    type Item: Copy;
    type Frame: IntoIterator<Item = Self::Item>;
    type Frames: Iterator<Item = Self::Frame>;
    type Channel: IntoIterator<Item = Self::Item>;
    type Channels: Iterator<Item = Self::Channel>;
    type Samples: Iterator<Item = (SampleAddress, Self::Item)>;
    type SamplesInterleaved: Iterator<Item = Self::Item>;
    type SamplesSeparated: Iterator<Item = Self::Item>;

    /// number of frames in this buffer
    fn frame_count(&self) -> FrameIndex;

    /// returns a single frame of this buffer
    fn frame(&self, index: FrameIndex) -> Self::Frame;

    /// returns an iterator over all frames in this buffer
    fn frames(&self) -> Self::Frames;

    /// number of channels in this buffer
    fn channel_count(&self) -> ChannelCount;

    /// returns a single channel of this buffer
    fn channel(&self, index: ChannelIndex) -> Self::Channel;

    /// returns an iterator over all channels in this buffer
    fn channels(&self) -> Self::Channels;

    /// Iterates over all samples in this buffer and passes them to a callback function along with the sample address.
    /// The order in which the samples are being iterated depends on the implementation but it is guaranteed that all
    /// samples will be visited exactly once.
    fn samples(&self) -> Self::Samples;

    /// Iterates over all samples in this buffer and passes them to a callback function along with the sample address.
    /// The samples will be grouped into frames as if they were stored in frame major order.
    /// i.e.: L0, R0, L1, R1, L2, R2, L3, R3, L4, R4, …
    fn samples_interleaved(&self) -> Self::SamplesInterleaved;

    /// Iterates over all samples in this buffer and passes them to a callback function along with the sample address.
    /// The samples will be grouped into channels as if they were stored in channel major order.
    /// i.e.: L0, L1, L2, L3, L4, … R0, R1, R2, R3, R4, …
    fn samples_separated(&self) -> Self::SamplesSeparated;
}

pub trait SampleBufferMut {
    type Item: Copy;

    /// number of frames in this buffer
    fn frame_count(&self) -> FrameIndex;

    /// writes a single frame into this buffer
    fn write_frame<Frame, Sample>(&mut self, index: FrameIndex, frame: Frame)
    where
        Frame: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;

    /// writes all frames into this buffer
    fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
    where
        Frames: IntoIterator<Item = Frame>,
        Frame: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;

    /// number of channels in this buffer
    fn channel_count(&self) -> ChannelCount;

    /// writes a single channel into this buffer
    fn write_channel<Channel, Sample>(&mut self, index: ChannelIndex, channel: Channel)
    where
        Channel: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;

    /// writes all channels into this buffer
    fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
    where
        Channels: IntoIterator<Item = Channel>,
        Channel: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;

    /// writes a single sample into this buffer
    fn write_sample<Sample>(&mut self, address: SampleAddress, sample: Sample)
    where
        Self::Item: From<Sample>;

    /// Writes all samples into this buffer. The samples will be grouped into frames as if they were stored in frame
    /// major order.
    /// i.e.: L0, R0, L1, R1, L2, R2, L3, R3, L4, R4, …
    fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;

    /// Writes all samples into this buffer. The samples will be grouped into channels as if they were stored in
    /// channel major order.
    /// i.e.: L0, L1, L2, L3, L4, … R0, R1, R2, R3, R4, …
    fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        Self::Item: From<Sample>;
}

/// multiple raw samples consecutively stored in memory
pub struct SampleSlice<'buffer, T: RawSample> {
    samples: &'buffer [T],
}

impl<'buffer, T: RawSample> SampleSlice<'buffer, T> {
    pub fn new(samples: &'buffer [T]) -> Self {
        Self { samples }
    }
}

/// Helper method to convert a byte slice into a slice of a different type (e.g. a `RawSample`).
pub unsafe fn transmute_from_bytes<T: RawSample>(bytes: &[u8]) -> &[T] {
    // make sure the buffer will have no dangling bytes after the conversion
    assert_eq!(bytes.len() % size_of::<T>(), 0);
    let element_count = bytes.len() / size_of::<T>();

    // transmute &[u8] -> &[T]
    slice::from_raw_parts(bytes.as_ptr() as *const T, element_count)
}

/// Helper method to convert a mutable byte slice into a slice of a different type (e.g. a `RawSample`).
pub unsafe fn transmute_from_bytes_mut<T: RawSample>(bytes: &mut [u8]) -> &mut [T] {
    // make sure the buffer will have no dangling bytes after the conversion
    assert_eq!(bytes.len() % size_of::<T>(), 0);
    let element_count = bytes.len() / size_of::<T>();

    // transmute &mut [u8] -> &mut [T]
    slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut T, element_count)
}

impl<'buffer, T: RawSample> IntoIterator for SampleSlice<'buffer, T> {
    type Item = T::Primitive;

    type IntoIter = Samples<'buffer, T>;

    fn into_iter(self) -> Self::IntoIter {
        Samples {
            samples: self.samples.iter(),
        }
    }
}

impl<'buffer, T: RawSample> Index<SampleIndex> for SampleSlice<'buffer, T> {
    type Output = T;

    fn index(&self, index: SampleIndex) -> &Self::Output {
        &self.samples[index]
    }
}

/// Iterator over the primitive view of the Samples stored in a `SampleSlice`.
pub struct Samples<'buffer, T: RawSample> {
    samples: std::slice::Iter<'buffer, T>,
}

impl<'buffer, T: RawSample> Iterator for Samples<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.samples.next().copied().map(T::Primitive::from)
    }
}

#[macro_export]
macro_rules! sized_sample {
    ($self:ident: $($variant:ident),*) => {
        impl $crate::SizedSample for Primitive {
            //const FORMAT: $crate::SampleFormat = FORMAT;

            type RawFormat = RawFormat;
            type Buffer<'buffer> = SampleBuffer<'buffer>;
            type BufferMut<'buffer> = SampleBufferMut<'buffer>;

            fn supports_format(format: $crate::RawSampleFormat) -> bool {
                matches!(format, $crate::RawSampleFormat::$self(_))
            }

            fn create_interleaved_buffer<'buffer>(
                bytes: &'buffer [u8],
                format: $crate::RawSampleFormat,
                channel_count: $crate::ChannelCount,
                frame_count: $crate::FrameCount,
            ) -> Option<Self::Buffer<'buffer>> {
                match format {
                    $(
                    $crate::RawSampleFormat::$self(RawFormat::$variant) => {
                        let samples = unsafe { $crate::buffers::transmute_from_bytes::<$variant>(bytes) };
                        let buffer = $crate::buffers::interleaved::InterleavedBuffer::new(
                            samples,
                            frame_count,
                            channel_count,
                        );
                        let buffer = SampleBuffer::Interleaved(InterleavedBuffer::$variant(buffer));
                        Some(buffer)
                    }
                    )*
                    _ => None,
                }
            }

            fn create_interleaved_buffer_mut<'buffer>(
                bytes: &'buffer mut [u8],
                format: $crate::RawSampleFormat,
                channel_count: $crate::ChannelCount,
                frame_count: $crate::FrameCount,
            ) -> Option<Self::BufferMut<'buffer>> {
                match format {
                    $(
                    $crate::RawSampleFormat::$self(RawFormat::$variant) => {
                        let samples = unsafe { $crate::buffers::transmute_from_bytes_mut::<$variant>(bytes) };
                        let buffer = $crate::buffers::interleaved::InterleavedBufferMut::new(
                            samples,
                            frame_count,
                            channel_count,
                        );
                        let buffer = SampleBufferMut::Interleaved(InterleavedBufferMut::$variant(buffer));
                        Some(buffer)
                    }
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Implements enum wrappers for buffers to generalize over a set of `RawSamples` sharing the same base primitive.
#[macro_export]
macro_rules! sample_buffer {
    ($($variant:ident),*) => {
        // Layout agnostic

        pub enum SampleBuffer<'buffer> {
            Interleaved(InterleavedBuffer<'buffer>),
            Separated(SeparatedBuffer<'buffer>),
        }

        pub enum SampleBufferMut<'buffer> {
            Interleaved(InterleavedBufferMut<'buffer>),
            Separated(SeparatedBufferMut<'buffer>),
        }

        pub enum Frames<'buffer> {
            Interleaved(InterleavedFrames<'buffer>),
            Separated(SeparatedFrames<'buffer>),
        }

        pub enum Frame<'buffer> {
            Interleaved(InterleavedFrame<'buffer>),
            Separated(SeparatedFrame<'buffer>),
        }

        pub enum FrameSamples<'buffer> {
            Interleaved(InterleavedFrameSamples<'buffer>),
            Separated(SeparatedFrameSamples<'buffer>),
        }

        pub enum Channels<'buffer> {
            Interleaved(InterleavedChannels<'buffer>),
            Separated(SeparatedChannels<'buffer>),
        }

        pub enum Channel<'buffer> {
            Interleaved(InterleavedChannel<'buffer>),
            Separated(SeparatedChannel<'buffer>),
        }

        pub enum ChannelSamples<'buffer> {
            Interleaved(InterleavedChannelSamples<'buffer>),
            Separated(SeparatedChannelSamples<'buffer>),
        }

        pub enum Samples<'buffer> {
            Interleaved(InterleavedSamples<'buffer>),
            Separated(SeparatedSamples<'buffer>),
        }

        pub enum SamplesInterleaved<'buffer> {
            Interleaved(InterleavedSamplesInterleaved<'buffer>),
            Separated(SeparatedSamplesInterleaved<'buffer>),
        }

        pub enum SamplesSeparated<'buffer> {
            Interleaved(InterleavedSamplesSeparated<'buffer>),
            Separated(SeparatedSamplesSeparated<'buffer>),
        }

        // Interleaved

        pub enum InterleavedBuffer<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedBuffer<'buffer, $variant>),)*
        }

        pub enum InterleavedBufferMut<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedBufferMut<'buffer, $variant>),)*
        }

        pub enum InterleavedFrames<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedFrames<'buffer, $variant>),)*
        }

        pub enum InterleavedFrame<'buffer> {
            $($variant($crate::buffers::SampleSlice<'buffer, $variant>),)*
        }

        pub enum InterleavedFrameSamples<'buffer> {
            $($variant($crate::buffers::Samples<'buffer, $variant>),)*
        }

        pub enum InterleavedChannels<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedChannels<'buffer, $variant>),)*
        }

        pub enum InterleavedChannel<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedChannel<'buffer, $variant>),)*
        }

        pub enum InterleavedChannelSamples<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedChannelSamples<'buffer, $variant>),)*
        }

        pub enum InterleavedSamples<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedSamples<'buffer, $variant>),)*
        }

        pub enum InterleavedSamplesInterleaved<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedSamplesInterleaved<'buffer, $variant>),)*
        }

        pub enum InterleavedSamplesSeparated<'buffer> {
            $($variant($crate::buffers::interleaved::InterleavedSamplesSeparated<'buffer, $variant>),)*
        }

        // Separated

        pub enum SeparatedBuffer<'buffer> {
            $($variant($crate::buffers::separated::SeparatedBuffer<'buffer, $variant>),)*
        }

        pub enum SeparatedBufferMut<'buffer> {
            $($variant($crate::buffers::separated::SeparatedBufferMut<'buffer, $variant>),)*
        }

        pub enum SeparatedFrames<'buffer> {
            $($variant($crate::buffers::separated::SeparatedFrames<'buffer, $variant>),)*
        }

        pub enum SeparatedFrame<'buffer> {
            $($variant($crate::buffers::separated::SeparatedFrame<'buffer, $variant>),)*
        }

        pub enum SeparatedFrameSamples<'buffer> {
            $($variant($crate::buffers::separated::SeparatedFrameSamples<'buffer, $variant>),)*
        }

        pub enum SeparatedChannels<'buffer> {
            $($variant($crate::buffers::separated::SeparatedChannels<'buffer, $variant>),)*
        }

        pub enum SeparatedChannel<'buffer> {
            $($variant($crate::buffers::SampleSlice<'buffer, $variant>),)*
        }

        pub enum SeparatedChannelSamples<'buffer> {
            $($variant($crate::buffers::Samples<'buffer, $variant>),)*
        }

        pub enum SeparatedSamples<'buffer> {
            $($variant($crate::buffers::separated::SeparatedSamples<'buffer, $variant>),)*
        }
        pub enum SeparatedSamplesInterleaved<'buffer> {
            $($variant($crate::buffers::separated::SeparatedSamplesInterleaved<'buffer, $variant>),)*
        }
        pub enum SeparatedSamplesSeparated<'buffer> {
            $($variant($crate::buffers::separated::SeparatedSamplesSeparated<'buffer, $variant>),)*
        }

        impl<'buffer> $crate::buffers::SampleBuffer for SampleBuffer<'buffer> {
            type Item = Primitive;
            type Frame = Frame<'buffer>;
            type Frames = Frames<'buffer>;
            type Channel = Channel<'buffer>;
            type Channels = Channels<'buffer>;
            type Samples = Samples<'buffer>;
            type SamplesInterleaved = SamplesInterleaved<'buffer>;
            type SamplesSeparated = SamplesSeparated<'buffer>;

            fn frame_count(&self) -> $crate::FrameCount {
                match *self {
                    Self::Interleaved(ref buffer) => buffer.frame_count(),
                    Self::Separated(ref buffer) => buffer.frame_count(),
                }
            }

            fn frame(&self, index: $crate::buffers::FrameIndex) -> Self::Frame {
                match *self {
                    Self::Interleaved(ref buffer) => Self::Frame::Interleaved(buffer.frame(index)),
                    Self::Separated(ref buffer) => Self::Frame::Separated(buffer.frame(index)),
                }
            }

            fn frames(&self) -> Self::Frames {
                match *self {
                    Self::Interleaved(ref buffer) => Self::Frames::Interleaved(buffer.frames()),
                    Self::Separated(ref buffer) => Self::Frames::Separated(buffer.frames()),
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
                match *self {
                    Self::Interleaved(ref buffer) => buffer.channel_count(),
                    Self::Separated(ref buffer) => buffer.channel_count(),
                }
            }

            fn channel(&self, index: $crate::buffers::ChannelIndex) -> Self::Channel {
                match *self {
                    Self::Interleaved(ref buffer) => Self::Channel::Interleaved(buffer.channel(index)),
                    Self::Separated(ref buffer) => Self::Channel::Separated(buffer.channel(index)),
                }
            }

            fn channels(&self) -> Self::Channels {
                match *self {
                    Self::Interleaved(ref buffer) => Self::Channels::Interleaved(buffer.channels()),
                    Self::Separated(ref buffer) => Self::Channels::Separated(buffer.channels()),
                }
            }

            fn samples(&self) -> Self::Samples {
                match *self {
                    Self::Interleaved(ref buffer) => Self::Samples::Interleaved(buffer.samples()),
                    Self::Separated(ref buffer) => Self::Samples::Separated(buffer.samples()),
                }
            }

            fn samples_interleaved(&self) -> Self::SamplesInterleaved {
                match *self {
                    Self::Interleaved(ref buffer) => {
                        Self::SamplesInterleaved::Interleaved(buffer.samples_interleaved())
                    }
                    Self::Separated(ref buffer) => {
                        Self::SamplesInterleaved::Separated(buffer.samples_interleaved())
                    }
                }
            }

            fn samples_separated(&self) -> Self::SamplesSeparated {
                match *self {
                    Self::Interleaved(ref buffer) => {
                        Self::SamplesSeparated::Interleaved(buffer.samples_separated())
                    }
                    Self::Separated(ref buffer) => {
                        Self::SamplesSeparated::Separated(buffer.samples_separated())
                    }
                }
            }
        }

        impl<'buffer> $crate::buffers::SampleBufferMut for SampleBufferMut<'buffer> {
            type Item = Primitive;

            fn frame_count(&self) -> $crate::FrameCount {
                match *self {
                    Self::Interleaved(ref buffer) => buffer.frame_count(),
                    Self::Separated(ref buffer) => buffer.frame_count(),
                }
            }

            fn write_frame<Frame, Sample>(&mut self, index: $crate::buffers::FrameIndex, frame: Frame)
            where
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_frame(index, frame),
                    Self::Separated(ref mut buffer) => buffer.write_frame(index, frame),
                }
            }

            fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
            where
                Frames: IntoIterator<Item = Frame>,
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_frames(frames),
                    Self::Separated(ref mut buffer) => buffer.write_frames(frames),
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
                match *self {
                    Self::Interleaved(ref buffer) => buffer.channel_count(),
                    Self::Separated(ref buffer) => buffer.channel_count(),
                }
            }

            fn write_channel<Channel, Sample>(&mut self, index: $crate::buffers::ChannelIndex, channel: Channel)
            where
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_channel(index, channel),
                    Self::Separated(ref mut buffer) => buffer.write_channel(index, channel),
                }
            }

            fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
            where
                Channels: IntoIterator<Item = Channel>,
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_channels(channels),
                    Self::Separated(ref mut buffer) => buffer.write_channels(channels),
                }
            }

            fn write_sample<Sample>(&mut self, address: $crate::buffers::SampleAddress, sample: Sample)
            where
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_sample(address, sample),
                    Self::Separated(ref mut buffer) => buffer.write_sample(address, sample),
                }
            }

            fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_samples_interleaved(samples),
                    Self::Separated(ref mut buffer) => buffer.write_samples_interleaved(samples),
                }
            }

            fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
                match *self {
                    Self::Interleaved(ref mut buffer) => buffer.write_samples_separated(samples),
                    Self::Separated(ref mut buffer) => buffer.write_samples_separated(samples),
                }
            }
        }

        impl<'buffer> $crate::buffers::SampleBuffer for InterleavedBuffer<'buffer> {
            type Item = Primitive;
            type Frame = InterleavedFrame<'buffer>;
            type Frames = InterleavedFrames<'buffer>;
            type Channel = InterleavedChannel<'buffer>;
            type Channels = InterleavedChannels<'buffer>;
            type Samples = InterleavedSamples<'buffer>;
            type SamplesInterleaved = InterleavedSamplesInterleaved<'buffer>;
            type SamplesSeparated = InterleavedSamplesSeparated<'buffer>;

            fn frame_count(&self) -> $crate::FrameCount {
                match *self {
                    $(Self::$variant(ref buffer) => buffer.frame_count(),)*
                }
            }

            fn frame(&self, index: $crate::buffers::FrameIndex) -> Self::Frame {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Frame::$variant(buffer.frame(index)),)*
                }
            }

            fn frames(&self) -> Self::Frames {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Frames::$variant(buffer.frames()),)*
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.channel_count(),)*
                }
            }

            fn channel(&self, index: $crate::buffers::ChannelIndex) -> Self::Channel {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Channel::$variant(buffer.channel(index)),)*
                }
            }

            fn channels(&self) -> Self::Channels {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Channels::$variant(buffer.channels()),)*
                }
            }

            fn samples(&self) -> Self::Samples {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Samples::$variant(buffer.samples()),)*
                }
            }

            fn samples_interleaved(&self) -> Self::SamplesInterleaved {
            match *self {
                    $(Self::$variant(ref buffer) => Self::SamplesInterleaved::$variant(buffer.samples_interleaved()),)*
                }
            }

            fn samples_separated(&self) -> Self::SamplesSeparated {
            match *self {
                    $(Self::$variant(ref buffer) => Self::SamplesSeparated::$variant(buffer.samples_separated()),)*
                }
            }
        }

        impl<'buffer> $crate::buffers::SampleBufferMut
            for InterleavedBufferMut<'buffer>
        {
            type Item = Primitive;

            fn frame_count(&self) -> $crate::FrameCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.frame_count(),)*
                }
            }

            fn write_frame<Frame, Sample>(&mut self, index: $crate::buffers::FrameIndex, frame: Frame)
            where
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_frame(index, frame),)*
                }
            }

            fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
            where
                Frames: IntoIterator<Item = Frame>,
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_frames(frames),)*
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.channel_count(),)*
                }
            }

            fn write_channel<Channel, Sample>(&mut self, index: $crate::buffers::ChannelIndex, channel: Channel)
            where
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_channel(index, channel),)*
                }
            }

            fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
            where
                Channels: IntoIterator<Item = Channel>,
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_channels(channels),)*
                }
            }

            fn write_sample<Sample>(&mut self, address: $crate::buffers::SampleAddress, sample: Sample)
            where
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_sample(address, sample),)*
                }
            }

            fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_samples_interleaved(samples),)*
                }
            }

            fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_samples_separated(samples),)*
                }
            }
        }

        impl<'buffer> $crate::buffers::SampleBuffer for SeparatedBuffer<'buffer> {
            type Item = Primitive;
            type Frame = SeparatedFrame<'buffer>;
            type Frames = SeparatedFrames<'buffer>;
            type Channel = SeparatedChannel<'buffer>;
            type Channels = SeparatedChannels<'buffer>;
            type Samples = SeparatedSamples<'buffer>;
            type SamplesInterleaved = SeparatedSamplesInterleaved<'buffer>;
            type SamplesSeparated = SeparatedSamplesSeparated<'buffer>;

            fn frame_count(&self) -> $crate::FrameCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.frame_count(),)*
                }
            }

            fn frame(&self, index: $crate::buffers::FrameIndex) -> Self::Frame {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Frame::$variant(buffer.frame(index)),)*
                }
            }

            fn frames(&self) -> Self::Frames {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Frames::$variant(buffer.frames()),)*
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.channel_count(),)*
                }
            }

            fn channel(&self, index: $crate::buffers::ChannelIndex) -> Self::Channel {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Channel::$variant(buffer.channel(index)),)*
                }
            }

            fn channels(&self) -> Self::Channels {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Channels::$variant(buffer.channels()),)*
                }
            }

            fn samples(&self) -> Self::Samples {
            match *self {
                    $(Self::$variant(ref buffer) => Self::Samples::$variant(buffer.samples()),)*
                }
            }

            fn samples_interleaved(&self) -> Self::SamplesInterleaved {
            match *self {
                    $(Self::$variant(ref buffer) => Self::SamplesInterleaved::$variant(buffer.samples_interleaved()),)*
                }
            }

            fn samples_separated(&self) -> Self::SamplesSeparated {
            match *self {
                    $(Self::$variant(ref buffer) => Self::SamplesSeparated::$variant(buffer.samples_separated()),)*
                }
            }
        }

        impl<'buffer> $crate::buffers::SampleBufferMut for SeparatedBufferMut<'buffer> {
            type Item = Primitive;

            fn frame_count(&self) -> $crate::FrameCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.frame_count(),)*
                }
            }

            fn write_frame<Frame, Sample>(&mut self, index: $crate::buffers::FrameIndex, frame: Frame)
            where
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_frame(index, frame),)*
                }
            }

            fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
            where
                Frames: IntoIterator<Item = Frame>,
                Frame: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_frames(frames),)*
                }
            }

            fn channel_count(&self) -> $crate::ChannelCount {
            match *self {
                    $(Self::$variant(ref buffer) => buffer.channel_count(),)*
                }
            }

            fn write_channel<Channel, Sample>(&mut self, index: $crate::buffers::ChannelIndex, channel: Channel)
            where
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_channel(index, channel),)*
                }
            }

            fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
            where
                Channels: IntoIterator<Item = Channel>,
                Channel: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_channels(channels),)*
                }
            }

            fn write_sample<Sample>(&mut self, address: $crate::buffers::SampleAddress, sample: Sample)
            where
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_sample(address, sample),)*
                }
            }

            fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_samples_interleaved(samples),)*
                }
            }

            fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
            where
                Samples: IntoIterator<Item = Sample>,
                Primitive: From<Sample>,
            {
            match *self {
                    $(Self::$variant(ref mut buffer) => buffer.write_samples_separated(samples),)*
                }
            }
        }

        impl<'buffer> IntoIterator for Frame<'buffer> {
            type Item = Primitive;

            type IntoIter = FrameSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
                match self {
                    Self::Interleaved(frame) => Self::IntoIter::Interleaved(frame.into_iter()),
                    Self::Separated(frame) => Self::IntoIter::Separated(frame.into_iter()),
                }
            }
        }

        impl<'buffer> Iterator for FrameSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(samples) => samples.next(),
                    Self::Separated(samples) => samples.next(),
                }
            }
        }

        impl<'buffer> Iterator for Frames<'buffer> {
            type Item = Frame<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(frames) => frames.next().map(Self::Item::Interleaved),
                    Self::Separated(frames) => frames.next().map(Self::Item::Separated),
                }
            }
        }

        impl<'buffer> IntoIterator for InterleavedFrame<'buffer> {
            type Item = Primitive;

            type IntoIter = InterleavedFrameSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
            match self {
                    $(Self::$variant(frame) => Self::IntoIter::$variant(frame.into_iter()),)*
                }
            }
        }

        impl<'buffer> Iterator for InterleavedFrameSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for InterleavedFrames<'buffer> {
            type Item = InterleavedFrame<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(frames) => frames.next().map(Self::Item::$variant),)*
                }
            }
        }

        impl<'buffer> IntoIterator for SeparatedFrame<'buffer> {
            type Item = Primitive;

            type IntoIter = SeparatedFrameSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
            match self {
                    $(Self::$variant(frame) => Self::IntoIter::$variant(frame.into_iter()),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedFrameSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedFrames<'buffer> {
            type Item = SeparatedFrame<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(frames) => frames.next().map(Self::Item::$variant),)*
                }
            }
        }

        impl<'buffer> IntoIterator for Channel<'buffer> {
            type Item = Primitive;

            type IntoIter = ChannelSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
                match self {
                    Self::Interleaved(channel) => Self::IntoIter::Interleaved(channel.into_iter()),
                    Self::Separated(channel) => Self::IntoIter::Separated(channel.into_iter()),
                }
            }
        }

        impl<'buffer> Iterator for ChannelSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(samples) => samples.next(),
                    Self::Separated(samples) => samples.next(),
                }
            }
        }

        impl<'buffer> Iterator for Channels<'buffer> {
            type Item = Channel<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(channels) => channels.next().map(Self::Item::Interleaved),
                    Self::Separated(channels) => channels.next().map(Self::Item::Separated),
                }
            }
        }

        impl<'buffer> IntoIterator for InterleavedChannel<'buffer> {
            type Item = Primitive;

            type IntoIter = InterleavedChannelSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
            match self {
                    $(Self::$variant(channel) => Self::IntoIter::$variant(channel.into_iter()),)*
                }
            }
        }

        impl<'buffer> Iterator for InterleavedChannelSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for InterleavedChannels<'buffer> {
            type Item = InterleavedChannel<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(channels) => channels.next().map(Self::Item::$variant),)*
                }
            }
        }

        impl<'buffer> IntoIterator for SeparatedChannel<'buffer> {
            type Item = Primitive;

            type IntoIter = SeparatedChannelSamples<'buffer>;

            fn into_iter(self) -> Self::IntoIter {
            match self {
                    $(Self::$variant(channel) => Self::IntoIter::$variant(channel.into_iter()),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedChannelSamples<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedChannels<'buffer> {
            type Item = SeparatedChannel<'buffer>;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(channels) => channels.next().map(Self::Item::$variant),)*
                }
            }
        }

        impl<'buffer> Iterator for Samples<'buffer> {
            type Item = ($crate::buffers::SampleAddress, Primitive);

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(samples) => samples.next(),
                    Self::Separated(samples) => samples.next(),
                }
            }
        }

        impl<'buffer> Iterator for InterleavedSamples<'buffer> {
            type Item = ($crate::buffers::SampleAddress, Primitive);

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedSamples<'buffer> {
            type Item = ($crate::buffers::SampleAddress, Primitive);

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SamplesInterleaved<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(samples) => samples.next(),
                    Self::Separated(samples) => samples.next(),
                }
            }
        }

        impl<'buffer> Iterator for InterleavedSamplesInterleaved<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedSamplesInterleaved<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SamplesSeparated<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Interleaved(samples) => samples.next(),
                    Self::Separated(samples) => samples.next(),
                }
            }
        }

        impl<'buffer> Iterator for InterleavedSamplesSeparated<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

        impl<'buffer> Iterator for SeparatedSamplesSeparated<'buffer> {
            type Item = Primitive;

            fn next(&mut self) -> Option<Self::Item> {
            match self {
                    $(Self::$variant(samples) => samples.next(),)*
                }
            }
        }

    };
}
