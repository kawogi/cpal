use std::fmt::Display;

pub use dasp_sample::{FromSample, Sample, I24, I48, U24, U48};

use crate::{
    buffers::{SampleBuffer, SampleBufferMut},
    types::{self, RawFormat},
    ChannelCount, FrameCount,
};

/// Format that each sample type has in memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RawSampleFormat {
    /// `i8` with a valid range of 'u8::MIN..=u8::MAX' with `0` being the origin
    I8(types::i8::RawFormat),

    /// `i16` with a valid range of 'u16::MIN..=u16::MAX' with `0` being the origin
    I16(types::i16::RawFormat),

    // /// `I24` with a valid range of '-(1 << 23)..(1 << 23)' with `0` being the origin
    I24(types::i24::RawFormat),

    /// `i32` with a valid range of 'u32::MIN..=u32::MAX' with `0` being the origin
    I32(types::i32::RawFormat),

    // /// `I24` with a valid range of '-(1 << 47)..(1 << 47)' with `0` being the origin
    // I48,
    /// `i64` with a valid range of 'u64::MIN..=u64::MAX' with `0` being the origin
    I64(types::i64::RawFormat),

    /// `u8` with a valid range of 'u8::MIN..=u8::MAX' with `1 << 7 == 128` being the origin
    U8(types::u8::RawFormat),

    /// `u16` with a valid range of 'u16::MIN..=u16::MAX' with `1 << 15 == 32768` being the origin
    U16(types::u16::RawFormat),

    // /// `U24` with a valid range of '0..16777216' with `1 << 23 == 8388608` being the origin
    U24(types::u24::RawFormat),

    /// `u32` with a valid range of 'u32::MIN..=u32::MAX' with `1 << 31` being the origin
    U32(types::u32::RawFormat),

    // /// `U48` with a valid range of '0..(1 << 48)' with `1 << 47` being the origin
    // U48(types::u48::RawFormat),
    /// `u64` with a valid range of 'u64::MIN..=u64::MAX' with `1 << 63` being the origin
    U64(types::u64::RawFormat),

    /// `f32` with a valid range of `-1.0..1.0` with `0.0` being the origin
    F32(types::f32::RawFormat),

    /// `f64` with a valid range of -1.0..1.0 with 0.0 being the origin
    F64(types::f64::RawFormat),
}

