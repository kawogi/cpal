use std::ops::{Index, Range};

use itertools::Itertools;

use crate::types::RawSample;

use super::{ChannelIndex, FrameIndex, SampleAddress, SampleBuffer, SampleBufferMut, SampleSlice};

/// Contains samples where every channel has a separate internal buffer. (non-interleaved)
pub struct SeparatedBuffer<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    frame_count: FrameIndex,
}

impl<'buffer, T: RawSample> SeparatedBuffer<'buffer, T> {
    pub fn new(channels: &'buffer [&'buffer [T]], frame_count: FrameIndex) -> Self {
        assert!(channels.iter().all(|channel| channel.len() == frame_count));

        Self {
            channels,
            frame_count,
        }
    }
}

impl<'buffer, T: RawSample> SampleBuffer<T::Primitive> for SeparatedBuffer<'buffer, T> {
    type Frame = SeparatedFrame<'buffer, T>;
    type Frames = SeparatedFrames<'buffer, T>;
    type Channel = SampleSlice<'buffer, T>;
    type Channels = SeparatedChannels<'buffer, T>;
    type Samples = SeparatedSamples<'buffer, T>;
    type SamplesInterleaved = SeparatedSamplesInterleaved<'buffer, T>;
    type SamplesSeparated = SeparatedSamplesSeparated<'buffer, T>;

    fn frame_count(&self) -> FrameIndex {
        self.frame_count
    }

    fn frame(&self, index: super::FrameIndex) -> Self::Frame {
        SeparatedFrame {
            channels: self.channels,
            frame_index: index,
        }
    }

    /// Returns an iterator over all frames of this buffer.
    /// Since this is an non-interleaved buffer, this type of access is not optimal.
    fn frames(&self) -> Self::Frames {
        SeparatedFrames {
            channels: self.channels,
            indices: 0..self.frame_count,
        }
    }

    fn channel_count(&self) -> ChannelIndex {
        self.channels.len()
    }

    fn channel(&self, index: super::ChannelIndex) -> Self::Channel {
        SampleSlice::new(self.channels[index])
    }

    /// Returns an iterator over all channels of this buffer.
    /// Since this is an non-interleaved buffer, this operation is very cheap.
    fn channels(&self) -> Self::Channels {
        SeparatedChannels {
            channels: self.channels.iter(),
        }
    }

    fn samples(&self) -> Self::Samples {
        SeparatedSamples {
            channels: self.channels,
            address: SampleAddress::default(),
        }
    }

    fn samples_interleaved(&self) -> Self::SamplesInterleaved {
        SeparatedSamplesInterleaved {
            channels: self.channels,
            frame_count: self.frame_count,
            frame_index: 0,
            channel_index: 0,
        }
    }

    fn samples_separated(&self) -> Self::SamplesSeparated {
        SeparatedSamplesSeparated {
            channels: self.channels,
            frame_index: 0,
        }
    }
}

impl<'buffer, T: RawSample> Index<SampleAddress> for SeparatedBuffer<'buffer, T> {
    type Output = T;

    fn index(&self, SampleAddress { channel, frame }: SampleAddress) -> &Self::Output {
        &self.channels[channel][frame]
    }
}

/// Iterator over all frames of a buffer
pub struct SeparatedFrames<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    indices: Range<FrameIndex>,
}

impl<'frame, 'buffer: 'frame, T: RawSample> Iterator for SeparatedFrames<'buffer, T> {
    type Item = SeparatedFrame<'buffer, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| SeparatedFrame {
            channels: self.channels,
            frame_index: index,
        })
    }
}

/// Iterator over all channels of a buffer
pub struct SeparatedChannels<'buffer, T: RawSample> {
    channels: std::slice::Iter<'buffer, &'buffer [T]>,
}

impl<'buffer, T: RawSample> Iterator for SeparatedChannels<'buffer, T> {
    type Item = SampleSlice<'buffer, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.channels
            .next()
            .map(|&samples| SampleSlice::new(samples))
    }
}

/// Provides access to all samples of a single frame
pub struct SeparatedFrame<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    frame_index: FrameIndex,
}

impl<'buffer, T: RawSample> IntoIterator for SeparatedFrame<'buffer, T> {
    type Item = T::Primitive;
    type IntoIter = SeparatedFrameSamples<'buffer, T>;

    fn into_iter(self) -> Self::IntoIter {
        SeparatedFrameSamples {
            channels: self.channels.iter(),
            index: self.frame_index,
        }
    }
}

impl<'buffer, T: RawSample> Index<ChannelIndex> for SeparatedFrame<'buffer, T> {
    type Output = T;

    fn index(&self, channel_index: ChannelIndex) -> &Self::Output {
        &self.channels[channel_index][self.frame_index]
    }
}

/// Iterator over all samples of a single frame
pub struct SeparatedFrameSamples<'buffer, T: RawSample> {
    channels: std::slice::Iter<'buffer, &'buffer [T]>,
    index: ChannelIndex,
}

impl<'frame, 'buffer: 'frame, T: RawSample> Iterator for SeparatedFrameSamples<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.channels
            .next()
            .map(|&samples| T::Primitive::from(samples[self.index]))
    }
}

