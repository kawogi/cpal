use std::fmt::Display;

pub use dasp_sample::{FromSample, I24, I48, U24, U48};

use crate::{
    buffers::{SampleBuffer, SampleBufferMut},
    types::Encoding,
    ChannelCount, FrameCount,
};

/// Format that each sample type has in memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SampleFormat {
    /// `i8` with a valid range of 'u8::MIN..=u8::MAX' with `0` being the origin
    I8(<i8 as Sample>::Encoding),

    /// `i16` with a valid range of 'u16::MIN..=u16::MAX' with `0` being the origin
    I16(<i16 as Sample>::Encoding),

    // /// `I24` with a valid range of '-(1 << 23)..(1 << 23)' with `0` being the origin
    I24(<I24 as Sample>::Encoding),

    /// `i32` with a valid range of 'u32::MIN..=u32::MAX' with `0` being the origin
    I32(<i32 as Sample>::Encoding),

    // /// `I24` with a valid range of '-(1 << 47)..(1 << 47)' with `0` being the origin
    // I48(<I48 as Sample>::Encoding),
    /// `i64` with a valid range of 'u64::MIN..=u64::MAX' with `0` being the origin
    I64(<i64 as Sample>::Encoding),

    /// `u8` with a valid range of 'u8::MIN..=u8::MAX' with `1 << 7 == 128` being the origin
    U8(<u8 as Sample>::Encoding),

    /// `u16` with a valid range of 'u16::MIN..=u16::MAX' with `1 << 15 == 32768` being the origin
    U16(<u16 as Sample>::Encoding),

    // /// `U24` with a valid range of '0..16777216' with `1 << 23 == 8388608` being the origin
    U24(<U24 as Sample>::Encoding),

    /// `u32` with a valid range of 'u32::MIN..=u32::MAX' with `1 << 31` being the origin
    U32(<u32 as Sample>::Encoding),

    // /// `U48` with a valid range of '0..(1 << 48)' with `1 << 47` being the origin
    // U48(<U48 as Sample>::Encoding),
    /// `u64` with a valid range of 'u64::MIN..=u64::MAX' with `1 << 63` being the origin
    U64(<u64 as Sample>::Encoding),

    /// `f32` with a valid range of `-1.0..1.0` with `0.0` being the origin
    F32(<f32 as Sample>::Encoding),

    /// `f64` with a valid range of -1.0..1.0 with 0.0 being the origin
    F64(<f64 as Sample>::Encoding),
}

impl Encoding for SampleFormat {
    /// Returns the size in bytes of a sample of this format.
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        match self {
            Self::I8(format) => format.sample_size(),
            Self::I16(format) => format.sample_size(),
            Self::I24(format) => format.sample_size(),
            Self::I32(format) => format.sample_size(),
            Self::I64(format) => format.sample_size(),
            Self::U8(format) => format.sample_size(),
            Self::U16(format) => format.sample_size(),
            Self::U24(format) => format.sample_size(),
            Self::U32(format) => format.sample_size(),
            Self::U64(format) => format.sample_size(),
            Self::F32(format) => format.sample_size(),
            Self::F64(format) => format.sample_size(),
        }
    }

    #[inline]
    #[must_use]
    fn is_le(self) -> bool {
        match self {
            Self::I8(format) => format.is_le(),
            Self::I16(format) => format.is_le(),
            Self::I24(format) => format.is_le(),
            Self::I32(format) => format.is_le(),
            Self::I64(format) => format.is_le(),
            Self::U8(format) => format.is_le(),
            Self::U16(format) => format.is_le(),
            Self::U24(format) => format.is_le(),
            Self::U32(format) => format.is_le(),
            Self::U64(format) => format.is_le(),
            Self::F32(format) => format.is_le(),
            Self::F64(format) => format.is_le(),
        }
    }

    #[inline]
    #[must_use]
    fn is_be(self) -> bool {
        match self {
            Self::I8(format) => format.is_be(),
            Self::I16(format) => format.is_be(),
            Self::I24(format) => format.is_be(),
            Self::I32(format) => format.is_be(),
            Self::I64(format) => format.is_be(),
            Self::U8(format) => format.is_be(),
            Self::U16(format) => format.is_be(),
            Self::U24(format) => format.is_be(),
            Self::U32(format) => format.is_be(),
            Self::U64(format) => format.is_be(),
            Self::F32(format) => format.is_be(),
            Self::F64(format) => format.is_be(),
        }
    }

    #[inline]
    #[must_use]
    fn is_ne(self) -> bool {
        match self {
            Self::I8(format) => format.is_ne(),
            Self::I16(format) => format.is_ne(),
            Self::I24(format) => format.is_ne(),
            Self::I32(format) => format.is_ne(),
            Self::I64(format) => format.is_ne(),
            Self::U8(format) => format.is_ne(),
            Self::U16(format) => format.is_ne(),
            Self::U24(format) => format.is_ne(),
            Self::U32(format) => format.is_ne(),
            Self::U64(format) => format.is_ne(),
            Self::F32(format) => format.is_ne(),
            Self::F64(format) => format.is_ne(),
        }
    }
}

impl Display for SampleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::I8(format) => write!(f, "i8:{}", format),
            Self::I16(format) => write!(f, "i16:{}", format),
            Self::I24(format) => write!(f, "i24:{}", format),
            Self::I32(format) => write!(f, "i32:{}", format),
            Self::I64(format) => write!(f, "i64:{}", format),
            Self::U8(format) => write!(f, "u8:{}", format),
            Self::U16(format) => write!(f, "u16:{}", format),
            Self::U24(format) => write!(f, "u24:{}", format),
            Self::U32(format) => write!(f, "u32:{}", format),
            Self::U64(format) => write!(f, "u64:{}", format),
            Self::F32(format) => write!(f, "f32:{}", format),
            Self::F64(format) => write!(f, "f64:{}", format),
        }
    }
}

// TODO `Display` should be required as well, but `dasp_sample` doesn't implement that trait
pub trait Sample: std::fmt::Debug + dasp_sample::Sample + Send + BufferFactory + 'static {
    type Encoding: Encoding;

    fn supports_format(format: SampleFormat) -> bool;
}

pub trait BufferFactory {
    type Buffer<'buffer>: SampleBuffer<Item = Self>;
    type BufferMut<'buffer>: SampleBufferMut<Item = Self>;

    fn create_interleaved_buffer(
        bytes: &[u8],
        format: SampleFormat,
        channel_count: ChannelCount,
        frame_count: FrameCount,
    ) -> Option<Self::Buffer<'_>>;

    fn create_interleaved_buffer_mut(
        bytes: &mut [u8],
        format: SampleFormat,
        channel_count: ChannelCount,
        frame_count: FrameCount,
    ) -> Option<Self::BufferMut<'_>>;
}