impl RawFormat for RawSampleFormat {
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

impl Display for RawSampleFormat {
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

// /// Format that each public facing sample has.
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// #[non_exhaustive]
// pub enum SampleFormat {
//     /// `i8` with a valid range of 'u8::MIN..=u8::MAX' with `0` being the origin
//     I8,

//     /// `i16` with a valid range of 'u16::MIN..=u16::MAX' with `0` being the origin
//     I16,

//     // /// `I24` with a valid range of '-(1 << 23)..(1 << 23)' with `0` being the origin
//     I24,

//     /// `i32` with a valid range of 'u32::MIN..=u32::MAX' with `0` being the origin
//     I32,

//     // /// `I24` with a valid range of '-(1 << 47)..(1 << 47)' with `0` being the origin
//     // I48,
//     /// `i64` with a valid range of 'u64::MIN..=u64::MAX' with `0` being the origin
//     I64,

//     /// `u8` with a valid range of 'u8::MIN..=u8::MAX' with `1 << 7 == 128` being the origin
//     U8,

//     /// `u16` with a valid range of 'u16::MIN..=u16::MAX' with `1 << 15 == 32768` being the origin
//     U16,

//     // /// `U24` with a valid range of '0..16777216' with `1 << 23 == 8388608` being the origin
//     U24,

//     /// `u32` with a valid range of 'u32::MIN..=u32::MAX' with `1 << 31` being the origin
//     U32,

//     // /// `U48` with a valid range of '0..(1 << 48)' with `1 << 47` being the origin
//     // U48,
//     /// `u64` with a valid range of 'u64::MIN..=u64::MAX' with `1 << 63` being the origin
//     U64,

//     /// `f32` with a valid range of `-1.0..1.0` with `0.0` being the origin
//     F32,

//     /// `f64` with a valid range of -1.0..1.0 with 0.0 being the origin
//     F64,
// }

// impl From<RawSampleFormat> for SampleFormat {
//     fn from(raw: RawSampleFormat) -> Self {
//         match raw {
//             RawSampleFormat::I8(_) => Self::I8,
//             RawSampleFormat::I16(_) => Self::I16,
//             RawSampleFormat::I24(_) => Self::I24,
//             RawSampleFormat::I32(_) => Self::I32,
//             RawSampleFormat::I64(_) => Self::I64,
//             RawSampleFormat::U8(_) => Self::U8,
//             RawSampleFormat::U16(_) => Self::U16,
//             RawSampleFormat::U24(_) => Self::U24,
//             RawSampleFormat::U32(_) => Self::U32,
//             RawSampleFormat::U64(_) => Self::U64,
//             RawSampleFormat::F32(_) => Self::F32,
//             RawSampleFormat::F64(_) => Self::F64,
//         }
//     }
// }

// impl SampleFormat {
//     /// Returns the size in bytes of a sample of this format.
//     #[inline]
//     #[must_use]
//     #[deprecated]
//     pub fn sample_size(&self) -> usize {
//         match *self {
//             SampleFormat::I8 | SampleFormat::U8 => mem::size_of::<i8>(),
//             SampleFormat::I16 | SampleFormat::U16 => mem::size_of::<i16>(),
//             SampleFormat::I24 | SampleFormat::U24 => 3,
//             SampleFormat::I32 | SampleFormat::U32 => mem::size_of::<i32>(),
//             SampleFormat::I64 | SampleFormat::U64 => mem::size_of::<i64>(),
//             SampleFormat::F32 => mem::size_of::<f32>(),
//             SampleFormat::F64 => mem::size_of::<f64>(),
//         }
//     }

//     #[inline]
//     #[must_use]
//     pub fn is_int(&self) -> bool {
//         matches!(
//             *self,
//             SampleFormat::I8
//                 | SampleFormat::I16
//                 | SampleFormat::I24
//                 | SampleFormat::I32
//                 | SampleFormat::I64
//         )
//     }

//     #[inline]
//     #[must_use]
//     pub fn is_uint(&self) -> bool {
//         matches!(
//             *self,
//             SampleFormat::U8
//                 | SampleFormat::U16
//                 | SampleFormat::U24
//                 | SampleFormat::U32
//                 | SampleFormat::U64
//         )
//     }

//     #[inline]
//     #[must_use]
//     pub fn is_float(&self) -> bool {
//         matches!(*self, SampleFormat::F32 | SampleFormat::F64)
//     }
// }

// impl Display for SampleFormat {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             SampleFormat::I8 => "i8",
//             SampleFormat::I16 => "i16",
//             SampleFormat::I24 => "i24",
//             SampleFormat::I32 => "i32",
//             // SampleFormat::I48 => "i48",
//             SampleFormat::I64 => "i64",
//             SampleFormat::U8 => "u8",
//             SampleFormat::U16 => "u16",
//             SampleFormat::U24 => "u24",
//             SampleFormat::U32 => "u32",
//             // SampleFormat::U48 => "u48",
//             SampleFormat::U64 => "u64",
//             SampleFormat::F32 => "f32",
//             SampleFormat::F64 => "f64",
//         }
//         .fmt(f)
//     }
// }

// TODO review name. Should be "Sample with format descriptor"
// TODO split into two traits. `BufferFactory` would make sense
// TODO `Display` should be required as well, but `dasp_sample` doesn't implement that trait
pub trait SizedSample: std::fmt::Debug + Sample + Send + 'static {
    //const FORMAT: SampleFormat;

