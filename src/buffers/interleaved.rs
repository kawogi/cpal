use std::{
    iter::{Cycle, Skip, StepBy, Zip},
    ops::{Index, Range},
    slice::{self, ChunksExact},
};

use itertools::Itertools;

use crate::{samples::RawSample, ChannelCount, FrameCount};

use super::{
    ChannelIndex, FrameIndex, SampleAddress, SampleBuffer, SampleBufferMut, SampleIndex,
    SampleSlice,
};

/// Contains samples in a single buffer grouped by frames.
pub struct InterleavedBuffer<'buffer, T: RawSample> {
    samples: &'buffer [T],
    frame_count: FrameCount,
    channel_count: ChannelCount,
}

impl<'buffer, T: RawSample> InterleavedBuffer<'buffer, T> {
    pub fn new(
        samples: &'buffer [T],
        frame_count: FrameCount,
        channel_count: ChannelCount,
    ) -> Self {
        assert_eq!(
            samples.len(),
            frame_count as usize * usize::from(channel_count)
        );

        Self {
            samples,
            frame_count,
            channel_count,
        }
    }

    fn offset(&self, SampleAddress { channel, frame }: SampleAddress) -> SampleIndex {
        usize::from(self.channel_count) * frame as usize + usize::from(channel)
    }

    fn frame_range(&self, frame_index: FrameIndex) -> Range<SampleIndex> {
        let start = frame_index as usize * usize::from(self.channel_count);
        start..(start + usize::from(self.channel_count))
    }
}

impl<'buffer, T: RawSample> SampleBuffer for InterleavedBuffer<'buffer, T> {
    type Item = T::Primitive;
    type Frame = SampleSlice<'buffer, T>;
    type Frames = InterleavedFrames<'buffer, T>;
    type Channel = InterleavedChannel<'buffer, T>;
    type Channels = InterleavedChannels<'buffer, T>;
    type Samples = InterleavedSamples<'buffer, T>;
    type SamplesInterleaved = InterleavedSamplesInterleaved<'buffer, T>;
    type SamplesSeparated = InterleavedSamplesSeparated<'buffer, T>;

    fn frame_count(&self) -> FrameIndex {
        self.frame_count
    }

    fn frame(&self, index: FrameIndex) -> Self::Frame {
        SampleSlice::new(&self.samples[self.frame_range(index)])
    }

    /// Returns an iterator over all frames of this buffer.
    /// Since this is an interleaved buffer, this operation is very cheap.
    fn frames(&self) -> Self::Frames {
        InterleavedFrames {
            frames: self.samples.chunks_exact(usize::from(self.channel_count)),
        }
    }

    fn channel_count(&self) -> ChannelCount {
        self.channel_count
    }

    fn channel(&self, index: ChannelIndex) -> Self::Channel {
        InterleavedChannel {
            samples: self.samples,
            channel_count: self.channel_count,
            channel_index: index,
        }
    }

    /// Returns an iterator over all channels of this buffer.
    /// Since this is an interleaved buffer, this type of access is not optimal.
    fn channels(&self) -> Self::Channels {
        InterleavedChannels {
            samples: self.samples,
            channel_indices: 0..self.channel_count,
        }
    }

    fn samples(&self) -> Self::Samples {
        InterleavedSamples::new(self.samples, self.frame_count, self.channel_count)
    }

    fn samples_interleaved(&self) -> InterleavedSamplesInterleaved<'buffer, T> {
        InterleavedSamplesInterleaved {
            samples: self.samples.iter(),
        }
    }

    fn samples_separated(&self) -> InterleavedSamplesSeparated<'buffer, T> {
        InterleavedSamplesSeparated {
            samples: self.samples,
            channel_count: self.channel_count,
            channel_index: 0,
            sample_index: 0,
        }
    }
}

impl<'buffer, T: RawSample> Index<SampleAddress> for InterleavedBuffer<'buffer, T> {
    type Output = T;

