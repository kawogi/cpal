pub mod f32;
pub mod f64;
pub mod i16;
pub mod i24;
pub mod i32;
pub mod i64;
pub mod i8;
pub mod u16;
pub mod u24;
pub mod u32;
pub mod u64;
pub mod u8;

use std::fmt::Display;

pub use dasp_sample::{FromSample, I24, I48, U24, U48};

use crate::buffers::BufferFactory;

/// Describes all supported sample types and their in-memory representation.
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

/// Represents a public facing primitive that can be used as a sample.
// TODO `Display` should be required as well, but `dasp_sample::Sample` doesn't implement that trait
pub trait Sample: std::fmt::Debug + dasp_sample::Sample + Send + BufferFactory + 'static {
    /// Specifies the available in-memory representations of this primitive value.
    type Encoding: Encoding;

    /// Returns whether the given format targets this primitive value.
    fn supports_format(format: SampleFormat) -> bool;
}

/// A single sample in its raw memory representation (`[u8; N]`).
pub trait RawSample: std::fmt::Debug + Copy + From<Self::Primitive> + 'static
where
    Self::Primitive: From<Self>,
{
    /// The _public facing_ type to use when converting from/to the raw byte representation. (e.g. `i16`, `I24`, `f32`)
    type Primitive: Sample;
}

/// Provides some meta-data about the in-memory representation of a sample.
pub trait Encoding: Copy {
    /// number of bytes the sample format occupies in a slice.
    #[must_use]
    fn sample_size(self) -> usize;

    /// Returns whether the sample format is little endian.
    #[must_use]
    fn is_le(self) -> bool;

    /// Returns whether the sample format is big endian.
    #[must_use]
    fn is_be(self) -> bool;

    /// Returns whether the sample format is in native endianness
    #[inline]
    #[must_use]
    fn is_ne(self) -> bool {
        #[cfg(target_endian = "little")]
        return self.is_le();
        #[cfg(target_endian = "big")]
        return self.is_be();
    }
}

/// Simple raw format descriptor where only the variants _little endian_ and _big_endian_ exist and both types share
/// the same size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimpleLittleBigEncoding<const SIZE: usize> {
    LE,
    BE,
}

impl<const SIZE: usize> Encoding for SimpleLittleBigEncoding<SIZE> {
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        SIZE
    }

    #[inline]
    #[must_use]
    fn is_le(self) -> bool {
        matches!(self, Self::LE)
    }

    #[inline]
    #[must_use]
    fn is_be(self) -> bool {
        matches!(self, Self::BE)
    }
}

impl<const SIZE: usize> std::fmt::Display for SimpleLittleBigEncoding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LE => "le",
            Self::BE => "be",
        }
        .fmt(f)
    }
}

/// Simple raw format descriptor where only only one variant (native endianess) exists.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimpleEncoding<const SIZE: usize> {
    NE,
}

impl<const SIZE: usize> Encoding for SimpleEncoding<SIZE> {
    #[inline]
    #[must_use]
    fn sample_size(self) -> usize {
        SIZE
    }

    #[inline]
    #[must_use]
    fn is_le(self) -> bool {
        matches!(self, Self::NE)
    }

    #[inline]
    #[must_use]
    fn is_be(self) -> bool {
        matches!(self, Self::NE)
    }
}

impl<const SIZE: usize> std::fmt::Display for SimpleEncoding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NE => "ne",
        }
        .fmt(f)
    }
}