    type RawFormat;
    type Buffer<'buffer>: SampleBuffer<Item = Self>;
    type BufferMut<'buffer>: SampleBufferMut<Item = Self>;

    fn supports_format(format: RawSampleFormat) -> bool;

    fn create_interleaved_buffer<'buffer>(
        bytes: &'buffer [u8],
        format: RawSampleFormat,
        channel_count: ChannelCount,
        frame_count: FrameCount,
    ) -> Option<Self::Buffer<'buffer>>;

    fn create_interleaved_buffer_mut<'buffer>(
        bytes: &'buffer mut [u8],
        format: RawSampleFormat,
        channel_count: ChannelCount,
        frame_count: FrameCount,
    ) -> Option<Self::BufferMut<'buffer>>;
}

// impl SizedSample for i8 {
//     const FORMAT: SampleFormat = SampleFormat::I8;

//     type Buffer<'buffer> = types::i8::SampleBuffer<'buffer>;
//     type BufferMut<'buffer> = types::i8::SampleBufferMut<'buffer>;

//     fn create_interleaved_buffer<'buffer>(
//         bytes: &'buffer [u8],
//         format: RawSampleFormat,
//         channel_count: ChannelCount,
//         frame_count: FrameCount,
//     ) -> Option<Self::Buffer<'buffer>> {
//         match format {
//             RawSampleFormat::I8 => {
//                 let samples = unsafe { transmute_from_bytes::<types::i8::NE>(bytes) };
//                 let buffer = InterleavedBuffer::new(samples, frame_count, channel_count);
//                 let buffer =
//                     types::i8::SampleBuffer::Interleaved(types::i8::InterleavedBuffer::NE(buffer));
//                 Some(buffer)
//             }
//             _ => None,
//         }
//     }

//     fn create_interleaved_buffer_mut<'buffer>(
//         bytes: &'buffer mut [u8],
//         format: RawSampleFormat,
//         channel_count: ChannelCount,
//         frame_count: FrameCount,
//     ) -> Option<Self::BufferMut<'buffer>> {
//         match format {
//             RawSampleFormat::I8 => {
//                 let samples = unsafe { transmute_from_bytes_mut::<types::i8::NE>(bytes) };
//                 let buffer = InterleavedBufferMut::new(samples, frame_count, channel_count);
//                 let buffer = types::i8::SampleBufferMut::Interleaved(
//                     types::i8::InterleavedBufferMut::NE(buffer),
//                 );
//                 Some(buffer)
//             }
//             _ => None,
//         }
//     }
// }

// impl SizedSample for i16 {
//     const FORMAT: SampleFormat = SampleFormat::I16;
// }

// impl SizedSample for I24 {
//     const FORMAT: SampleFormat = SampleFormat::I24;
// }

// impl SizedSample for i32 {
//     const FORMAT: SampleFormat = SampleFormat::I32;
// }

// // impl SizedSample for I48 { const FORMAT: SampleFormat = SampleFormat::I48; }

// impl SizedSample for i64 {
//     const FORMAT: SampleFormat = SampleFormat::I64;
// }

// impl SizedSample for u8 {
//     const FORMAT: SampleFormat = SampleFormat::U8;
// }

// impl SizedSample for u16 {
//     const FORMAT: SampleFormat = SampleFormat::U16;
// }

// impl SizedSample for U24 {
//     const FORMAT: SampleFormat = SampleFormat::U24;
// }

// impl SizedSample for u32 {
//     const FORMAT: SampleFormat = SampleFormat::U32;
// }

// // impl SizedSample for U48 { const FORMAT: SampleFormat = SampleFormat::U48; }

// impl SizedSample for u64 {
//     const FORMAT: SampleFormat = SampleFormat::U64;
// }

// impl SizedSample for f32 {
//     const FORMAT: SampleFormat = SampleFormat::F32;
// }

// impl SizedSample for f64 {
//     const FORMAT: SampleFormat = SampleFormat::F64;
// }