/// Iterator over all samples in native order
pub struct SeparatedSamples<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    address: SampleAddress,
}

impl<'buffer, T: RawSample> Iterator for SeparatedSamples<'buffer, T> {
    type Item = (SampleAddress, T::Primitive);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((&head, tail)) = self.channels.split_first() {
            if let Some(sample) = head
                .get(self.address.frame)
                .copied()
                .map(T::Primitive::from)
            {
                let result = (self.address, sample);
                self.address.frame += 1;
                return Some(result);
            }

            // next channel
            self.channels = tail;
            self.address.channel += 1;
            // restart with the first frame
            self.address.frame = 0;
        }

        None
    }
}

/// Iterator over all samples in interleaved order
pub struct SeparatedSamplesInterleaved<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    frame_count: FrameIndex,
    frame_index: FrameIndex,
    channel_index: ChannelIndex,
}

impl<'buffer, T: RawSample> Iterator for SeparatedSamplesInterleaved<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        (self.frame_index < self.frame_count).then(|| {
            let sample = T::Primitive::from(self.channels[self.channel_index][self.frame_index]);

            self.channel_index += 1;
            if self.channel_index == self.channels.len() {
                self.channel_index = 0;
                self.frame_index += 1;
            }

            sample
        })
    }
}

/// Iterator over all samples in separated order
pub struct SeparatedSamplesSeparated<'buffer, T: RawSample> {
    channels: &'buffer [&'buffer [T]],
    frame_index: FrameIndex,
}

impl<'buffer, T: RawSample> Iterator for SeparatedSamplesSeparated<'buffer, T> {
    type Item = T::Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((&head, tail)) = self.channels.split_first() {
            if let Some(sample) = head.get(self.frame_index).copied().map(T::Primitive::from) {
                self.frame_index += 1;
                return Some(sample);
            }

            self.channels = tail;
            // restart with the first frame
            self.frame_index = 0;
        }

        None
    }
}

/// Contains samples where every channel has a separate internal buffer. (non-interleaved)
pub struct SeparatedBufferMut<'buffer, T: RawSample> {
    channels: &'buffer mut [&'buffer mut [T]],
    frame_count: FrameIndex,
}

impl<'buffer, T: RawSample> SeparatedBufferMut<'buffer, T> {
    pub fn new(channels: &'buffer mut [&'buffer mut [T]], frame_count: FrameIndex) -> Self {
        assert!(channels.iter().all(|channel| channel.len() == frame_count));

        Self {
            channels,
            frame_count,
        }
    }
}

impl<'buffer, T: RawSample> SampleBufferMut<'buffer, T::Primitive>
    for SeparatedBufferMut<'buffer, T>
{
    fn frame_count(&self) -> FrameIndex {
        self.frame_count
    }

    fn write_frame<Frame, Sample>(&mut self, index: FrameIndex, frame: Frame)
    where
        Frame: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        self.channels
            .iter_mut()
            .map(|channel| &mut channel[index])
            .zip(frame)
            .for_each(|(sample_out, sample_in)| {
                *sample_out = T::from(T::Primitive::from(sample_in));
            });
    }

    fn write_frames<Frames, Frame, Sample>(&mut self, frames: Frames)
    where
        Frames: IntoIterator<Item = Frame>,
        Frame: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        frames
            .into_iter()
            .enumerate()
            .for_each(|(frame_index, frame_in)| self.write_frame(frame_index, frame_in));
    }

    fn channel_count(&self) -> ChannelIndex {
        self.channels.len()
    }

    fn write_channel<Channel, Sample>(&mut self, index: ChannelIndex, channel: Channel)
    where
        Channel: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let channel_samples = channel.into_iter().map(T::Primitive::from).map(T::from);
        self.channels[index]
            .iter_mut()
            .zip(channel_samples)
            .for_each(|(sample_out, sample_in)| *sample_out = sample_in);
    }

    fn write_channels<Channels, Channel, Sample>(&mut self, channels: Channels)
    where
        Channels: IntoIterator<Item = Channel>,
        Channel: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        self.channels
            .iter_mut()
            .zip(channels)
            .for_each(|(channel_out, channel_in)| {
                let channel_samples_in =
                    channel_in.into_iter().map(T::Primitive::from).map(T::from);
                channel_out.iter_mut().zip(channel_samples_in).for_each(
                    |(sample_out, sample_in)| {
                        *sample_out = sample_in;
                    },
                );
            });
    }

    fn write_sample<Sample>(
        &mut self,
        SampleAddress { channel, frame }: SampleAddress,
        sample: Sample,
    ) where
        T::Primitive: From<Sample>,
    {
        self.channels[channel][frame] = T::from(T::Primitive::from(sample));
    }

    fn write_samples_interleaved<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let frames = samples.into_iter().chunks(self.channel_count());
        self.write_frames(frames.into_iter());
    }

    fn write_samples_separated<Samples, Sample>(&mut self, samples: Samples)
    where
        Samples: IntoIterator<Item = Sample>,
        T::Primitive: From<Sample>,
    {
        let channels = samples.into_iter().chunks(self.frame_count);
        self.write_channels(channels.into_iter());
    }
}