    fn index(&self, sample_address: SampleAddress) -> &Self::Output {
        &self.samples[self.offset(sample_address)]
    }
}

/// Iterator over all frames of a buffer
pub struct InterleavedFrames<'buffer, T: RawSample> {
    frames: ChunksExact<'buffer, T>,
}

impl<'buffer, T: RawSample> Iterator for InterleavedFrames<'buffer, T> {
    type Item = SampleSlice<'buffer, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.frames.next().map(|frame| SampleSlice::new(frame))
    }
}

/// Iterator over all channels of a buffer
pub struct InterleavedChannels<'buffer, T: RawSample> {
    samples: &'buffer [T],
    channel_indices: Range<ChannelIndex>,
}

impl<'buffer, T: RawSample> Iterator for InterleavedChannels<'buffer, T> {
    type Item = InterleavedChannel<'buffer, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.channel_indices
            .next()
            .map(|channel_index| InterleavedChannel {
                samples: self.samples,
                channel_count: self.channel_indices.end,
                channel_index,
            })
    }
}

/// Provides access to all samples of a single channel
pub struct InterleavedChannel<'buffer, T: RawSample> {
    samples: &'buffer [T],
    channel_count: ChannelCount,
    channel_index: ChannelIndex,
}

impl<'buffer, T: RawSample> IntoIterator for InterleavedChannel<'buffer, T> {
    type Item = T::Primitive;
    type IntoIter = InterleavedChannelSamples<'buffer, T>;

    fn into_iter(self) -> Self::IntoIter {
        InterleavedChannelSamples {
            samples: self
                .samples
                .iter()
                .skip(usize::from(self.channel_index))
                .step_by(usize::from(self.channel_count)),
        }
    }
}

impl<'buffer, T: RawSample> Index<FrameIndex> for InterleavedChannel<'buffer, T> {
    type Output = T;

    fn index(&self, frame_index: FrameIndex) -> &Self::Output {
        &self.samples[usize::from(self.channel_index) * usize::from(self.channel_count)
            + frame_index as usize]
    }
}

/// Iterator over all samples of a single channel
pub struct InterleavedChannelSamples<'buffer, T: RawSample> {
    samples: StepBy<Skip<slice::Iter<'buffer, T>>>,
}

impl<'buffer, T: RawSample> Iterator for InterleavedChannelSamples<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.samples.next().copied().map(T::Primitive::from)
    }
}

/// Iterator over all samples in native order
pub struct InterleavedSamples<'buffer, T: RawSample> {
    addresses: Zip<Range<FrameIndex>, Cycle<Range<ChannelIndex>>>,
    samples: std::slice::Iter<'buffer, T>,
}

impl<'buffer, T: RawSample> InterleavedSamples<'buffer, T> {
    fn new(samples: &'buffer [T], frame_count: FrameIndex, channel_count: ChannelCount) -> Self {
        Self {
            addresses: (0..frame_count).zip((0..channel_count).cycle()),
            samples: samples.iter(),
        }
    }
}

impl<'buffer, T: RawSample> Iterator for InterleavedSamples<'buffer, T> {
    type Item = (SampleAddress, T::Primitive);

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some((frame, channel)), Some(&sample)) =
            (self.addresses.next(), self.samples.next())
        {
            Some((SampleAddress { channel, frame }, T::Primitive::from(sample)))
        } else {
            None
        }
    }
}

/// Iterator over all samples in interleaved order
pub struct InterleavedSamplesInterleaved<'buffer, T: RawSample> {
    samples: std::slice::Iter<'buffer, T>,
}

impl<'buffer, T: RawSample> Iterator for InterleavedSamplesInterleaved<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.samples.next().copied().map(T::Primitive::from)
    }
}

/// Iterator over all samples in separated order
pub struct InterleavedSamplesSeparated<'buffer, T: RawSample> {
    samples: &'buffer [T],
    channel_count: ChannelCount,
    channel_index: ChannelIndex,
    sample_index: SampleIndex,
}

impl<'buffer, T: RawSample> Iterator for InterleavedSamplesSeparated<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while self.channel_index < self.channel_count {
            if let Some(sample) = self
                .samples
                .get(self.sample_index)
                .copied()
                .map(T::Primitive::from)
            {
                self.sample_index += usize::from(self.channel_count);
                return Some(sample);
            }
            self.channel_index += 1;
            // restart with the first frame
            self.sample_index = usize::from(self.channel_index);
        }

        None
    }
}

/// Contains mutable samples in a single buffer grouped by frames.
pub struct InterleavedBufferMut<'buffer, T: RawSample> {
    samples: &'buffer mut [T],
    frame_count: FrameIndex,
    channel_count: ChannelCount,
}

impl<'buffer, T: RawSample> InterleavedBufferMut<'buffer, T> {
    pub fn new(
        samples: &'buffer mut [T],
        frame_count: FrameCount,
        channel_count: ChannelCount,
    ) -> Self {
        assert_eq!(
            samples.len(),
            frame_count as usize * usize::from(channel_count)
        );

        Self {
            samples,
            frame_count,
            channel_count,
        }
    }

    fn offset(&self, SampleAddress { channel, frame }: SampleAddress) -> SampleIndex {
        usize::from(self.channel_count) * frame as usize + usize::from(channel)
    }

    fn frame_range(&self, frame_index: FrameIndex) -> Range<SampleIndex> {
        let start = frame_index as usize * usize::from(self.channel_count);
        start..(start + usize::from(self.channel_count))
    }
}

impl<'buffer, T: RawSample> SampleBufferMut for InterleavedBufferMut<'buffer, T> {
    type Item = T::Primitive;

    fn frame_count(&self) -> FrameIndex {
        self.frame_count
    }

    fn write_frame<Frame, Sample>(&mut self, index: FrameIndex, frame: Frame)
    where
        Frame: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let range = self.frame_range(index);
        self.samples[range]
            .iter_mut()
            .zip(frame.into_iter().map(T::Primitive::from).map(T::from))
            .for_each(|(sample_out, sample_in)| *sample_out = sample_in);
    }

    fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
    where
        Frames: IntoIterator<Item = Frame>,
        Frame: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        for (frame_out, frame_in) in self
            .samples
            .chunks_exact_mut(usize::from(self.channel_count))
            .zip(frames)
        {
            let frame_samples = frame_in.into_iter().map(T::Primitive::from).map(T::from);
            frame_out
                .iter_mut()
                .zip(frame_samples)
                .for_each(|(sample_out, sample_in)| *sample_out = sample_in);
        }
    }

    fn channel_count(&self) -> ChannelCount {
        self.channel_count
    }

    fn write_channel<Channel, Sample>(&mut self, index: ChannelIndex, channel: Channel)
    where
        Channel: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let channel_samples = channel.into_iter().map(T::Primitive::from).map(T::from);
        self.samples
            .iter_mut()
            .skip(usize::from(index))
            .step_by(usize::from(self.channel_count))
            .zip(channel_samples)
            .for_each(|(sample_out, sample_in)| *sample_out = sample_in);
    }

    fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
    where
        Channels: IntoIterator<Item = Channel>,
        Channel: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        channels
            .into_iter()
            .enumerate()
            .for_each(|(channel_index, channel)| {
                self.write_channel(channel_index as ChannelIndex, channel)
            });
    }

    fn write_sample<Sample>(&mut self, address: SampleAddress, sample: Sample)
    where
        T::Primitive: From<Sample>,
    {
        self.samples[self.offset(address)] = T::from(T::Primitive::from(sample));
    }

    fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let samples = samples.into_iter().map(T::Primitive::from).map(T::from);
        self.samples
            .iter_mut()
            .zip(samples)
            .for_each(|(sample_out, sample_in)| *sample_out = sample_in);
    }

    fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let channels = samples.into_iter().chunks(self.frame_count as usize);
        self.write_channels(channels.into_iter());
    }
}
